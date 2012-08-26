export MulRecord;
export MulReader;
export reader;

const undef_record:u32 = 0xFEFEFEFF;

type MulRecord = {
    data: ~[u8],
    start: u32,
    length: u32,
    opt1: u16,
    opt2: u16
};

struct MulReader {
    idx_reader: io::Reader;
    data_reader: io::Reader;
    mut index: uint;
    mut is_eof: bool;
}

fn reader(path: ~str, idx_name: ~str, mul_name: ~str) -> option<MulReader>{
    //Try to load the two readers
    let maybe_idx_reader: result::result<io::Reader, ~str> = io::file_reader(path + idx_name);

    if result::is_err::<io::Reader, ~str>(maybe_idx_reader) {
        io::println(#fmt("%s", result::get_err(maybe_idx_reader)));
        return option::none;
    }

    let maybe_data_reader: result::result<io::Reader, ~str> = io::file_reader(path + mul_name);

    if result::is_err::<io::Reader, ~str>(maybe_data_reader) {
        io::println(#fmt("%s", result::get_err(maybe_data_reader)));
        return option::none;
    }

    return option::some(MulReader {
        idx_reader: result::unwrap(maybe_idx_reader),
        data_reader: result::unwrap(maybe_data_reader),
        index: 0,
        is_eof: false
    });
}

impl MulReader {
    fn eof(&self) -> bool {
        return self.is_eof;
    }

    fn read(&self) -> option<MulRecord> {
        //Check for eof
        if (self.eof() == true) { return option::none };
        
        let start: u32 = self.idx_reader.read_le_uint(4) as u32;
        let length: u32 = self.idx_reader.read_le_uint(4) as u32;
        let opt1: u16 = self.idx_reader.read_le_uint(2) as u16;
        let opt2: u16 = self.idx_reader.read_le_uint(2) as u16;

        self.index += 1; //Increment our pointer

        //Set EOF if needed
        if (self.idx_reader.eof()) {
            self.is_eof = true;
        }

        //Check for empty cell
        if (start == undef_record || start == u32::max_value) { 
            return option::none;
        };
        
        self.data_reader.seek(start as int, io::SeekSet);

        return option::some({
            data: self.data_reader.read_bytes(length as uint),
            start: start,
            length: length,
            opt1: opt1,
            opt2: opt2
        });
    }
}
