use mul_reader::MulReader;
use color::Color16;
use std::io::{Result, Error, ErrorKind, Cursor, SeekFrom, Seek, Write};
use byteorder::{LittleEndian, ReadBytesExt};
use std::path::Path;

#[derive(Clone, Copy)]
pub struct GumpPair {
    color: Color16,
    count: u16
}

#[derive(Clone)]
pub struct Gump {
    width: u16,
    height: u16,
    data: Vec<GumpPair>
}

pub struct GumpReader {
    mul_reader: MulReader
}

impl GumpReader {

    pub fn new(index_path: &Path, mul_path: &Path) -> Result<GumpReader> {
        let mul_reader = try!(MulReader::new(index_path, mul_path));
        Ok(GumpReader {
            mul_reader: mul_reader
        })
    }

    pub fn read_gump(&mut self, index: u32) -> Result<Gump> {
        let raw = try!(self.mul_reader.read(index));
        let mut output = vec![];
        let len = raw.data.len();
        assert!(len % 4 == 0);
        let mut reader = Cursor::new(raw.data);
        for _i in 0..(len / 4) {
            let color = try!(reader.read_u16::<LittleEndian>());
            let count = try!(reader.read_u16::<LittleEndian>());
            output.push(GumpPair {
                color: color,
                count: count
            });
        };
        Ok(Gump {
            height: raw.opt1,
            width: raw.opt2,
            data: output
        })
    }
}
