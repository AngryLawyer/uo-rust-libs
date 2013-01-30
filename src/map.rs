pub type Map = ~[Block];

pub type Block = ~[Cell];

pub struct Cell {
    graphic: u16,
    altitude: i8,
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
    fn read_block(&self, id: uint) -> Block {
        //Cycle to id * 196 + Offset
        self.data_reader.seek(((id * BLOCK_SIZE) + OFFSET) as int, io::SeekSet);
        let map_reader = self.data_reader as io::ReaderUtil;
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
    fn read_map(&self, max_blocks: uint) -> Map {
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
pub fn MapReader(path: ~str) -> result::Result<MapReader, ~str> {
    let mul_path: path::Path = path::Path(path);
    match io::file_reader(&mul_path) {
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
