//! Methods for reading map files from the various map muls
//!
//! Maps are stored as single, unindexed files. They contain a sequence of blocks:
//!
//! `|checksum:u32|cells:[Block..64]`
//!
//! And blocks are stored as
//!
//! `|graphic:u16|altitude:i8|`
//!
//! You need to know the dimensions of the map to read it correctly;
//! some of these are stored in `map_size`
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

pub mod diff;
pub mod radarcol;
#[cfg(not(test))]
mod shared;
#[cfg(test)]
pub mod shared;
pub mod static_location;

use crate::error::{MulReaderError, MulReaderResult};
use crate::map::diff::MapDiffReader;
use crate::map::shared::read_block;
pub use crate::map::shared::{Block, Cell, StaticLocation};

/// Constants for map sizes, in blocks
pub mod map_size {
    pub const SOSARIA: (u32, u32) = (896, 512);
    pub const ILSHENAR: (u32, u32) = (288, 200);
    pub const MALAS: (u32, u32) = (320, 256);
    pub const TOKUNO: (u32, u32) = (181, 181);
    pub const TER_MUR: (u32, u32) = (160, 512);
}

/// A struct to help read out Map data
///
/// The methods on this struct optionally take a MapDiffReader,
/// to make it easier to apply patches to a map
#[derive(Debug)]
pub struct MapReader<T: Read + Seek> {
    data_reader: T,
    /// Width, in blocks
    width: u32,
    /// Height, in blocks
    height: u32,
}

impl MapReader<File> {
    /// Create a new MapReader from a mul path
    pub fn new(
        map_path: &Path,
        width_blocks: u32,
        height_blocks: u32,
    ) -> MulReaderResult<MapReader<File>> {
        let data_reader = File::open(map_path)?;

        Ok(MapReader {
            data_reader,
            width: width_blocks,
            height: height_blocks,
        })
    }
}

impl<T: Read + Seek> MapReader<T> {
    /// Create a MapReader from an existing readable
    pub fn from_readable(data_reader: T, width_blocks: u32, height_blocks: u32) -> MapReader<T> {
        MapReader {
            data_reader,
            width: width_blocks,
            height: height_blocks,
        }
    }

    /// Read a block from the map by its id
    /// Blocks are stored in columns, from top of the map to to bottom
    pub fn read_block<U: Read + Seek>(
        &mut self,
        id: u32,
        patch: Option<&mut MapDiffReader<U>>,
    ) -> MulReaderResult<Block> {
        match patch {
            Some(reader) => reader
                .read(id)
                .unwrap_or_else(|| read_block(&mut self.data_reader, id)),
            None => read_block(&mut self.data_reader, id),
        }
    }

    /// Read a block from the map by its absolute coordinates
    pub fn read_block_from_coordinates<U: Read + Seek>(
        &mut self,
        x: u32,
        y: u32,
        patch: Option<&mut MapDiffReader<U>>,
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
