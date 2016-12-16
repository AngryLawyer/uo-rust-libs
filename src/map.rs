use std::io::{Cursor, Result, SeekFrom, Seek, Error, ErrorKind, Read};
use std::fs::{File};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use mul_reader::MulReader;
use color::Color16;

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
    pub checksum: u32,  //Not actually used
    pub cells: [Cell; 64]
}

impl Clone for Block {
    fn clone(&self) -> Self {
        let mut cells = [Cell {graphic: 0, altitude: 0}; 64];
        for i in 0..64 {
            cells[i] = self.cells[i].clone();
        }
        Block {
            checksum: self.checksum,
            cells: cells
        }
    }
}

#[derive(Clone, Copy)]
pub struct StaticLocation {
    pub object_id: u16,
    pub x: u8,
    pub y: u8,
    pub altitude: i8,
    pub checksum: u16  //Not actually used
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
        let mut block = Block {
            checksum: try!(self.data_reader.read_u32::<LittleEndian>()),
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
            self.read_block(y + (x * height))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("{} {} is outside of valid map coordinates", x, y)
            ))
        }
    }
}

pub struct StaticReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
    width: u32,
    height: u32
}

impl StaticReader<File> {
    pub fn new(index_path: &Path, mul_path: &Path, width: u32, height: u32) -> Result<StaticReader<File>> {
        let mul_reader = try!(MulReader::new(index_path, mul_path));

        Ok(StaticReader {
            mul_reader: mul_reader,
            width: width,
            height: height
        })
    }
}

impl<T: Read + Seek> StaticReader<T> {

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
            let checksum = try!(reader.read_u16::<LittleEndian>());
            statics.push(StaticLocation{
                object_id: object_id,
                x: x,
                y: y,
                altitude: altitude,
                checksum: checksum
            });
        };
        Ok(statics)
    }

    pub fn read_block_from_coordinates(&mut self, x: u32, y: u32) -> Result<Vec<StaticLocation>> {
        let width = self.width;
        let height = self.height;
        if x < width && y < height {
            self.read_block(y + (x * height))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("{} {} is outside of valid map coordinates", x, y)
            ))
        }
    }
}

pub struct RadarColReader {
    data_reader: File
}

impl RadarColReader {
    pub fn new(radar_col_path: &Path) -> Result<RadarColReader> {
        let data_reader = try!(File::open(radar_col_path));

        Ok(RadarColReader {
            data_reader: data_reader,
        })
    }

    pub fn read_color(&mut self, id: u32) -> Result<Color16> {
        try!(self.data_reader.seek(SeekFrom::Start((id * 2) as u64)));
        let data = try!(self.data_reader.read_u16::<LittleEndian>());
        Ok(data)
    }

    pub fn read_colors(&mut self) -> Result<Vec<Color16>> {
        try!(self.data_reader.seek(SeekFrom::Start(0)));
        let meta = try!(self.data_reader.metadata());
        let mut output = vec![];
        for _i in 0..(meta.len() / 2) {
            output.push(try!(self.data_reader.read_u16::<LittleEndian>()));
        }
        Ok(output)
    }
}
