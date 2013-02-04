use mul_reader;
use byte_helpers;

pub struct Gump {
    data_size: u16,
    width: u16,
    height: u16
}

pub struct GumpReader {
    mul_reader: mul_reader::MulReader
}

impl GumpReader {
    fn read_gump(&self, id: uint) -> option::Option<Gump> {
        option::None
    }
}

pub fn GumpReader(index_path: &path::Path, mul_path: &path::Path) -> result::Result<GumpReader, ~str> {
    match mul_reader::MulReader(index_path, mul_path) {
        result::Err(message) => result::Err(message),
        result::Ok(mul_reader) => {
            result::Ok(GumpReader{
                mul_reader: mul_reader
            })
        }
    }
}
