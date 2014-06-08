use std::num::Bounded;
use std::io::{File, FileMode, Open, Read, Write, IoResult, SeekSet, OtherIoError, IoError};

static UNDEF_RECORD:u32 = 0xFEFEFEFF;
static INDEX_SIZE: uint = 12;

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
        let idx = File::open_mode(idx_path, Open, Read);
        let mul = File::open_mode(mul_path, Open, Read);

        match (idx, mul) {
            (Ok(idx_reader), Ok(data_reader)) => Ok(MulReader {
                idx_reader: idx_reader,
                data_reader: data_reader
            }),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err)
        }
    }

    pub fn read(&mut self, index: uint) -> IoResult<MulRecord> {
        //Wind the idx reader to the index position
        let data = self.idx_reader.seek((index * INDEX_SIZE) as i64, SeekSet).and_then(|()| {
            self.idx_reader.read_le_uint_n(4).and_then(|start| {
                self.idx_reader.read_le_uint_n(4).and_then(|length| {
                    self.idx_reader.read_le_uint_n(2).and_then(|opt1| {
                        self.idx_reader.read_le_uint_n(2).and_then(|opt2| {
                            Ok((start as u32, length as u32, opt1 as u16, opt2 as u16))
                        })
                    })
                })
            })
        });

        match data {
            Ok((start, length, opt1, opt2)) => {
                //Check for empty cell
                if start == UNDEF_RECORD || start == Bounded::max_value() { 
                    Err(IoError {
                        kind: OtherIoError,
                        desc: "Trying to read out-of-bounds record",
                        detail: Some(format!("Trying to read out of bounds record {}, with a start of {}", index, start))
                    })
                } else {
                    let maybe_data = self.data_reader.seek(start as i64, SeekSet).and_then(|()| {
                        self.data_reader.read_exact(length as uint)
                    });
                    match maybe_data {
                        Ok(data) => {
                            Ok(MulRecord {
                                data: data,
                                start: start,
                                length: length,
                                opt1: opt1,
                                opt2: opt2
                            })
                        },
                        Err(err) => Err(err)
                    }
                }
            },
            Err(err) => Err(err)
        }
    }
}

pub struct MulWriter {
    idx_writer: File,
    data_writer: File
}

impl MulWriter{

    pub fn new(idx_path: &Path, mul_path: &Path, method: FileMode) -> IoResult<MulWriter> {
        let idx = File::open_mode(idx_path, method, Write);
        let mul = File::open_mode(mul_path, method, Write);

        match (idx, mul) {
            (Ok(idx_writer), Ok(data_writer)) => Ok(MulWriter {
                idx_writer: idx_writer,
                data_writer: data_writer
            }),
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err)
        }
    }

    pub fn append(&mut self, data: &Vec<u8>, opt1: Option<u16>, opt2: Option<u16>) -> IoResult<()> {

        let sizes = self.idx_writer.stat().and_then(|idx_stat| {
            self.data_writer.stat().and_then(|data_stat| {
                Ok((idx_stat.size as i64, data_stat.size as i64))
            })
        });

        if sizes.is_err() {
            return Err(sizes.unwrap_err());
        };

        let (idx_size, mul_size) = match sizes {
            Ok(result) => result,
            Err(_) => fail!("Should be impossible")
        };

        //Wind the files to the end
        let seek_result = self.idx_writer.seek(idx_size, SeekSet).and_then(|()| {
            self.data_writer.seek(mul_size, SeekSet)
        });

        match seek_result {
            Ok(_) => {
                //Fill up our fields
                let start = mul_size as u32;
                let length = data.len() as u32;
                let opt1 = match opt1 { Some(value) => value, None => 0} as u16;
                let opt2 = match opt2 { Some(value) => value, None => 0} as u16;
                self.data_writer.write(data.as_slice()).and_then(|()| {
                    self.idx_writer.write_le_u32(start).and_then(|()| {
                        self.idx_writer.write_le_u32(length).and_then(|()| {
                            self.idx_writer.write_le_u16(opt1).and_then(|()| {
                                self.idx_writer.write_le_u16(opt2)
                            })
                        })
                    })
                })
            },
            Err(err) => Err(err)
        }
    }
}
