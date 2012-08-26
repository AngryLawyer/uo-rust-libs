export mul_record;
export mul_reader;

const undef_record:u32 = 0xFEFEFEFF;

type mul_record = {
    data: ~[u8],
    start: u32,
    length: u32,
    opt1: u16,
    opt2: u16
};

struct mul_reader {
    idx_reader: io::reader;
    data_reader: io::reader;
    mut index: uint;
    mut is_eof: bool;
}

fn mul_reader(path: ~str, idx_name: ~str, mul_name: ~str) -> option<mul_reader>{
    //Try to load the two readers
    let maybe_idx_reader: result::result<io::reader, ~str> = io::file_reader(path + idx_name);

    if result::is_err::<io::reader, ~str>(maybe_idx_reader) {
        io::println(#fmt("%s", result::get_err(maybe_idx_reader)));
        return option::none;
    }

    let maybe_data_reader: result::result<io::reader, ~str> = io::file_reader(path + mul_name);

    if result::is_err::<io::reader, ~str>(maybe_data_reader) {
        io::println(#fmt("%s", result::get_err(maybe_data_reader)));
        return option::none;
    }

    return mul_reader {
        idx_reader: result::unwrap(maybe_idx_reader),
        data_reader: result::unwrap(maybe_data_reader),
        index: 0,
        is_eof: false
    };
}


/*class mul_reader {

    priv {
        let idx_reader: io::reader;
        let data_reader: io::reader;
        let mut index: uint;
        let mut is_eof: bool;
    }

    new(path:~str, idx_name:~str, mul_name:~str) {

        self.index = 0;
        self.is_eof = false;
 
        let maybe_idx_reader: result::result<io::reader, ~str> = io::file_reader(path + idx_name);
        self.idx_reader = result::unwrap(maybe_idx_reader);

        let maybe_data_reader: result::result<io::reader, ~str> = io::file_reader(path + mul_name);

        self.data_reader = result::unwrap(maybe_data_reader);
        //TODO: Error checking
    }

    fn eof() -> bool {
        return self.is_eof;
    }

    fn read() -> option<mul_record> {

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
        
        self.data_reader.seek(start as int, io::seek_set);

        return option::some({
            data: self.data_reader.read_bytes(length as uint),
            start: start,
            length: length,
            opt1: opt1,
            opt2: opt2
        });
    }
}*/
