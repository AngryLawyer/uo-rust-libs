use std::{result, option, path, str};
use mul_reader;

pub struct Skill {
    clickable: bool,
    name: ~str
}

pub struct SkillReader {
    mul_reader: mul_reader::MulReader
}

impl SkillReader {
    pub fn read_skill(&self, id: uint) -> option::Option<Skill>{
        self.mul_reader.read(id).chain(|record| {
            option::Some(Skill {
                clickable: (*record.data.head() == 1),
                name: str::from_bytes(record.data.slice(1, record.data.len() - 1))
            })
        })
    }
}

pub fn SkillReader(index_path: &path::Path, mul_path: &path::Path) -> result::Result<SkillReader, ~str> {
    mul_reader::MulReader::new(index_path, mul_path).chain(|mul_reader| {
        result::Ok(SkillReader {
            mul_reader: mul_reader
        })
    })
}

pub fn load_skills(index_path: &path::Path, mul_path: &path::Path) -> result::Result<~[Skill], ~str> {

    SkillReader(index_path, mul_path).chain(|reader| {
        let mut result:~[Skill] = ~[];
        let mut id:uint = 0;
        
        loop {
            match reader.read_skill(id) {
                option::Some(skill) => {
                    result.push(skill);
                },
                option::None => {
                    break;
                }
            }
            id += 1;
        }

        result::Ok(result)
   })
}
