use byteorder::{LittleEndian, ReadBytesExt};
use color::Color16;
use std::fs::File;
use std::io::{Result, Seek, SeekFrom};
use std::path::Path;

pub struct RadarColReader {
    data_reader: File,
}

impl RadarColReader {
    pub fn new(radar_col_path: &Path) -> Result<RadarColReader> {
        let data_reader = File::open(radar_col_path)?;

        Ok(RadarColReader {
            data_reader: data_reader,
        })
    }

    pub fn read_color(&mut self, id: u32) -> Result<Color16> {
        self.data_reader.seek(SeekFrom::Start((id * 2) as u64))?;
        let data = self.data_reader.read_u16::<LittleEndian>()?;
        Ok(data)
    }

    pub fn read_colors(&mut self) -> Result<Vec<Color16>> {
        self.data_reader.seek(SeekFrom::Start(0))?;
        let meta = self.data_reader.metadata()?;
        let mut output = vec![];
        for _i in 0..(meta.len() / 2) {
            output.push(self.data_reader.read_u16::<LittleEndian>()?);
        }
        Ok(output)
    }
}
