//! Methods for reading radar colors from radarcol.mul
//!
//! This file is one of the simplest muls - a sequence of Color16s
//!
//! It's expected for the file to contain 65536 records.
//! Each color matches a given tile in Art, and (offset by 16384 bytes) each static in there, too
use crate::color::Color16;
use crate::error::MulReaderResult;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// A struct to help read colors out of RadarCol
pub struct RadarColReader<T: Read + Seek> {
    data_reader: T,
    length: u32,
}

impl RadarColReader<File> {
    /// Create a RadarColReader from a path
    pub fn new(radar_col_path: &Path) -> MulReaderResult<RadarColReader<File>> {
        let data_reader = File::open(radar_col_path)?;
        let meta = data_reader.metadata()?;
        let length = meta.len() as u32;

        Ok(RadarColReader {
            data_reader,
            length,
        })
    }
}

impl<T: Read + Seek + ExactSizeIterator> RadarColReader<T> {
    /// Create a RadarColReader from an existing readable
    pub fn from_readable(data_reader: T) -> RadarColReader<T> {
        let length = data_reader.len() as u32;
        RadarColReader {
            data_reader,
            length,
        }
    }
}

impl<T: Read + Seek> RadarColReader<T> {
    /// Read the color at a specific index
    pub fn read_color(&mut self, id: u32) -> MulReaderResult<Color16> {
        self.data_reader.seek(SeekFrom::Start((id * 2) as u64))?;
        let data = self.data_reader.read_u16::<LittleEndian>()?;
        Ok(data)
    }

    /// Read all colors contained in the file
    pub fn read_colors(&mut self) -> MulReaderResult<Vec<Color16>> {
        let mut output = vec![];
        self.data_reader.seek(SeekFrom::Start(0))?;
        for _i in 0..(self.length / 2) {
            output.push(self.data_reader.read_u16::<LittleEndian>()?);
        }
        Ok(output)
    }
}
