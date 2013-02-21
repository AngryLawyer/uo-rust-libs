use mul_reader;
use byte_helpers;

#[deriving_eq]
pub enum Flags {
    pub BackgroundFlag =    0x00000001, 
    pub WeaponFlag =        0x00000002, 
    pub TransparentFlag =   0x00000004, 
    pub TranslucentFlag =   0x00000008, 
    pub WallFlag =          0x00000010, 
    pub DamagingFlag =      0x00000020, 
    pub ImpassableFlag =    0x00000040, 
    pub WetFlag =           0x00000080, 
    pub UnknownFlag =       0x00000100, 
    pub SurfaceFlag =       0x00000200, 
    pub BridgeFlag =        0x00000400, 
    pub StackableFlag =     0x00000800, 
    pub WindowFlag =        0x00001000, 
    pub NoShootFlag =       0x00002000, 
    pub PrefixAFlag =       0x00004000, 
    pub PrefixAnFlag =      0x00008000, 
    pub InternalFlag =      0x00010000, 
    pub FoliageFlag =       0x00020000, 
    pub PartialHueFlag =    0x00040000, 
    pub Unknown1Flag =      0x00080000, 
    pub MapFlag =           0x00100000, 
    pub ContainerFlag =     0x00200000, 
    pub WearableFlag =      0x00400000, 
    pub LightSourceFlag =   0x00800000, 
    pub AnimatedFlag =      0x01000000, 
    pub NoDiagonalFlag =    0x02000000, 
    pub Unknown2Flag =      0x04000000, 
    pub ArmorFlag =         0x08000000, 
    pub RoofFlag =          0x10000000, 
    pub DoorFlag =          0x20000000, 
    pub StairBackFlag =     0x40000000,
    pub StairRightFlag =    0x80000000
}

pub struct MapTileData {
    flags: u32,
    texture_id: u16,
    name: ~str
}

pub struct StaticTileData {
    flags: u32,
    weight: u8,
    quality_layer_light_id: u8,
    quantity_weapon_class_armor_class: u8,
    anim_id: u16,
    hue: u8,
    height_capacity: u8,
    name: ~str
}

pub struct TileDataReader {
    data_reader: io::Reader
}

// Tile data is odd, as we have [(unknown, (LAND_TILE_DATA) *32) * 512]
const GROUP_HEADER_SIZE:uint = 4;
const MAP_TILE_SIZE:uint = 26;
const STATIC_TILE_SIZE:uint 37;
const STATIC_OFFSET:uint = 428032; 

impl TileDataReader {
    static fn new(tile_data_path: &path::Path) -> result::Result<TileDataReader, ~str> {
        match io::file_reader(tile_data_path) {
            result::Ok(data_reader) => {
                result::Ok(TileDataReader {
                    data_reader: data_reader
                })
            },
            result::Err(error_message) => {
                result::Err(error_message)
            }
        }
    }

    fn read_map_tile_data(&self, idx: uint) -> option::Option<MapTileData> {
        self.data_reader.seek(self.calculate_map_tile_offset(idx) as int, io::SeekSet);
        let tile_data_reader = self.data_reader as io::ReaderUtil;
        option::Some(MapTileData{
            flags: tile_data_reader.read_le_u32(),
            texture_id: tile_data_reader.read_le_u16(),
            name: str::from_bytes(tile_data_reader.read_bytes(20))
        })
    }

    fn calculate_map_tile_offset(&self, idx: uint) -> uint {
        //For every 32, we have to add an unknown header
        let group_header_jumps = ((idx / 32) + 1) * GROUP_HEADER_SIZE;
        (idx * MAP_TILE_SIZE) + group_header_jumps
    }

    fn read_static_tile_data(&self, idx: uint) -> option::Option<StaticTileData> {
        self.data_reader.seek(self.calculate_static_tile_offset(idx) as int, io::SeekSet);
        let tile_data_reader = self.data_reader as io::ReaderUtil;

        let flags = tile_data_reader.read_le_u32();
        let weight = tile_data_reader.read_byte();
        let quality  = tile_data_reader.read_byte();
        let _unknown = tile_data_reader.read_le_u16();
        let _unknown1 = tile_data_reader.read_byte();
        let quantity = tile_data_reader.read_byte();
        let anim_id = tile_data_reader.read_le_u16();
        let _unknown2 = tile_data_reader.read_byte();
        let hue = tile_data_reader.read_byte();
        let _unknown3 = tile_data_reader.read_le_u16();
        let height = tile_data_reader.read_byte();
        let name = str::from_bytes(tile_data_reader.read_bytes(20))

        option::Some(StaticTileData{
        })
    }

    fn calculate_static_tile_offset(&self, idx: uint) -> uint {
        //For every 32, we have to add an unknown header
        let group_header_jumps = ((idx / 32) + 1) * GROUP_HEADER_SIZE;
        (idx * STATIC_TILE_SIZE) + group_header_jumps
    }

}
