use mul_reader::{MulReader, MulWriter, MulWriterMode, simple_from_vecs};
use std::path::Path;
use std::ffi::CString;
use std::io::Cursor;
use byteorder::{LittleEndian, WriteBytesExt};

fn fake_reader() -> MulReader<Cursor<Vec<u8>>> {
    let mut idx_cursor = Cursor::new(vec![]);
    idx_cursor.write_u32::<LittleEndian>(0).unwrap();  //Position
    idx_cursor.write_u32::<LittleEndian>(1).unwrap();  //Length
    idx_cursor.write_u16::<LittleEndian>(0).unwrap();  //Opt1
    idx_cursor.write_u16::<LittleEndian>(0).unwrap();  //Opt2
    idx_cursor.write_u32::<LittleEndian>(1).unwrap();  //Position
    idx_cursor.write_u32::<LittleEndian>(4).unwrap();  //Length
    idx_cursor.write_u16::<LittleEndian>(2).unwrap();  //Opt1
    idx_cursor.write_u16::<LittleEndian>(3).unwrap();  //Opt2

    let mut data_cursor = Cursor::new(vec![]);
    data_cursor.write_u8(255).unwrap();
    data_cursor.write_u32::<LittleEndian>(0xdeadbeef).unwrap();
    MulReader::from_readables(idx_cursor, data_cursor)
}

#[test]
fn test_read_entries() {
    let mut mul_reader = fake_reader();
    let record1 = mul_reader.read(0);
    let record2 = mul_reader.read(1);
    match record1 {
        Ok(record) => {
            assert_eq!(record.start, 0);
            assert_eq!(record.length, 1);
            assert_eq!(record.opt1, 0);
            assert_eq!(record.opt2, 0);
            assert_eq!(record.data.len(), 1);
            assert_eq!(record.data[0], 255);
        },
        Err(err) => {
            panic!(err)
        }
    };
    match record2 {
        Ok(record) => {
            assert_eq!(record.start, 1);
            assert_eq!(record.length, 4);
            assert_eq!(record.opt1, 2);
            assert_eq!(record.opt2, 3);
            assert_eq!(record.data.len(), 4);
        },
        Err(err) => {
            panic!(err)
        }
    }
}

#[test]
fn test_read_entried_provided_by_helper() {
    let mut data_cursor = Cursor::new(vec![]);
    data_cursor.write_u32::<LittleEndian>(0xdeadbeef).unwrap();
    let mut mul_reader = simple_from_vecs(vec![
        vec![255],
        data_cursor.into_inner()
    ]);
    let record1 = mul_reader.read(0);
    let record2 = mul_reader.read(1);
    match record1 {
        Ok(record) => {
            assert_eq!(record.start, 0);
            assert_eq!(record.length, 1);
            assert_eq!(record.opt1, 0);
            assert_eq!(record.opt2, 0);
            assert_eq!(record.data.len(), 1);
            assert_eq!(record.data[0], 255);
        },
        Err(err) => {
            panic!(err)
        }
    };
    match record2 {
        Ok(record) => {
            assert_eq!(record.start, 1);
            assert_eq!(record.length, 4);
            assert_eq!(record.opt1, 0);
            assert_eq!(record.opt2, 0);
            assert_eq!(record.data.len(), 4);
        },
        Err(err) => {
            panic!(err)
        }
    }
}

#[test]
fn test_read_impossible_entry() {
    let mut mul_reader = fake_reader();
    let record = mul_reader.read(999999);
    match record {
        Ok(_record) => {
            panic!("Unexpectedly read a result from a known impossible address")
        },
        Err(_) => {
            //Passed
        }
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
