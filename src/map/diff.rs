//! Methods for reading patches over a map from mapdifl (the lookup file) and mapdif (the data).
//! This module also provides methods for reading static patches from stadifl (lookup), stadifi (index) and stadif (data)
//!
//! For both types of lookup files, the format is simple
//!
//! `|lookups:[u32..file_size]|`
//!
//! Each lookup value represents a block index in the map file, while their index represents
//! the appropriate index in the patch data files. In the static files, the index instead
//! represents a lookup into the index file.
//!
//! mapdif files are internally structured as a list of blocks.
//! stadif and stadifl are structured the same way as the static locations files.
use super::shared::{Block, StaticLocation, read_block, read_block_statics};
use crate::error::MulReaderResult;
use crate::mul::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

fn generate_lookup_table<T: Read + Seek>(
    data: &mut T,
    length: u32,
) -> MulReaderResult<HashMap<u32, u32>> {
    let mut lookup_table = HashMap::new();

    for i in 0..(length / 4) {
        lookup_table.insert(data.read_u32::<LittleEndian>()?, i);
    }

    Ok(lookup_table)
}

/// A struct to help read out map blocks from a diff file
#[derive(Debug)]
pub struct MapDiffReader<T: Read + Seek> {
    lookup_table: HashMap<u32, u32>,
    diff: T,
}

impl MapDiffReader<File> {
    /// Create a new MapDiffReader from an lookup path and mul path
    pub fn new(lookup_path: &Path, diff_path: &Path) -> MulReaderResult<MapDiffReader<File>> {
        // Start by reading all of the lookup info
        let mut lookup = File::open(lookup_path)?;
        let diff = File::open(diff_path)?;

        let meta = lookup.metadata()?;
        let lookup_table = generate_lookup_table(&mut lookup, meta.len() as u32)?;

        Ok(MapDiffReader { lookup_table, diff })
    }
}

impl<T: Read + Seek> MapDiffReader<T> {
    /// Create a MapDiffReader from existing lookup and data readers
    pub fn from_readable<U: Read + Seek>(
        mut lookup_reader: U,
        data_reader: T,
        lookup_file_length: u32,
    ) -> MulReaderResult<MapDiffReader<T>> {
        let lookup_table = generate_lookup_table(&mut lookup_reader, lookup_file_length)?;

        Ok(MapDiffReader {
            lookup_table,
            diff: data_reader,
        })
    }

    /// Read a map block, if one exists
    pub fn read(&mut self, idx: u32) -> Option<MulReaderResult<Block>> {
        match self.lookup_table.get(&idx) {
            Some(block_idx) => Some(read_block(&mut self.diff, *block_idx)),
            None => None,
        }
    }

    /// Read all map blocks
    pub fn read_all(&mut self) -> HashMap<u32, MulReaderResult<Block>> {
        let mut out = HashMap::new();
        let keys = self.lookup_table.keys().copied().collect::<Vec<u32>>();
        for map_idx in keys {
            out.insert(
                map_idx,
                self.read(map_idx)
                    .expect("Tried to read cached lookup that no longer exists"),
            );
        }
        out
    }
}

/// A struct to help read out static locations for a block from a diff file
#[derive(Debug)]
pub struct StaticLocationDiffReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
    lookup_table: HashMap<u32, u32>,
}

impl StaticLocationDiffReader<File> {
    /// Create a new StaticLocationDiffReader from a lookup path, an index path and mul path
    pub fn new(
        lookup_path: &Path,
        diff_idx_path: &Path,
        diff_path: &Path,
    ) -> MulReaderResult<StaticLocationDiffReader<File>> {
        let mut lookup = File::open(lookup_path)?;

        let meta = lookup.metadata()?;
        let lookup_table = generate_lookup_table(&mut lookup, meta.len() as u32)?;
        let mul_reader = MulReader::new(diff_idx_path, diff_path)?;

        Ok(StaticLocationDiffReader {
            mul_reader,
            lookup_table,
        })
    }
}

impl<T: Read + Seek> StaticLocationDiffReader<T> {
    /// Create a StaticLocationDiffReader from existing lookup and mul readers
    pub fn from_mul_reader<U: Read + Seek>(
        mut lookup_reader: U,
        mul_reader: MulReader<T>,
        lookup_file_length: u32,
    ) -> MulReaderResult<StaticLocationDiffReader<T>> {
        let lookup_table = generate_lookup_table(&mut lookup_reader, lookup_file_length)?;

        Ok(StaticLocationDiffReader {
            mul_reader,
            lookup_table,
        })
    }

    /// Read statics for a map block, if they exist
    pub fn read(&mut self, idx: u32) -> Option<MulReaderResult<Vec<StaticLocation>>> {
        match self.lookup_table.get(&idx) {
            Some(block_idx) => Some(read_block_statics(&mut self.mul_reader, *block_idx)),
            None => None,
        }
    }

    /// Read all static locations
    pub fn read_all(&mut self) -> HashMap<u32, MulReaderResult<Vec<StaticLocation>>> {
        let mut out = HashMap::new();
        let keys = self.lookup_table.keys().copied().collect::<Vec<u32>>();
        for map_idx in keys {
            out.insert(
                map_idx,
                self.read(map_idx)
                    .expect("Tried to read cached lookup that no longer exists"),
            );
        }
        out
    }
}
