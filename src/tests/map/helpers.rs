use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Result};

pub fn raw_block() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(0)?;
    for i in 0..64 {
        data.write_u16::<LittleEndian>(3)?;
        data.write_i8(i)?;
    }
    Ok(data.into_inner())
}

pub fn raw_static_location() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u16::<LittleEndian>(23)?; // Object id
    data.write_u8(2)?; // X
    data.write_u8(3)?; // Y
    data.write_i8(-1)?; // Z
    data.write_u16::<LittleEndian>(0)?; // Checksum
    Ok(data.into_inner())
}
