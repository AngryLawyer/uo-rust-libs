//! Skill objects represent named skills that appear in UO's Skills menu.
//! They also contain a flag denoting whether they are clicked to activate
//!
//! Skills are traditionally stored in skills.mul/skills.idx
//!
//! Skills are encoded as a list of
//! |clickable:u8|name:c-string|

use crate::errors::{MulReaderError, MulReaderResult};
use crate::mul::MulReader;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use std::str::from_utf8;

/// A skill name, plus whether it requires clicking to activate
pub struct Skill {
    pub clickable: bool,
    pub name: String,
}

impl Skill {
    /// Create a new Skill object
    pub fn new(clickable: bool, name: String) -> Skill {
        Skill { clickable, name }
    }

    /// Convert a skill back into its canonical mul representation
    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![if self.clickable { 1 } else { 0 }];
        vec.extend_from_slice(self.name.as_bytes());
        vec.push(0);
        vec
    }
}

pub struct SkillReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

impl SkillReader<File> {
    /// Create a new SkillReader from an index and mul path
    pub fn new(index_path: &Path, mul_path: &Path) -> MulReaderResult<SkillReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(SkillReader { mul_reader })
    }
}

impl<T: Read + Seek> SkillReader<T> {
    pub fn from_mul(reader: MulReader<T>) -> SkillReader<T> {
        SkillReader { mul_reader: reader }
    }

    pub fn read_skill(&mut self, id: u32) -> MulReaderResult<Skill> {
        let record = self.mul_reader.read(id)?;
        let slice = &record.data[1..record.data.len() - 1];
        match from_utf8(slice) {
            Ok(string) => Ok(Skill::new(record.data[0] == 1, String::from(string))),
            Err(e) => Err(MulReaderError::FailedParse(format!(
                "Failed to parse skill at index {} - {}",
                id, e
            ))),
        }
    }

    pub fn read_all(&mut self) -> Vec<Skill> {
        let mut result = vec![];
        let mut id = 0;
        loop {
            match self.read_skill(id) {
                Ok(skill) => {
                    result.push(skill);
                    id += 1;
                }
                Err(_) => break,
            }
        }
        result
    }
}
