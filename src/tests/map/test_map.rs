use byteorder::{LittleEndian, WriteBytesExt};

use crate::{
    error::MulReaderResult,
    map::{MapReader, diff::MapDiffReader},
    tests::map::helpers::raw_block,
};
use std::io::{Cursor, Result, Write};

fn raw_map() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..4 {
        data.write_all(&raw_block(3)?)?;
    }
    Ok(data.into_inner())
}

fn raw_patch() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..2 {
        data.write_all(&raw_block(4)?)?;
    }
    Ok(data.into_inner())
}

fn raw_references() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(2)?;
    data.write_u32::<LittleEndian>(5678)?;
    Ok(data.into_inner())
}

fn map_reader() -> MulReaderResult<MapReader<Cursor<Vec<u8>>>> {
    let reader = Cursor::new(raw_map()?);
    Ok(MapReader::from_readable(reader, 2, 2))
}

fn diff_reader() -> MulReaderResult<MapDiffReader<Cursor<Vec<u8>>>> {
    let reader = Cursor::new(raw_patch()?);
    let references = Cursor::new(raw_references()?);
    MapDiffReader::from_readable(references, reader, 8)
}

#[test]
fn test_read_block() {
    let mut map_reader = map_reader().unwrap();
    let block = map_reader.read_block::<Cursor<Vec<u8>>>(2, None).unwrap();
    assert_eq!(block.cells[0].graphic, 3);
}

#[test]
fn test_read_block_from_coordinates() {
    let mut map_reader = map_reader().unwrap();
    let block = map_reader
        .read_block_from_coordinates::<Cursor<Vec<u8>>>(1, 1, None)
        .unwrap();
    assert_eq!(block.cells[0].graphic, 3);
}

#[test]
fn test_read_block_patch() {
    let mut map_reader = map_reader().unwrap();
    let mut diff_reader = diff_reader().unwrap();
    let block = map_reader.read_block(2, Some(&mut diff_reader)).unwrap();
    assert_eq!(block.cells[0].graphic, 4);
    let block = map_reader.read_block(1, Some(&mut diff_reader)).unwrap();
    assert_eq!(block.cells[0].graphic, 3);
}
