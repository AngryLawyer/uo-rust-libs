use mul_reader::MulReader;

#[test]
fn test_load_mulreader() {
    match MulReader::new(&Path::new("./src/tests/testdata/test_skills.idx"), &Path::new("./src/tests/testdata/test_skills.mul")) {
        Ok(mul_reader) => {
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
                Some(record) => {
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
