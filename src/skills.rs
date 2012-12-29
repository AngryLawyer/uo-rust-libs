pub type Skill = {
    clickable: bool,
    name: ~[u8]
};

pub fn load_skills(root_path: ~str) -> ~[Skill] {
    match mul_reader::reader(root_path, ~"skills.idx", ~"skills.mul") {
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
}
