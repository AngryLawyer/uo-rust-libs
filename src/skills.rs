export Skill;
export load_skills;

type Skill = {
    clickable: bool,
    name: ~[u8]
};

fn load_skills(root_path: ~str) -> ~[Skill] {
    let maybe_reader: option::option<mul_reader::MulReader> = mul_reader::reader(root_path, ~"skills.idx", ~"skills.mul");

    if option::is_none(maybe_reader) {
        io::println("Error reading skills");
        assert false;
    }

    let reader: mul_reader::MulReader = option::get(maybe_reader);

    let mut result:~[Skill] = ~[];

    while (reader.eof() != true) {
        let item: option::option<mul_reader::MulRecord> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::MulRecord = option::get(item);
            vec::push(result, {
                clickable: vec::head(unwrapped.data) == 1,
                name: vec::tail(unwrapped.data)
            });
        }
    }

    return result;
}
