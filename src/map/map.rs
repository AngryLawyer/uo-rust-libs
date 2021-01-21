use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::Path;

pub const BLOCK_SIZE: usize = 196;
pub const OFFSET: u32 = 4;
pub const MAP0_SIZE: u32 = 393216;

pub mod map_size {
    pub const FELUCCA: (u32, u32) = (7168, 4096);
    pub const TRAMMEL: (u32, u32) = (7168, 4096);
    pub const ILSHENAR: (u32, u32) = (2304, 1600);
    pub const MALAS: (u32, u32) = (2560, 2048);
    pub const TOKUNO: (u32, u32) = (1448, 1448);
    pub const TER_MUR: (u32, u32) = (1280, 4096);
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub graphic: u16,
    pub altitude: i8,
}

#[derive(Copy)]
pub struct Block {
    pub checksum: u32, //Not actually used
    pub cells: [Cell; 64],
}

impl Clone for Block {
    fn clone(&self) -> Self {
        let mut cells = [Cell {
            graphic: 0,
            altitude: 0,
        }; 64];
        for i in 0..64 {
            cells[i] = self.cells[i].clone();
        }
        Block {
            checksum: self.checksum,
            cells: cells,
        }
    }
}

pub struct MapReader {
    data_reader: File,
    width: u32,
    height: u32,
}

impl MapReader {
    pub fn new(map_path: &Path, width: u32, height: u32) -> Result<MapReader> {
        let data_reader = File::open(map_path)?;

        Ok(MapReader {
            data_reader: data_reader,
            width: width,
            height: height,
        })
    }

    /**
     * Read a specific block from a map
     */
    pub fn read_block(&mut self, id: u32) -> Result<Block> {
        //Cycle to id * 192
        self.data_reader
            .seek(SeekFrom::Start((id * BLOCK_SIZE as u32) as u64))?;
        //Read the header
        let mut block = Block {
            checksum: self.data_reader.read_u32::<LittleEndian>()?,
            cells: [Cell {
                graphic: 0,
                altitude: 0,
            }; 64],
        };
        //Read 64 cells
        for i in 0..64 {
            block.cells[i] = Cell {
                graphic: self.data_reader.read_u16::<LittleEndian>()?,
                altitude: self.data_reader.read_i8()?,
            };
        }
        Ok(block)
    }

    pub fn read_block_from_coordinates(&mut self, x: u32, y: u32) -> Result<Block> {
        let width = self.width;
        let height = self.height;
        if x < width && y < height {
            self.read_block(y + (x * height))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("{} {} is outside of valid map coordinates", x, y),
            ))
        }
    }
}
