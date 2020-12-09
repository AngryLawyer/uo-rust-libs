use std::io::{Result, SeekFrom, Seek, Read};
use std::fs::{File};
use std::path::Path;
use std::str::{from_utf8};
use byteorder::{LittleEndian, ReadBytesExt};

pub enum Flags {
    BackgroundFlag =    0x00000001,
    WeaponFlag =        0x00000002,
    TransparentFlag =   0x00000004,
    TranslucentFlag =   0x00000008,
    WallFlag =          0x00000010,
    DamagingFlag =      0x00000020,
    ImpassableFlag =    0x00000040,
    WetFlag =           0x00000080,
    UnknownFlag =       0x00000100,
    SurfaceFlag =       0x00000200,
    BridgeFlag =        0x00000400,
    StackableFlag =     0x00000800,
    WindowFlag =        0x00001000,
    NoShootFlag =       0x00002000,
    PrefixAFlag =       0x00004000,
    PrefixAnFlag =      0x00008000,
    InternalFlag =      0x00010000,
    FoliageFlag =       0x00020000,
    PartialHueFlag =    0x00040000,
    Unknown1Flag =      0x00080000,
    MapFlag =           0x00100000,
    ContainerFlag =     0x00200000,
    WearableFlag =      0x00400000,
    LightSourceFlag =   0x00800000,
    AnimatedFlag =      0x01000000,
    NoDiagonalFlag =    0x02000000,
    Unknown2Flag =      0x04000000,
    ArmorFlag =         0x08000000,
    RoofFlag =          0x10000000,
    DoorFlag =          0x20000000,
    StairBackFlag =     0x40000000,
    StairRightFlag =    0x80000000
}

// Tile data is odd, as we have [(unknown, (LAND_TILE_DATA) *32) * 512]
static GROUP_HEADER_SIZE:u32 = 4;
static MAP_TILE_SIZE:u32 = 26;
static STATIC_TILE_SIZE:u32 = 37;
static STATIC_OFFSET:u32 = 428032;

#[derive(Clone)]
pub struct MapTileData {
    pub flags: u32,
    pub texture_id: u16,
    pub name: String
}

#[derive(Clone)]
pub struct StaticTileData {
    pub flags: u32,
    pub weight: u8,
    pub quality_layer_light_id: u8,
    pub quantity_weapon_class_armor_class: u8,
    pub anim_id: u16,
    pub hue: u8,
    pub height_capacity: u8,
    pub name: String
}

pub struct TileDataReader {
    data_reader: File  //FIXME: This should be a Read + Seek instead of File
}

impl TileDataReader {
    pub fn new(mul_path: &Path) -> Result<TileDataReader> {
        let data_reader = File::open(mul_path)?;

        Ok(TileDataReader {
            data_reader: data_reader
        })
    }

    pub fn read_map_tile_data(&mut self, idx: u32) -> Result<MapTileData> {
        let offset = self.calculate_map_tile_offset(idx);
        self.data_reader.seek(SeekFrom::Start(offset))?;
        let flags = self.data_reader.read_u32::<LittleEndian>()?;
        let texture_id = self.data_reader.read_u16::<LittleEndian>()?;
        let reader = &self.data_reader;

        let raw_name = reader.bytes().take_while(|ref c| match *c {
            &Ok(n)  => n != 0,
            &Err(_) => true,
        }).collect::<Result<Vec<u8>>>()?;

        Ok(MapTileData {
            flags: flags,
            texture_id: texture_id,
            name: String::from(from_utf8(&raw_name).unwrap())
        })
    }

    fn calculate_map_tile_offset(&self, idx: u32) -> u64 {
        //For every 32, we have to add an unknown header
        let group_header_jumps = ((idx / 32) + 1) * GROUP_HEADER_SIZE;
        ((idx * MAP_TILE_SIZE) + group_header_jumps) as u64
    }

    pub fn read_static_tile_data(&mut self, idx: u32) -> Result<StaticTileData> {
        let offset = self.calculate_static_tile_offset(idx);
        self.data_reader.seek(SeekFrom::Start(offset))?;

        let flags = self.data_reader.read_u32::<LittleEndian>()?;
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

        let reader = &self.data_reader;
        let raw_name = reader.bytes().take_while(|ref c| match *c {
            &Ok(n)  => n != 0,
            &Err(_) => true,
        }).collect::<Result<Vec<u8>>>()?;

        Ok(StaticTileData {
            flags: flags,
            weight: weight,
            quality_layer_light_id: quality,
            quantity_weapon_class_armor_class: quantity,
            anim_id: anim_id,
            hue: hue,
            height_capacity: height,
            name: String::from(from_utf8(&raw_name).unwrap())
        })
    }

    fn calculate_static_tile_offset(&self, idx: u32) -> u64 {
        //For every 32, we have to add an unknown header
        let group_header_jumps = ((idx / 32) + 1) * GROUP_HEADER_SIZE;
        ((idx * STATIC_TILE_SIZE) + group_header_jumps + STATIC_OFFSET) as u64
    }
}
