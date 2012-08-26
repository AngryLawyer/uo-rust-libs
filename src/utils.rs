export get_writer;
export extract_muls;

fn extract_muls(path: ~str, idx: ~str, mul: ~str, name: ~str) {
    let maybe_reader: option::option<mul_reader::MulReader> = mul_reader::reader(path, idx, mul);

    if option::is_none(maybe_reader) {
        io::println("Error reading tiles");
        assert false;
    }

    let reader: mul_reader::MulReader = option::get(maybe_reader);
    let mut index:uint = 0;
    while (reader.eof() != true) {
        let item: option::option<mul_reader::MulRecord> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::MulRecord = option::get(item);
            slice_mul(unwrapped, #fmt("%s-%u", name, index))
        }
        index += 1;
    }
}

fn get_writer(path: ~str) -> io::Writer {
    
    let maybe_writer = io::file_writer(path, ~[io::Create, io::Truncate]);

    if result::is_err::<io::Writer, ~str>(maybe_writer) {
        io::println(#fmt("%s", result::get_err(maybe_writer)));
        fail;
    }

    return result::unwrap(maybe_writer);
}

fn slice_mul(record: mul_reader::MulRecord, name: ~str) {
    let header: io::Writer = get_writer(#fmt("./output/%s.mulheader", name));
    let body: io::Writer = get_writer(#fmt("./output/%s.mulslice", name));
    io::u64_to_le_bytes(record.start as u64, 4u, |v| header.write(v));
    io::u64_to_le_bytes(record.length as u64, 4u, |v| header.write(v));
    io::u64_to_le_bytes(record.opt1 as u64, 2u, |v| header.write(v));
    io::u64_to_le_bytes(record.opt2 as u64, 2u, |v| header.write(v));
    body.write(record.data);
}

