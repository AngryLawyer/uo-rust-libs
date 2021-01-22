use super::shared::{read_block, read_block_statics, Block, StaticLocation};
use byteorder::{LittleEndian, ReadBytesExt};
use mul_reader::MulReader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result, Seek};
use std::path::Path;

pub struct MapDiffReader {
    lookup_table: HashMap<u32, u32>,
    diff: File,
}

impl MapDiffReader {
    pub fn new(lookup_path: &Path, diff_path: &Path) -> Result<MapDiffReader> {
        // Start by reading all of the lookup info
        let mut lookup = File::open(lookup_path)?;
        let diff = File::open(diff_path)?;

        let meta = lookup.metadata()?;
        let mut lookup_table = HashMap::new();

        for i in 0..(meta.len() as u32 / 4) {
            lookup_table.insert(lookup.read_u32::<LittleEndian>()?, i);
        }

        Ok(MapDiffReader { lookup_table, diff })
    }

    pub fn read(&mut self, idx: u32) -> Option<Result<Block>> {
        match self.lookup_table.get(&idx) {
            Some(block_idx) => Some(read_block(&mut self.diff, *block_idx)),
            None => None,
        }
    }

    pub fn read_all(&mut self) -> HashMap<u32, Result<Block>> {
        let mut out = HashMap::new();
        let keys = self
            .lookup_table
            .keys()
            .map(|key| *key)
            .collect::<Vec<u32>>();
        for map_idx in keys {
            out.insert(map_idx, self.read(map_idx).unwrap());
        }
        out
    }
}

pub struct StaticDiffReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
    lookup_table: HashMap<u32, u32>,
}

impl StaticDiffReader<File> {
    pub fn new(
        lookup_path: &Path,
        diff_idx_path: &Path,
        diff_path: &Path,
    ) -> Result<StaticDiffReader<File>> {
        let mut lookup = File::open(lookup_path)?;

        let meta = lookup.metadata()?;
        let mut lookup_table = HashMap::new();

        for i in 0..(meta.len() as u32 / 4) {
            lookup_table.insert(lookup.read_u32::<LittleEndian>()?, i);
        }
        let mul_reader = MulReader::new(diff_idx_path, diff_path)?;

        Ok(StaticDiffReader {
            mul_reader,
            lookup_table,
        })
    }
}

impl<T: Read + Seek> StaticDiffReader<T> {
    pub fn read(&mut self, idx: u32) -> Option<Result<Vec<StaticLocation>>> {
        match self.lookup_table.get(&idx) {
            Some(block_idx) => Some(read_block_statics(&mut self.mul_reader, *block_idx)),
            None => None,
        }
    }

    pub fn read_all(&mut self) -> HashMap<u32, Result<Vec<StaticLocation>>> {
        let mut out = HashMap::new();
        let keys = self
            .lookup_table
            .keys()
            .map(|key| *key)
            .collect::<Vec<u32>>();
        for map_idx in keys {
            out.insert(map_idx, self.read(map_idx).unwrap());
        }
        out
    }
}
