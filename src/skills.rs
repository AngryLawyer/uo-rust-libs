use std::result;
use std::option;
use std::path;
use std::str;
use std::vec;
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
        match self.mul_reader.read(id) {
            option::None => option::None,
            option::Some(record) => {
                option::Some(Skill {
                    clickable: (*record.data.head() == 1),
                    name: str::from_bytes(record.data.slice(1, record.data.len() - 1))
                })
            }
        }
    }
}

pub fn SkillReader(index_path: &path::Path, mul_path: &path::Path) -> result::Result<SkillReader, ~str> {
    match mul_reader::MulReader::new(index_path, mul_path) {
        result::Err(message) => result::Err(message),
        result::Ok(mul_reader) => {
            result::Ok(SkillReader{
                mul_reader: mul_reader
            })
        }
    }
}

pub fn load_skills(index_path: &path::Path, mul_path: &path::Path) -> result::Result<~[Skill], ~str> {

    match SkillReader(index_path, mul_path) {
        result::Ok(reader) => {
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
        },
        result::Err(message) => result::Err(message)
   }
}
