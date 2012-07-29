export skill;
export load_skills;

type skill = {
    clickable: bool,
    name: ~[u8]
};

fn load_skills(root_path: ~str) -> ~[skill] {

    let reader:mul_reader::mul_reader = mul_reader::mul_reader(root_path, ~"skills.idx", ~"skills.mul");

    let mut result:~[skill] = ~[];

    while (reader.eof() != true) {
        let item: option::option<mul_reader::mul_record> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::mul_record = option::get(item);
            vec::push(result, {
                clickable: vec::head(unwrapped.data) == 1,
                name: vec::tail(unwrapped.data)
            });
        }
    }

    ret result;
}
