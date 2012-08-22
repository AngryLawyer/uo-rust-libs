export get_writer;
export extract_muls;

fn extract_muls(path: ~str, idx: ~str, mul: ~str, name: ~str) {
    let reader:mul_reader::mul_reader = mul_reader::mul_reader(root_path, idx, mul);
    
    let mut index:uint = 0;
    while (reader.eof() != true) {
        let item: option::option<mul_reader::mul_record> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::mul_record = option::get(item);
            slice_mul(unwrapped, #fmt("%s-%i", name, index))
        }
        index += 1;
    }
}

fn get_writer(path: ~string) -> io::writer {
    
    let maybe_writer = io::file_writer(path, ~[io::create, io::truncate]);

    if result::is_err::<io::writer, ~str>(maybe_writer) {
        io::println(#fmt("%s", result::get_err(maybe_writer)));
        assert false;
    }

    result::unwrap(maybe_writer);
}

fn slice_mul(record: mul_reader::mul_record, name: ~str) {

    let header: io::writer = get_writer(#fmt("%s.mulheader", name));
    let body: io::writer = get_writer(#fmt("%s.mulslice", name));

    header.write_le_u16(record.opt1);
    header.write_le_u16(record.opt2);

    for record.data.each |byte| {
        body.write_u8(byte);
    }
}

