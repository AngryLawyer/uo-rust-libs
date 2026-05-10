use crate::{
    map::{
        StaticLocation,
        shared::{read_block, read_block_statics},
    },
    mul::tests::simple_from_vecs,
    tests::map::helpers::{raw_block, raw_static_location},
};
use std::io::{Cursor, Result, Write};

fn raw_map() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..4 {
        data.write_all(&raw_block(3)?)?;
    }
    Ok(data.into_inner())
}

fn raw_static_locations() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    let raw_location = raw_static_location(23)?;
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
    let mut mul_reader = simple_from_vecs(vec![
        (raw_static_locations().unwrap(), 0, 0),
        (raw_static_locations().unwrap(), 0, 0),
        (raw_static_locations().unwrap(), 0, 0),
    ]);
    let locations = read_block_statics(&mut mul_reader, 1).unwrap();
    assert_eq!(locations.len(), 3);
    let second_loc = locations[1];
    assert_eq!(
        second_loc,
        StaticLocation {
            object_id: 23,
            x: 2,
            y: 3,
            altitude: -1,
            checksum: 0
        }
    )
}
