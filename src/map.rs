use core::io;
use core::io::ReaderUtil;
use mul_reader;
use byte_helpers;

pub type Map = ~[Block];

pub type Block = ~[Cell];

pub struct Cell {
    graphic: u16,
    altitude: i8,
}

pub type Statics = ~[StaticLocation];

pub struct StaticLocation {
    object_id: u16,
    x: u8,
    y: u8,
    altitude: i8,
    remainder: u16
}

const BLOCK_SIZE: uint = 196;
const OFFSET: uint = 4;

pub struct MapReader {
    data_reader: io::Reader
}

impl MapReader {

    /**
     * Read a specific block from a map
     */
    pub fn read_block(&self, id: uint) -> Block {
        //Cycle to id * 196 + Offset
        self.data_reader.seek(((id * BLOCK_SIZE) + OFFSET) as int, io::SeekSet);
        let map_reader = self.data_reader;
        //Read the 64 cells
        let mut block: Block = ~[];
        //Read 64 cells
        for uint::range(0, 64) |_index| {
            block.push(Cell{
                graphic: map_reader.read_le_u16(),
                altitude: map_reader.read_i8()
            });
        }
        block
    }

    /**
     * Read the whole map!
     */
    pub fn read_map(&self, max_blocks: uint) -> Map {
        let mut map: Map = ~[];
        let mut index = 0;
        while index < max_blocks {
            map.push(self.read_block(index));
            index += 1;
        }
        map
    }
}

/**
 * Create a handle to a mapreader, and read out given blocks as needed
 */
pub fn MapReader(mul_path: &path::Path) -> result::Result<MapReader, ~str> {
    match io::file_reader(mul_path) {
        result::Ok(data_reader) => {
            result::Ok(MapReader {
                data_reader: data_reader
            })
        },
        result::Err(error_message) => {
            result::Err(error_message)
        }
    }
}

pub struct StaticReader {
    mul_reader: mul_reader::MulReader
}

impl StaticReader {
    pub fn read_block(&self, id: uint) -> option::Option<Statics> {
        match self.mul_reader.read(id) {
            option::Some(record) => {
                assert record.data.len() % 7 == 0;
                let mut statics:Statics = ~[];
                let mut data_source = byte_helpers::Buffer(copy record.data);
                for uint::range_step(0, record.data.len(), 7) |_i| {
                    let object_id: u16 = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;
                    let x: u8 = byte_helpers::bytes_to_le_uint(data_source.read_items(1)) as u8;
                    let y: u8 = byte_helpers::bytes_to_le_uint(data_source.read_items(1)) as u8;
                    let altitude: i8 = byte_helpers::bytes_to_le_uint(data_source.read_items(1)) as i8;
                    let remainder: u16 = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;
                    statics.push(StaticLocation{
                        object_id: object_id,
                        x: x,
                        y: y,
                        altitude: altitude,
                        remainder: remainder
                    });
                }
                option::Some(statics)
            }
            option::None => {
                option::None
            }
        }
    }
}

pub fn StaticReader(index_path: &path::Path, mul_path: &path::Path) -> result::Result<StaticReader, ~str> {
    match mul_reader::MulReader(index_path, mul_path) {
        result::Err(message) => result::Err(message),
        result::Ok(mul_reader) => {
            result::Ok(StaticReader{
                mul_reader: mul_reader
            })
        }
    }
}
