use std::io::{Cursor, Result};
use crate::map::radarcol::RadarColReader;
use byteorder::{LittleEndian, WriteBytesExt};

fn raw_color_table() -> Result<Vec<u8>> {
    let mut out = Cursor::new(vec![]);
    for i in 0..=65535 {
        out.write_u16::<LittleEndian>(i)?;
    }
    Ok(out.into_inner())
}

#[test]
fn test_read() {
    let raw = raw_color_table().unwrap();
    let len = raw.len();
    let data = Cursor::new(raw);
    let mut reader = RadarColReader::from_readable(data, len as u32);
    assert_eq!(reader.read(1).unwrap(), 1);
}

#[test]
fn test_read_all() {
    let raw = raw_color_table().unwrap();
    let len = raw.len();
    let data = Cursor::new(raw);
    let mut reader = RadarColReader::from_readable(data, len as u32);
    assert_eq!(reader.read_all().unwrap().len(), 65536);
}

