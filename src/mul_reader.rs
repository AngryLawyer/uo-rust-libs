const undef_record:u32 = 0xFEFEFEFF;

pub type MulRecord = {
    data: ~[u8],
    start: u32,
    length: u32,
    opt1: u16,
    opt2: u16
};

pub struct MulReader {
    idx_reader: io::Reader,
    data_reader: io::Reader,
    mut index: uint,
    mut is_eof: bool,
}

//TODO: we should really be passing in a Path, rather than a String
pub fn reader(path: ~str, idx_name: ~str, mul_name: ~str) -> result::Result<MulReader, ~str>{
    //Try to load the two readers

    let idx_path: path::Path = path::Path(path + idx_name);
    match io::file_reader(&idx_path) {
        result::Ok(idx_reader) => {
            let mul_path: path::Path = path::Path(path + mul_name);
            match io::file_reader(&mul_path) {
                result::Ok(data_reader) => {
                    result::Ok(MulReader {
                        idx_reader: move idx_reader,
                        data_reader: move data_reader,
                        index: 0,
                        is_eof: false
                    })
                },
                result::Err(error_message) => {
                    result::Err(error_message)
                }
            }
        },
        result::Err(error_message) => {
            result::Err(error_message)
        }
    }

}

impl MulReader {
    fn eof(&self) -> bool {
        return self.is_eof;
    }

    fn read(&self) -> Option<MulRecord> {
        //Check for eof
        if (self.eof() == true) { return option::None };
        let idx_reader = self.idx_reader as io::ReaderUtil;
        
        let start: u32 = idx_reader.read_le_uint_n(4) as u32;
        let length: u32 = idx_reader.read_le_uint_n(4) as u32;
        let opt1: u16 = idx_reader.read_le_uint_n(2) as u16;
        let opt2: u16 = idx_reader.read_le_uint_n(2) as u16;

        self.index += 1; //Increment our pointer

        //Set EOF if needed
        if (self.idx_reader.eof()) {
            self.is_eof = true;
        }

        //Check for empty cell
        if (start == undef_record || start == u32::max_value) { 
            return option::None;
        };
        
        self.data_reader.seek(start as int, io::SeekSet);
        let reader_util = self.data_reader as io::ReaderUtil; 
        return option::Some({
            data: reader_util.read_bytes(length as uint),
            start: start,
            length: length,
            opt1: opt1,
            opt2: opt2
        });
    }
}
