use std::fs::File;
use std::path::Path;

mod diff;
mod radarcol;
mod shared;
mod static_location;

use crate::errors::MulReaderError;
use crate::errors::MulReaderResult;

pub use self::diff::*;
pub use self::radarcol::*;
pub use self::shared::*;
pub use self::static_location::*;

pub struct MapReader {
    data_reader: File,
    width: u32,
    height: u32,
}

impl MapReader {
    pub fn new(map_path: &Path, width: u32, height: u32) -> MulReaderResult<MapReader> {
        let data_reader = File::open(map_path)?;

        Ok(MapReader {
            data_reader,
            width,
            height,
        })
    }

    /**
     * Read a specific block from a map
     */
    pub fn read_block(
        &mut self,
        id: u32,
        patch: Option<&mut MapDiffReader>,
    ) -> MulReaderResult<Block> {
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
    ) -> MulReaderResult<Block> {
        let width = self.width;
        let height = self.height;
        if x < width && y < height {
            self.read_block(y + (x * height), patch)
        } else {
            Err(MulReaderError::CoordinatesOutOfBounds { x, y })
        }
    }
}
