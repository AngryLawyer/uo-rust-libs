use super::diff::MapDiffReader;
use super::shared::{read_block, Block};
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub struct MapReader {
    data_reader: File,
    width: u32,
    height: u32,
}

impl MapReader {
    pub fn new(map_path: &Path, width: u32, height: u32) -> Result<MapReader> {
        let data_reader = File::open(map_path)?;

        Ok(MapReader {
            data_reader: data_reader,
            width: width,
            height: height,
        })
    }

    /**
     * Read a specific block from a map
     */
    pub fn read_block(&mut self, id: u32, patch: Option<&mut MapDiffReader>) -> Result<Block> {
        match patch {
            Some(reader) => reader
                .read(id)
                .unwrap_or_else(|| read_block(&mut self.data_reader, id)),
            None => read_block(&mut self.data_reader, id),
        }
    }

    pub fn read_block_from_coordinates(
        &mut self,
        x: u32,
        y: u32,
        patch: Option<&mut MapDiffReader>,
    ) -> Result<Block> {
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
