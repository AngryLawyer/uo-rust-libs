use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Result, Write};
use crate::map::shared::{read_block, read_block_statics};

fn raw_block() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(0)?;
    for i in 0..64 {
        data.write_u16::<LittleEndian>(3)?;
        data.write_i8(i)?;
    }
    Ok(data.into_inner())
}

fn raw_map() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..4 {
        data.write_all(&raw_block()?)?;
    }
    Ok(data.into_inner())
}

fn raw_static_location() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u16::<LittleEndian>(23)?; // Object id
    data.write_u8(2)?; // X
    data.write_u8(3)?; // Y
    data.write_i8(-1)?; // Z
    data.write_u16::<LittleEndian>(0)?; // Checksum
    Ok(data.into_inner())
}

fn raw_static_locations() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    let raw_location = raw_static_location()?;
    data.write_all(&raw_location)?;
    data.write_all(&raw_location)?;
    data.write_all(&raw_location)?;
    Ok(data.into_inner())
}

#[test]
fn test_read_block() {
    let mut reader = Cursor::new(raw_map().unwrap());
    let result = read_block(&mut reader, 1).unwrap();
    assert_eq!(result.checksum, 0);
    assert_eq!(result.cells.len(), 64);
    assert_eq!(result.cells[3].graphic, 3);
    assert_eq!(result.cells[3].altitude, 3);
}

#[test]
fn test_read_block_statics() {
}
