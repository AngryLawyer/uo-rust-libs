export get_writer;
export extract_muls;

fn extract_muls(path: ~str, idx: ~str, mul: ~str, name: ~str) {
    let reader:mul_reader::mul_reader = mul_reader::mul_reader(path, idx, mul);
    
    let mut index:uint = 0;
    while (reader.eof() != true) {
        let item: option::option<mul_reader::mul_record> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::mul_record = option::get(item);
            slice_mul(unwrapped, #fmt("%s-%u", name, index))
        }
        index += 1;
    }
}

fn get_writer(path: ~str) -> io::writer {
    
    let maybe_writer = io::file_writer(path, ~[io::create, io::truncate]);

    if result::is_err::<io::writer, ~str>(maybe_writer) {
        io::println(#fmt("%s", result::get_err(maybe_writer)));
        fail;
    }

    return result::unwrap(maybe_writer);
}

fn slice_mul(record: mul_reader::mul_record, name: ~str) {
    let header: io::writer = get_writer(#fmt("./output/%s.mulheader", name));
    let body: io::writer = get_writer(#fmt("./output/%s.mulslice", name));
    io::u64_to_le_bytes(record.start as u64, 4u, |v| header.write(v));
    io::u64_to_le_bytes(record.length as u64, 4u, |v| header.write(v));
    io::u64_to_le_bytes(record.opt1 as u64, 2u, |v| header.write(v));
    io::u64_to_le_bytes(record.opt2 as u64, 2u, |v| header.write(v));
    body.write(record.data);
}

