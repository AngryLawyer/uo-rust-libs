use super::diff::StaticDiffReader;
use super::shared::{read_block_statics, StaticLocation};
use mul_reader::MulReader;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek};
use std::path::Path;

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
    pub fn read_block(
        &mut self,
        id: u32,
        patch: Option<&mut StaticDiffReader<T>>,
    ) -> Result<Vec<StaticLocation>> {
        match patch {
            Some(reader) => reader
                .read(id)
                .unwrap_or_else(|| read_block_statics(&mut self.mul_reader, id)),
            None => read_block_statics(&mut self.mul_reader, id),
        }
    }

    pub fn read_block_from_coordinates(
        &mut self,
        x: u32,
        y: u32,
        patch: Option<&mut StaticDiffReader<T>>,
    ) -> Result<Vec<StaticLocation>> {
        let width = self.width;
        let height = self.height;
        if x < width && y < height {
            self.read_block(y + (x * height), patch)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("{} {} is outside of valid map coordinates", x, y),
            ))
        }
    }
}
