/*pub fn extract_muls(path: ~str, idx: ~str, mul: ~str, name: ~str) {
    match mul_reader::reader(path, idx, mul) {
        result::Ok(reader) => {
            let mut index:uint = 0;
            while (reader.eof() != true) {
                match reader.read() {
                    option::Some(item) => {
                        slice_mul(item, fmt!("%s-%u", name, index));
                    },
                    option::None => ()
                };
                index += 1;
            }
        },
        result::Err(message) => {
            io::println(fmt!("Error reading tiles - %s", message));
        }
    };

}

pub fn get_writer(path: ~str) -> io::Writer {
    
    match io::file_writer(&path::Path(path), ~[io::Create, io::Truncate]) {
        result::Err(message) => {
            io::println(fmt!("%s", message));
            fail;
        },
        result::Ok(writer) => writer
    }
}

fn slice_mul(record: mul_reader::MulRecord, name: ~str) {
    let header: io::Writer = get_writer(fmt!("./output/%s.mulheader", name));
    let body: io::Writer = get_writer(fmt!("./output/%s.mulslice", name));
    io::u64_to_le_bytes(record.start as u64, 4u, |v| header.write(v));
    io::u64_to_le_bytes(record.length as u64, 4u, |v| header.write(v));
    io::u64_to_le_bytes(record.opt1 as u64, 2u, |v| header.write(v));
    io::u64_to_le_bytes(record.opt2 as u64, 2u, |v| header.write(v));
    body.write(record.data);
}
*/
