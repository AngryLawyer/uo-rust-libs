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
    pub image_centre_x: u16,
    pub image_centre_y: u16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<Row>
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
    Ok(AnimFrame {
        image_centre_x: image_centre_x,
        image_centre_y: image_centre_y,
        width: width,
        height: height,
        data: vec![]
    })
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
        let mut frame_offsets = vec![];
        for _ in 0..frame_count {
            frame_offsets.push(try!(reader.read_u32::<LittleEndian>()));
        }

        let mut frames = vec![];
        for offset in frame_offsets {
            try!(reader.seek(SeekFrom::Start((256 * 2 + offset) as u64)));
            frames.push(try!(read_frame(&mut reader)));
        }

        Ok(AnimGroup {
            palette: palette,
            frame_count: frame_count,
            frames: frames
        })
    }
}
