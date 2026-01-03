//! Skill objects represent named skills that appear in UO's Skills menu.
//! They also contain a flag denoting whether they are clicked to activate

use crate::mul_reader::MulReader;
use std::io::{Error, Read, Result, Seek};
use std::path::Path;
use std::str::from_utf8;

pub struct Skill {
    pub clickable: bool,
    pub name: String,
}

impl Skill {
    pub fn new(clickable: bool, name: String) -> Skill {
        Skill { clickable, name }
    }

    /**
     * Convert a skill back into its canonical mul representation
     */
    pub fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![if self.clickable { 1 } else { 0 }];
        vec.extend_from_slice(self.name.as_bytes());
        vec.push(0);
        vec
    }
}

/// A reader of a Skills file-like object
///
/// Skills are traditionally stored in skills.mul/skills.idx
///
/// Skills are encoded as a list of
/// |clickable:u8|name:c-string|
pub struct Skills {
    pub skills: Vec<Skill>,
}

impl Skills {
    pub fn from_mul<T: Seek + Read>(reader: &mut MulReader<T>) -> Result<Skills> {
        //Unpack the lot
        let mut result = vec![];
        let mut id = 0;

        while let Ok(record) = reader.read(id) {
            let slice = &record.data[1..record.data.len() - 1];
            match from_utf8(slice) {
                Ok(string) => {
                    result.push(Skill::new(record.data[0] == 1, String::from(string)));
                    id += 1;
                }
                Err(e) => {
                    return Err(Error::other(format!(
                        "Failed to parse skill at index {} - {}",
                        id, e
                    )));
                }
            }
        }

        Ok(Skills { skills: result })
    }

    pub fn new(index_path: &Path, mul_path: &Path) -> Result<Skills> {
        let mut reader = MulReader::new(index_path, mul_path)?;
        Skills::from_mul(&mut reader)
    }
}
