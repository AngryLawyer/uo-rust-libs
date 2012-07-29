export mul_record;
export mul_reader;

const undef_record:u32 = 0xFEFEFEFF;

type mul_record = {
    data: ~[u8],
    opt1: u16,
    opt2: u16
};

class mul_reader {

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
        ret self.is_eof;
    }

    fn read() -> option<mul_record> {

        //Check for eof
        if (self.eof() == true) { ret option::none };
        
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
            ret option::none;
        };
        
        self.data_reader.seek(start as int, io::seek_set);

        ret option::some({
            data: self.data_reader.read_bytes(length as uint),
            opt1: opt1,
            opt2: opt2
        });
    }
}
