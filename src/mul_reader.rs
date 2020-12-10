//! Methods for reading from standardized Mul and Idx files
//!
//! IDX files are defined as `|index:u32|size:u32|opt1:u16|opt2:u16|`
//!
//! Where index and size represent references into the equivalent Mul file
//!
//! Index values of `0xFEFEFEFF` are considered undefined, and should be skipped

use std::fs::{File, OpenOptions};
use std::io::{Result, SeekFrom, Error, ErrorKind, Seek, Read, Write};
#[cfg(test)]
use std::io::Cursor;
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

static UNDEF_RECORD: u32 = 0xFEFEFEFF;
static INDEX_SIZE: u32 = 12;

/**
 * An individual record, read from a Mul file
 */
pub struct MulRecord {
    ///Raw Mul data
    pub data: Vec<u8>,
    ///The index position in the Mul of this item
    pub start: u32,
    ///The total length in the Mul of this item
    pub length: u32,
    ///An implementation-specific variable
    pub opt1: u16,
    ///An implementation-specific variable
    pub opt2: u16
}

/**
 * Read Mul records out of am idx and a mul
 */
pub struct MulReader<T: Read + Seek> {
    idx_reader: T,
    data_reader: T
}

impl MulReader<File> {

    pub fn new(idx_path: &Path, mul_path: &Path) -> Result<MulReader<File>> {
        let idx_reader = File::open(idx_path)?;
        let data_reader = File::open(mul_path)?;

        Ok(MulReader {
            idx_reader: idx_reader,
            data_reader: data_reader
        })
    }
}

impl<T: Read + Seek> MulReader<T> {

    pub fn from_readables(idx_reader: T, data_reader: T) -> MulReader<T> {
        MulReader {
            idx_reader: idx_reader,
            data_reader: data_reader
        }
    }

    pub fn read(&mut self, index: u32) -> Result<MulRecord> {
        //Wind the idx reader to the index position
        self.idx_reader.seek(SeekFrom::Start((index * INDEX_SIZE) as u64))?;
        let start = self.idx_reader.read_u32::<LittleEndian>()?;
        if start == UNDEF_RECORD || start == u32::max_value() {
            //Check for empty cell
            Err(Error::new(ErrorKind::Other, format!("Trying to read out of bounds record {}, with a start of {}", index, start)))
        } else {
            let length = self.idx_reader.read_u32::<LittleEndian>()?;
            let mut data = vec![0; length as usize];
            let opt1 = self.idx_reader.read_u16::<LittleEndian>()?;
            let opt2 = self.idx_reader.read_u16::<LittleEndian>()?;
            self.data_reader.seek(SeekFrom::Start(start as u64))?;

            self.data_reader.read_exact(data.as_mut_slice())?;

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

/**
 * Write new records onto existing Mul and Idx files
 */
pub struct MulWriter<T: Write + Seek> {
    idx_writer: T,
    data_writer: T
}

pub enum MulWriterMode {
    Append,
    Truncate
}

impl MulWriter<File> {

    pub fn new(idx_path: &Path, mul_path: &Path, mode: MulWriterMode) -> Result<MulWriter<File>> {
        let mut options = OpenOptions::new();
        let options = options.write(true).create(true).truncate(match mode { MulWriterMode::Append => false, MulWriterMode::Truncate => true});

        let idx_writer = options.open(idx_path)?;
        let data_writer = options.open(mul_path)?;

        Ok(MulWriter {
            idx_writer: idx_writer,
            data_writer: data_writer
        })
    }
}

impl<T: Write + Seek> MulWriter<T> {

    pub fn append(&mut self, data: &Vec<u8>, opt1: Option<u16>, opt2: Option<u16>) -> Result<()> {

        //Wind the files to the end
        self.idx_writer.seek(SeekFrom::End(0))?;
        let mul_size = self.data_writer.seek(SeekFrom::End(0))?;

        //Fill up our fields
        let start = mul_size as u32;
        let length = data.len() as u32;
        let opt1 = match opt1 { Some(value) => value, None => 0} as u16;
        let opt2 = match opt2 { Some(value) => value, None => 0} as u16;

        self.data_writer.write(data.as_slice())?;
        self.idx_writer.write_u32::<LittleEndian>(start)?;
        self.idx_writer.write_u32::<LittleEndian>(length)?;
        self.idx_writer.write_u16::<LittleEndian>(opt1)?;
        self.idx_writer.write_u16::<LittleEndian>(opt2)?;

        Ok(())
    }
}

#[cfg(test)]
pub fn simple_from_vecs(vectors: Vec<Vec<u8>>, opt1: u16, opt2: u16) -> MulReader<Cursor<Vec<u8>>> {
    let mut idx_reader = Cursor::new(vec![]);
    let mut mul_reader = Cursor::new(vec![]);
    //For every MUL record, we should have an index record pointing at it
    for vec in vectors {
        let len = vec.len();
        let mul_size = mul_reader.seek(SeekFrom::End(0)).unwrap();
        let mut idx_cursor = Cursor::new(vec![]);
        idx_cursor.write_u32::<LittleEndian>(mul_size as u32).unwrap();  //Position
        idx_cursor.write_u32::<LittleEndian>(len as u32).unwrap();  //Length
        idx_cursor.write_u16::<LittleEndian>(opt1).unwrap();  //Opt1
        idx_cursor.write_u16::<LittleEndian>(opt2).unwrap();  //Opt2
        idx_reader.write(idx_cursor.get_ref()).unwrap();
        mul_reader.write(&vec).unwrap();
    }
    MulReader::from_readables(idx_reader, mul_reader)
}

#[cfg(test)]
pub fn simple_from_mul_records(records: Vec<MulRecord>) -> MulReader<Cursor<Vec<u8>>> {
    let mut idx_reader = Cursor::new(vec![]);
    let mut mul_reader = Cursor::new(vec![]);
    //For every MUL record, we should have an index record pointing at it
    for record in records {
        let mul_size = mul_reader.seek(SeekFrom::End(0)).unwrap();
        let mut idx_cursor = Cursor::new(vec![]);
        idx_cursor.write_u32::<LittleEndian>(mul_size as u32).unwrap();  //Position
        idx_cursor.write_u32::<LittleEndian>(record.data.len() as u32).unwrap();  //Length
        idx_cursor.write_u16::<LittleEndian>(record.opt1).unwrap();  //Opt1
        idx_cursor.write_u16::<LittleEndian>(record.opt2).unwrap();  //Opt2
        idx_reader.write(idx_cursor.get_ref()).unwrap();
        mul_reader.write(&record.data).unwrap();
    }
    MulReader::from_readables(idx_reader, mul_reader)
}
