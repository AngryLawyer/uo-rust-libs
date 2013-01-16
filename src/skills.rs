pub struct Skill = {
    clickable: bool,
    name: ~[u8]
};

pub struct SkillReader {
    mul_reader: mul_reader::MulReader
}

impl SkillReader {
    fn read_skill() -> option::Option<Skill>{
    }
};

pub fn SkillReader(index_path: &path::Path, mul_path: &path::Path) -> result::Result<SkillReader, ~str> {
    match mul_reader::MulReader(index_path, mul_path) {
        result::Err(message) => result::Err(message),
        result::Ok(mul_reader) => {
            result::Ok(SkillReader{
                mul_reader: mul_reader
            })
        }
    }
}

/*pub fn load_skills(root_path: ~str) -> ~[Skill] {
    match mul_reader::MulReader(root_path + ~"skills.idx", root_path + ~"skills.mul") {
        result::Ok(reader) => {
            let mut result:~[Skill] = ~[];

            while (reader.eof() != true) {
                let item: option::Option<mul_reader::MulRecord> = reader.read();
                if option::is_some(&item) {
                    let unwrapped: mul_reader::MulRecord = option::unwrap(item);
                    result.push({
                        clickable: vec::head(unwrapped.data) == 1,
                        name: vec::tail(unwrapped.data)
                    });
                }
            }

            return result;
        },
        result::Err(message) => {
            io::println(fmt!("Error reading skills - %s", message));
            fail;
        }
    }
}*/
