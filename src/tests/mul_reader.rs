use mul_reader::{MulReader, MulWriter};
use std::path::BytesContainer;

#[test]
fn test_load_mulreader() {
    match MulReader::new(&Path::new("./src/tests/testdata/test_skills.idx"), &Path::new("./src/tests/testdata/test_skills.mul")) {
        Ok(_mul_reader) => {
            //Passed
        },
        Err(message) => fail!(message)
    }
}

#[test]
fn test_read_first_entry() {
    match MulReader::new(&Path::new("./src/tests/testdata/test_skills.idx"), &Path::new("./src/tests/testdata/test_skills.mul")) {
        Ok(mut mul_reader) => {
            let record = mul_reader.read(0);
            match record {
                Some(_record) => {
                    //Passed
                },
                None => {
                    fail!("Couldn't find a record!")
                }
            }
        },
        Err(message) => fail!(message)
    }
}

#[test]
fn test_read_impossible_entry() {
    match MulReader::new(&Path::new("./src/tests/testdata/test_skills.idx"), &Path::new("./src/tests/testdata/test_skills.mul")) {
        Ok(mut mul_reader) => {
            let record = mul_reader.read(999999);
            match record {
                Some(_record) => {
                    fail!("Unexpectedly read a result from a known impossible address")
                },
                None => {
                    //Passed
                }
            }
        },
        Err(message) => fail!(message)
    }
}

#[test]
fn test_write_simple_mul() {
    match MulWriter::new(&Path::new("./bin/test_mul_out.idx"), &Path::new("./bin/test_mul_out.mul")) {
        Ok(mut mul_writer) => {
            let mut out_buffer = Vec::from_slice(String::from_str("Bakery").to_c_str().as_bytes());
            out_buffer.unshift(1);
            mul_writer.append(&out_buffer, None, None);
        },
        Err(message) => fail!(message)
    }
}
