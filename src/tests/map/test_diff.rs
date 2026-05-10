use byteorder::{LittleEndian, WriteBytesExt};
use std::{
    collections::HashSet,
    io::{Cursor, Result, Write},
};

use crate::{
    map::diff::{MapDiffReader, StaticLocationDiffReader},
    mul::tests::simple_from_vecs,
    tests::map::helpers::{raw_block, raw_static_location},
};

fn raw_map() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..2 {
        data.write_all(&raw_block()?)?;
    }
    Ok(data.into_inner())
}

fn raw_references() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(1234)?;
    data.write_u32::<LittleEndian>(5678)?;
    Ok(data.into_inner())
}

fn raw_static_locations() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_all(&raw_static_location()?)?;
    Ok(data.into_inner())
}

#[test]
fn test_map_diff_read() {
    let reader = Cursor::new(raw_map().unwrap());
    let references = Cursor::new(raw_references().unwrap());
    let mut diff_reader = MapDiffReader::from_readable(references, reader, 8).unwrap();
    let missing_entry = diff_reader.read(1235);
    assert!(missing_entry.is_none());
    let known_entry = diff_reader.read(1234);
    assert!(known_entry.is_some());
    assert_eq!(known_entry.unwrap().unwrap().cells[0].graphic, 3);
}

#[test]
fn test_map_diff_read_all() {
    let reader = Cursor::new(raw_map().unwrap());
    let references = Cursor::new(raw_references().unwrap());
    let mut diff_reader = MapDiffReader::from_readable(references, reader, 8).unwrap();
    let patches = diff_reader.read_all();
    assert_eq!(
        patches.keys().collect::<HashSet<&u32>>(),
        HashSet::from_iter(&[1234, 5678])
    );
}

#[test]
fn test_static_diff_read() {
    let mul_reader = simple_from_vecs(vec![
        (raw_static_locations().unwrap(), 0, 0),
        (raw_static_locations().unwrap(), 0, 0),
    ]);
    let references = Cursor::new(raw_references().unwrap());
    let mut diff_reader =
        StaticLocationDiffReader::from_mul_reader(references, mul_reader, 8).unwrap();
    let missing_entry = diff_reader.read(1235);
    assert!(missing_entry.is_none());
    let known_entry = diff_reader.read(1234);
    assert!(known_entry.is_some());
    assert_eq!(known_entry.unwrap().unwrap().len(), 1);
}

#[test]
fn test_static_diff_read_all() {
    let mul_reader = simple_from_vecs(vec![
        (raw_static_locations().unwrap(), 0, 0),
        (raw_static_locations().unwrap(), 0, 0),
    ]);
    let references = Cursor::new(raw_references().unwrap());
    let mut diff_reader =
        StaticLocationDiffReader::from_mul_reader(references, mul_reader, 8).unwrap();
    let patches = diff_reader.read_all();
    assert_eq!(
        patches.keys().collect::<HashSet<&u32>>(),
        HashSet::from_iter(&[1234, 5678])
    );
}
