//! Methods for reading animated characters out of anim.mul/anim.idx
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use color::{Color, Color16, Color32};
use mul_reader::MulReader;
use std::fs::{File};
use std::io::{Result, Error, ErrorKind, Cursor, SeekFrom, Seek, Read};
use std::path::Path;

pub struct Row {
    header: u16,
    offset: u16,
    image_data: Vec<u8>
}

pub struct AnimFrame {
    image_centre_x: u16,
    image_centre_y: u16,
    width: u16,
    height: u16,
    data: Vec<Row>
}

pub struct AnimGroup {
    pub palette: [Color16; 256],
    pub frame_count: u32,
    pub frames: Vec<AnimFrame>
}

pub struct AnimReader<T: Read + Seek> {
    mul_reader: MulReader<T>
}

fn read_frame<T: Read + Seek>(reader: &mut T) -> Result<AnimFrame> {
    let image_centre_x = try!(reader.read_u16::<LittleEndian>());
    let image_centre_y = try!(reader.read_u16::<LittleEndian>());
    let width = try!(reader.read_u16::<LittleEndian>());
    let height = try!(reader.read_u16::<LittleEndian>());
    // Read data
}

impl AnimReader<File> {

    pub fn new(index_path: &Path, mul_path: &Path) -> Result<AnimReader<File>> {
        let mul_reader = try!(MulReader::new(index_path, mul_path));
        Ok(AnimReader {
            mul_reader: mul_reader
        })
    }
}

impl <T: Read + Seek> AnimReader<T> {

    pub fn from_mul(reader: MulReader<T>) -> AnimReader<T> {
        AnimReader {
            mul_reader: reader
        }
    }

    pub fn read(&mut self, id: u32) -> Result<AnimGroup> {
        let raw = try!(self.mul_reader.read(id));
        let mut reader = Cursor::new(raw.data);
        // Read the palette
        let mut palette = [0; 256];
        for i in 0..256 {
            palette[i] = try!(reader.read_u16::<LittleEndian>());
        }
        let frame_count = try!(reader.read_u32::<LittleEndian>());
        Ok(AnimGroup {
            palette: palette,
            frame_count: frame_count,
            frames: vec![]
        })
    }
}
