use mul_reader::{MulReader, MulWriter, MulWriterMode};
use std::path::Path;
use std::ffi::CString;


#[test]
fn test_load_mulreader() {
    match MulReader::new(&Path::new("./testdata/test_skills.idx"), &Path::new("./testdata/test_skills.mul")) {
        Ok(_mul_reader) => {
            //Passed
        },
        Err(message) => panic!("{}", message)
    }
}

#[test]
fn test_read_first_entry() {
    match MulReader::new(&Path::new("./testdata/test_skills.idx"), &Path::new("./testdata/test_skills.mul")) {
        Ok(mut mul_reader) => {
            let record = mul_reader.read(0);
            match record {
                Ok(_record) => {
                    //Passed
                },
                Err(err) => {
                    panic!(err)
                }
            }
        },
        Err(message) => panic!("{}", message)
    }
}

#[test]
fn test_read_impossible_entry() {
    match MulReader::new(&Path::new("./testdata/test_skills.idx"), &Path::new("./testdata/test_skills.mul")) {
        Ok(mut mul_reader) => {
            let record = mul_reader.read(999999);
            match record {
                Ok(_record) => {
                    panic!("Unexpectedly read a result from a known impossible address")
                },
                Err(_) => {
                    //Passed
                }
            }
        },
        Err(message) => panic!("{}", message)
    }
}

#[test]
fn test_write_simple_mul() {
    match MulWriter::new(&Path::new("./target/test_mul_out.idx"), &Path::new("./target/test_mul_out.mul"), MulWriterMode::Truncate) {
        Ok(mut mul_writer) => {
            let mut out_buffer = CString::new("Bakery").unwrap().into_bytes();
            out_buffer.insert(0, 1);
            match mul_writer.append(&out_buffer, None, None) {
                Ok(_) => {
                    //Success
                },
                Err(message) => panic!("{}", message)
            }
        },
        Err(message) => panic!("{}", message)
    }
}
