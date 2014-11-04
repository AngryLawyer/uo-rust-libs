use mul_reader::{MulReader, MulWriter};

use std::io::Truncate;

#[test]
fn test_load_mulreader() {
    match MulReader::new(&Path::new("./testdata/test_skills.idx"), &Path::new("./testdata/test_skills.mul")) {
        Ok(_mul_reader) => {
            //Passed
        },
        Err(message) => panic!(message)
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
                    panic!(err.desc)
                }
            }
        },
        Err(message) => panic!(message)
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
        Err(message) => panic!(message)
    }
}

#[test]
fn test_write_simple_mul() {
    match MulWriter::new(&Path::new("./bin/test_mul_out.idx"), &Path::new("./bin/test_mul_out.mul"), Truncate) {
        Ok(mut mul_writer) => {
            let mut out_buffer = "Bakery".to_c_str().as_bytes().to_vec();
            out_buffer.insert(0, 1);
            match mul_writer.append(&out_buffer, None, None) {
                Ok(_) => {
                    //Success
                },
                Err(message) => panic!(message)
            }
        },
        Err(message) => panic!(message)
    }
}
