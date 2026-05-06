//! Methods for reading supplementary data about tiles and statics from tiledata.mul
//!
//! `tiledata` is split in two, much like `art`; first with those for map tiles, then statics.
//!
//! The file starts with 512 blocks of MapTileData, which are defined as
//!
//! `|unknown:u32|tiles:[MapTileData..32]|`
//!
//! MapTileData is defined as
//!
//! `|flags:u32|texture_id:u16|name:[u8..20(CString)]|`
//!
//! The rest of the file is Static tile data, also organised into blocks
//!
//! `|unknown:u32|tiles:[StaticTileData..32]|`
//!
//! StaticTileData is defined as
//!
//! `|flags:u32|weight:u8|quality:u8|unknown:u16|unknown:u8|quantity:u8|anim_id:u16|unknown:u8|hue:u8|unknown:u16|height:u8|name:[u8..20(CString)]|`
//!
//! Several fields are used to represent multiple attributes:
//!
//! * Quality also represents Layer for wearables, and Light ID for lights
//! * Quantity represents Weapon Class for weapons, and Armor value for Armor
//! * Height represents capacity for containers
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::str::from_utf8;

use crate::error::MulReaderResult;

bitflags! {
    /// Bitflags associated with a tile
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u32 {
        const BackgroundFlag = 0x00000001;
        const WeaponFlag = 0x00000002;
        const TransparentFlag = 0x00000004;
        const TranslucentFlag = 0x00000008;
        const WallFlag = 0x00000010;
        const DamagingFlag = 0x00000020;
        const ImpassableFlag = 0x00000040;
        const WetFlag = 0x00000080;
        const UnknownFlag = 0x00000100;
        const SurfaceFlag = 0x00000200;
        const BridgeFlag = 0x00000400;
        const StackableFlag = 0x00000800;
        const WindowFlag = 0x00001000;
        const NoShootFlag = 0x00002000;
        const PrefixAFlag = 0x00004000;
        const PrefixAnFlag = 0x00008000;
        const InternalFlag = 0x00010000;
        const FoliageFlag = 0x00020000;
        const PartialHueFlag = 0x00040000;
        const Unknown1Flag = 0x00080000;
        const MapFlag = 0x00100000;
        const ContainerFlag = 0x00200000;
        const WearableFlag = 0x00400000;
        const LightSourceFlag = 0x00800000;
        const AnimatedFlag = 0x01000000;
        const NoDiagonalFlag = 0x02000000;
        const Unknown2Flag = 0x04000000;
        const ArmorFlag = 0x08000000;
        const RoofFlag = 0x10000000;
        const DoorFlag = 0x20000000;
        const StairBackFlag = 0x40000000;
        const StairRightFlag = 0x80000000;
    }
}

// Tile data is odd, as we have [(unknown, (LAND_TILE_DATA) * 32) * 512]
// All values are in bytes
const GROUP_HEADER_SIZE: u32 = 4;
const MAP_TILE_SIZE: u32 = 26;
const STATIC_TILE_SIZE: u32 = 37;
const STATIC_OFFSET: u32 = 428032;

/// Information about a given Map tile
#[derive(Clone, Debug)]
pub struct MapTileData {
    pub flags: Flags,
    /// Which TexMap to use instead if this tile is non-flat
    pub texture_id: u16,
    pub name: String,
}

/// Information about a given Static tile
#[derive(Clone, Debug)]
pub struct StaticTileData {
    pub flags: Flags,
    pub weight: u8,
    /// This field becomes Layer for wearables, and the Light ID for lights, otherwise quality.
    pub quality_layer_light_id: u8,
    /// This field becomes weapon class for weapons, armor class for armor, or defaults to quantity.
    pub quantity_weapon_class_armor_class: u8,
    pub anim_id: u16,
    pub hue: u8,
    /// This field becomes capacity for containers, otherwise height
    pub height_capacity: u8,
    pub name: String,
}

/// A struct to help read out MapTileData and StaticTileData data
pub struct TileDataReader<T: Read + Seek> {
    data_reader: T,
}

impl TileDataReader<File> {
    /// Create a new TileDataReader from a mul path
    pub fn new(mul_path: &Path) -> MulReaderResult<TileDataReader<File>> {
        let data_reader = File::open(mul_path)?;

        Ok(TileDataReader { data_reader })
    }
}

impl<T: Read + Seek> TileDataReader<T> {
    /// Create a TileDataReader from an existing file reader
    pub fn from_readable(reader: T) -> TileDataReader<T> {
        TileDataReader {
            data_reader: reader,
        }
    }

    /// Read a map tile's associated data.
    ///
    /// The ID matches the data in ArtReader's `read_tile`
    pub fn read_map_tile_data(&mut self, idx: u32) -> MulReaderResult<MapTileData> {
        let offset = self.calculate_map_tile_offset(idx);
        self.data_reader.seek(SeekFrom::Start(offset))?;
        let flags = Flags::from_bits(self.data_reader.read_u32::<LittleEndian>()?)
            .unwrap_or(Flags::empty());
        let texture_id = self.data_reader.read_u16::<LittleEndian>()?;

        let mut raw_name = vec![];
        loop {
            match self.data_reader.read_u8()? {
                0 => break,
                x => raw_name.push(x),
            }
        }

        Ok(MapTileData {
            flags,
            texture_id,
            name: String::from(from_utf8(&raw_name).unwrap_or("ERROR")),
        })
    }

    fn calculate_map_tile_offset(&self, idx: u32) -> u64 {
        //For every 32, we have to add an unknown header
        let group_header_jumps = ((idx / 32) + 1) * GROUP_HEADER_SIZE;
        ((idx * MAP_TILE_SIZE) + group_header_jumps) as u64
    }

    /// Read a static tile's associated data.
    ///
    /// The ID is read from the static offset, and matches the data in ArtReader's `read_static`
    pub fn read_static_tile_data(&mut self, idx: u32) -> MulReaderResult<StaticTileData> {
        let offset = self.calculate_static_tile_offset(idx);
        self.data_reader.seek(SeekFrom::Start(offset))?;

        let flags = Flags::from_bits(self.data_reader.read_u32::<LittleEndian>()?)
            .unwrap_or(Flags::empty());
        let weight = self.data_reader.read_u8()?;
        let quality = self.data_reader.read_u8()?;
        let _unknown = self.data_reader.read_u16::<LittleEndian>()?;
        let _unknown1 = self.data_reader.read_u8()?;
        let quantity = self.data_reader.read_u8()?;
        let anim_id = self.data_reader.read_u16::<LittleEndian>()?;
        let _unknown2 = self.data_reader.read_u8()?;
        let hue = self.data_reader.read_u8()?;
        let _unknown3 = self.data_reader.read_u16::<LittleEndian>()?;
        let height = self.data_reader.read_u8()?;

        let mut raw_name = vec![];
        loop {
            match self.data_reader.read_u8()? {
                0 => break,
                x => raw_name.push(x),
            }
        }

        Ok(StaticTileData {
            flags,
            weight,
            quality_layer_light_id: quality,
            quantity_weapon_class_armor_class: quantity,
            anim_id,
            hue,
            height_capacity: height,
            name: String::from(from_utf8(&raw_name).unwrap_or("ERROR")),
        })
    }

    fn calculate_static_tile_offset(&self, idx: u32) -> u64 {
        //For every 32, we have to add an unknown header
        let group_header_jumps = ((idx / 32) + 1) * GROUP_HEADER_SIZE;
        ((idx * STATIC_TILE_SIZE) + group_header_jumps + STATIC_OFFSET) as u64
    }
}
