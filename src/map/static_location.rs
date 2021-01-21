use byteorder::{LittleEndian, ReadBytesExt};
use mul_reader::MulReader;
use std::fs::File;
use std::io::{Cursor, Error, ErrorKind, Read, Result, Seek};
use std::path::Path;

#[derive(Clone, Copy)]
pub struct StaticLocation {
    pub object_id: u16,
    pub x: u8,
    pub y: u8,
    pub altitude: i8,
    pub checksum: u16, //Not actually used
}

impl StaticLocation {
    pub fn color_idx(&self) -> u16 {
        self.object_id + 16384
    }
}

pub struct StaticReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
    width: u32,
    height: u32,
}

impl StaticReader<File> {
    pub fn new(
        index_path: &Path,
        mul_path: &Path,
        width_blocks: u32,
        height_blocks: u32,
    ) -> Result<StaticReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;

        Ok(StaticReader {
            mul_reader: mul_reader,
            width: width_blocks,
            height: height_blocks,
        })
    }
}

impl<T: Read + Seek> StaticReader<T> {
    pub fn read_block(&mut self, id: u32) -> Result<Vec<StaticLocation>> {
        let raw = self.mul_reader.read(id)?;
        let len = raw.data.len();
        assert!(len % 7 == 0);
        let mut reader = Cursor::new(raw.data);
        let mut statics = vec![];
        for _i in 0..(len / 7) {
            let object_id = reader.read_u16::<LittleEndian>()?;
            let x = reader.read_u8()?;
            let y = reader.read_u8()?;
            let altitude = reader.read_i8()?;
            let checksum = reader.read_u16::<LittleEndian>()?;
            statics.push(StaticLocation {
                object_id: object_id,
                x: x,
                y: y,
                altitude: altitude,
                checksum: checksum,
            });
        }
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
                format!("{} {} is outside of valid map coordinates", x, y),
            ))
        }
    }
}
