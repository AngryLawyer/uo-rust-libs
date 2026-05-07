//! Methods for reading static prop locations from muls
//!
//! Each index location maps directly to a block in the map files. Each mul record represents
//! all of the static objects in a block, as a list of StaticLocations
//!
//! A StaticLocation is defined as
//!
//! `|object_id:u16|x:y8|y:u8|altitude:i8|`
//!
//! As with MapReader, methods allow the passing of a patch reader to simplify applying patches.
use super::diff::StaticLocationDiffReader;
use super::shared::{StaticLocation, read_block_statics};
use crate::error::{MulReaderError, MulReaderResult};
use crate::mul::MulReader;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

/// A struct to help read out Static locations for a map
///
/// The methods on this struct optionally take a StaticLocationDiffReader,
/// to make it easier to apply patches to a map
pub struct StaticLocationReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
    /// Width, in blocks
    width: u32,
    /// Height, in blocks
    height: u32,
}

impl StaticLocationReader<File> {
    /// Create a new StaticLocationReader from an index and mul path
    pub fn new(
        index_path: &Path,
        mul_path: &Path,
        width_blocks: u32,
        height_blocks: u32,
    ) -> MulReaderResult<StaticLocationReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;

        Ok(StaticLocationReader {
            mul_reader,
            width: width_blocks,
            height: height_blocks,
        })
    }
}

impl<T: Read + Seek> StaticLocationReader<T> {
    /// Create an ArtReader from an existing mul reader
    pub fn from_mul(
        mul_reader: MulReader<T>,
        width_blocks: u32,
        height_blocks: u32,
    ) -> StaticLocationReader<T> {
        StaticLocationReader {
            mul_reader,
            width: width_blocks,
            height: height_blocks,
        }
    }

    /// Read all statics for a block from the map by its id
    /// Blocks are stored in columns, from top of the map to to bottom
    pub fn read_block(
        &mut self,
        id: u32,
        patch: Option<&mut StaticLocationDiffReader<T>>,
    ) -> MulReaderResult<Vec<StaticLocation>> {
        match patch {
            Some(reader) => reader
                .read(id)
                .unwrap_or_else(|| read_block_statics(&mut self.mul_reader, id)),
            None => read_block_statics(&mut self.mul_reader, id),
        }
    }

    /// Read all statics block from the map by its absolute coordinates
    pub fn read_block_from_coordinates(
        &mut self,
        x: u32,
        y: u32,
        patch: Option<&mut StaticLocationDiffReader<T>>,
    ) -> MulReaderResult<Vec<StaticLocation>> {
        let width = self.width;
        let height = self.height;
        if x < width && y < height {
            self.read_block(y + (x * height), patch)
        } else {
            Err(MulReaderError::CoordinatesOutOfBounds { x, y })
        }
    }
}
