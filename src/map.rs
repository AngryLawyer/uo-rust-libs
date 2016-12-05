use std::io::{Cursor, Result, SeekFrom, Seek, Error, ErrorKind};
use std::fs::{File};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use mul_reader::MulReader;

pub const BLOCK_SIZE: usize = 196;
pub const OFFSET: u32 = 4;
pub const MAP0_SIZE: u32 = 393216;

#[derive(Clone, Copy)]
pub struct Cell {
    pub graphic: u16,
    pub altitude: i8,
}

pub struct Block {
    pub header: [u32; 4],
    pub cells: [Cell; 64]
}

pub struct StaticLocation {
    pub object_id: u16,
    pub x: u8,
    pub y: u8,
    pub altitude: i8,
    pub unknown: u16
}

impl StaticLocation {
    pub fn color_idx(&self) -> u16 {
        self.object_id + 16384
    }
}

pub struct MapReader {
    data_reader: File,
    width: u32,
    height: u32
}

impl MapReader {

    pub fn new(map_path: &Path, width: u32, height: u32) -> Result<MapReader> {
        let data_reader = try!(File::open(map_path));

        Ok(MapReader {
            data_reader: data_reader,
            width: width,
            height: height
        })
    }

    /**
     * Read a specific block from a map
     */
    pub fn read_block(&mut self, id: u32) -> Result<Block> {
        //Cycle to id * 192
        try!(self.data_reader.seek(SeekFrom::Start((id * BLOCK_SIZE as u32) as u64)));
        //Read the header
        //Read the 64 cells
        let mut block = Block {
            header: [
                try!(self.data_reader.read_u32::<LittleEndian>()),
                try!(self.data_reader.read_u32::<LittleEndian>()),
                try!(self.data_reader.read_u32::<LittleEndian>()),
                try!(self.data_reader.read_u32::<LittleEndian>())
            ],
            cells: [Cell {graphic: 0, altitude: 0}; 64]
        };
        //Read 64 cells
        for i in 0..64 {
            block.cells[i] = Cell{
                graphic: try!(self.data_reader.read_u16::<LittleEndian>()),
                altitude: try!(self.data_reader.read_i8())
            };
        }
        Ok(block)
    }

    pub fn read_block_from_coordinates(&mut self, x: u32, y: u32) -> Result<Block> {
        let width = self.width;
        let height = self.height;
        if x < width && y < height {
            self.read_block(x + (y * width))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("{} {} is Outside of valid map coordinates", x, y)
            ))
        }
    }
}

pub struct StaticReader {
    mul_reader: MulReader,
    width: u32,
    height: u32
}

impl StaticReader {

    pub fn new(index_path: &Path, mul_path: &Path, width: u32, height: u32) -> Result<StaticReader> {
        let mul_reader = try!(MulReader::new(index_path, mul_path));

        Ok(StaticReader {
            mul_reader: mul_reader,
            width: width,
            height: height
        })
    }

    pub fn read_block(&mut self, id: u32) -> Result<Vec<StaticLocation>> {
        let raw = try!(self.mul_reader.read(id));
        let len = raw.data.len();
        assert!(len % 7 == 0);
        let mut reader = Cursor::new(raw.data);
        let mut statics = vec![];
        for _i in 0..(len / 7) {
            let object_id = try!(reader.read_u16::<LittleEndian>());
            let x = try!(reader.read_u8());
            let y = try!(reader.read_u8());
            let altitude = try!(reader.read_i8());
            let unknown = try!(reader.read_u16::<LittleEndian>());
            statics.push(StaticLocation{
                object_id: object_id,
                x: x,
                y: y,
                altitude: altitude,
                unknown: unknown
            });
        };
        Ok(statics)
    }

    pub fn read_block_from_coordinates(&mut self, x: u32, y: u32) -> Result<Vec<StaticLocation>> {
        let width = self.width;
        let height = self.height;
        if x < width && y < height {
            self.read_block(x + (y * width))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("{} {} is Outside of valid map coordinates", x, y)
            ))
        }
    }
}
