use byteorder::{LittleEndian, WriteBytesExt};

use crate::{
    error::MulReaderResult,
    map::{diff::StaticLocationDiffReader, static_location::StaticLocationReader},
    mul::tests::simple_from_vecs,
    tests::map::helpers::raw_static_location,
};
use std::io::{Cursor, Result, Write};

fn raw_static_locations() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..3 {
        data.write_all(&raw_static_location(23)?)?;
    }
    Ok(data.into_inner())
}

fn raw_patch() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..2 {
        data.write_all(&raw_static_location(24)?)?;
    }
    Ok(data.into_inner())
}

fn raw_references() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(2)?;
    data.write_u32::<LittleEndian>(5678)?;
    Ok(data.into_inner())
}

fn static_location_reader() -> MulReaderResult<StaticLocationReader<Cursor<Vec<u8>>>> {
    let raw_locations = raw_static_locations()?;
    let mul_reader = simple_from_vecs((0..4).map(|_| (raw_locations.clone(), 0, 0)).collect());
    Ok(StaticLocationReader::from_mul(mul_reader, 2, 2))
}

fn diff_reader() -> MulReaderResult<StaticLocationDiffReader<Cursor<Vec<u8>>>> {
    let mul_reader = simple_from_vecs(vec![(raw_patch()?, 0, 0), (raw_patch()?, 0, 0)]);
    let references = Cursor::new(raw_references()?);
    StaticLocationDiffReader::from_mul_reader(references, mul_reader, 8)
}

#[test]
fn test_read_block() {
    let mut static_location_reader = static_location_reader().unwrap();
    let block = static_location_reader.read_block(2, None).unwrap();
    assert_eq!(block.len(), 3);
}

#[test]
fn test_read_block_from_coordinates() {
    let mut static_location_reader = static_location_reader().unwrap();
    let block = static_location_reader
        .read_block_from_coordinates(1, 1, None)
        .unwrap();
    assert_eq!(block.len(), 3);
}

#[test]
fn test_read_block_patch() {
    let mut static_location_reader = static_location_reader().unwrap();
    let mut diff_reader = diff_reader().unwrap();
    let block = static_location_reader
        .read_block(2, Some(&mut diff_reader))
        .unwrap();
    assert_eq!(block.len(), 2);
    let block = static_location_reader
        .read_block(1, Some(&mut diff_reader))
        .unwrap();
    assert_eq!(block.len(), 3);
}
