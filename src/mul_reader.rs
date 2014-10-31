use std::num::Bounded;
use std::io::{File, FileMode, Open, Read, Write, IoResult, SeekSet, OtherIoError, IoError};

static UNDEF_RECORD: u32 = 0xFEFEFEFF;
static INDEX_SIZE: u32 = 12;

pub struct MulRecord {
    pub data: Vec<u8>,
    pub start: u32,
    pub length: u32,
    pub opt1: u16,
    pub opt2: u16
}

pub struct MulReader {
    idx_reader: File,
    data_reader: File
}

impl MulReader {

    pub fn new(idx_path: &Path, mul_path: &Path) -> IoResult<MulReader> {
        let idx_reader = try!(File::open_mode(idx_path, Open, Read));
        let data_reader = try!(File::open_mode(mul_path, Open, Read));

        Ok(MulReader {
            idx_reader: idx_reader,
            data_reader: data_reader
        })
    }

    pub fn read(&mut self, index: u32) -> IoResult<MulRecord> {
        //Wind the idx reader to the index position
        try!(self.idx_reader.seek((index * INDEX_SIZE) as i64, SeekSet));
        let start = try!(self.idx_reader.read_le_u32());
        if start == UNDEF_RECORD || start == Bounded::max_value() { 
            //Check for empty cell
            Err(IoError {
                kind: OtherIoError,
                desc: "Trying to read out-of-bounds record",
                detail: Some(format!("Trying to read out of bounds record {}, with a start of {}", index, start))
            })
        } else {
            let length = try!(self.idx_reader.read_le_u32());
            let opt1 = try!(self.idx_reader.read_le_u16());
            let opt2 = try!(self.idx_reader.read_le_u16());
            try!(self.data_reader.seek(start as i64, SeekSet));

            let data = try!(self.data_reader.read_exact(length as uint));

            Ok(MulRecord {
                data: data,
                start: start,
                length: length,
                opt1: opt1,
                opt2: opt2
            })
        }
    }
}

pub struct MulWriter {
    idx_writer: File,
    data_writer: File
}

impl MulWriter {

    pub fn new(idx_path: &Path, mul_path: &Path, method: FileMode) -> IoResult<MulWriter> {
        let idx_writer = try!(File::open_mode(idx_path, method, Write));
        let data_writer = try!(File::open_mode(mul_path, method, Write));

        Ok(MulWriter {
            idx_writer: idx_writer,
            data_writer: data_writer
        })
    }

    pub fn append(&mut self, data: &Vec<u8>, opt1: Option<u16>, opt2: Option<u16>) -> IoResult<()> {

        let idx_size = try!(self.idx_writer.stat()).size as i64;
        let mul_size = try!(self.data_writer.stat()).size as i64;

        //Wind the files to the end
        try!(self.idx_writer.seek(idx_size, SeekSet));
        try!(self.data_writer.seek(mul_size, SeekSet));

        //Fill up our fields
        let start = mul_size as u32;
        let length = data.len() as u32;
        let opt1 = match opt1 { Some(value) => value, None => 0} as u16;
        let opt2 = match opt2 { Some(value) => value, None => 0} as u16;

        try!(self.data_writer.write(data.as_slice()));
        try!(self.idx_writer.write_le_u32(start));
        try!(self.idx_writer.write_le_u32(length));
        try!(self.idx_writer.write_le_u16(opt1));
        try!(self.idx_writer.write_le_u16(opt2));

        Ok(())
    }
}
