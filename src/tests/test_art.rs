#[cfg(feature = "image")]
use image::Pixel;
use std::io::{Cursor, Result};

use byteorder::{LittleEndian, WriteBytesExt};

#[cfg(feature = "image")]
use crate::art::{Art, Tile};
use crate::art::{ArtReader, STATIC_OFFSET};
use crate::mul::tests::simple_from_vecs;

fn raw_tile_data() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(0x6)?; // Header
    for i in 0..1022 {
        data.write_u16::<LittleEndian>(if i % 2 == 0 { 0xFFFF } else { 0x0 })?;
    }
    Ok(data.into_inner())
}

#[test]
fn test_read_tile() {
    let mul_reader = simple_from_vecs(vec![(raw_tile_data().unwrap(), 0, 0)]);
    let mut reader = ArtReader::from_mul(mul_reader);
    match reader.read_tile(0) {
        Ok(tile) => {
            assert_eq!(tile.header, 0x6);
            assert_eq!(tile.image_data[0], 0xFFFF);
            assert_eq!(tile.image_data[1], 0x0);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_serialize_tile() {
    let raw = raw_tile_data().unwrap();
    let mul_reader = simple_from_vecs(vec![(raw.clone(), 0, 0)]);
    let mut reader = ArtReader::from_mul(mul_reader);
    let tile = reader.read_tile(0).unwrap();
    let serialized = tile.serialize();
    assert_eq!(raw, serialized);
}

#[cfg(feature = "image")]
#[test]
fn test_tile_to_image() {
    let tile = Tile {
        header: 0,
        image_data: [0xFFFF; 1022],
    };
    let image = tile.to_image();
    assert_eq!(image.width(), 44);
    assert_eq!(image.height(), 44);

    // Check the first row
    for i in 0..44 {
        if i == 21 || i == 22 {
            assert_eq!(image.get_pixel(i, 0).channels(), [0xFF, 0xFF, 0xFF, 0xFF]);
        } else {
            assert_eq!(image.get_pixel(i, 0).channels(), [0, 0, 0, 0]);
        }
    }

    // Check the middle row
    for i in 0..44 {
        assert_eq!(image.get_pixel(i, 22).channels(), [0xFF, 0xFF, 0xFF, 0xFF]);
    }
}

fn raw_static() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);

    data.write_u16::<LittleEndian>(0)?; //Size, unused
    data.write_u16::<LittleEndian>(1)?; //Trigger, unknown
    data.write_u16::<LittleEndian>(3)?; //Width in pixels
    data.write_u16::<LittleEndian>(3)?; //Height in pixels

    data.write_u16::<LittleEndian>(0)?; //Row 1 offset
    data.write_u16::<LittleEndian>(5)?; //Row 2 offset
    data.write_u16::<LittleEndian>(12)?; //Row 3 offset

    data.write_u16::<LittleEndian>(1)?; //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1)?; //Row 1, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF)?; //Row 1, run 1 data
    data.write_u32::<LittleEndian>(0)?; //Row 1 EOL

    data.write_u16::<LittleEndian>(0)?; //Row 2, run 1 spacer
    data.write_u16::<LittleEndian>(3)?; //Row 2, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF)?; //Row 2, run 1 data
    data.write_u16::<LittleEndian>(0xFFFF)?; //Row 2, run 1 data
    data.write_u16::<LittleEndian>(0xFFFF)?; //Row 2, run 1 data
    data.write_u32::<LittleEndian>(0)?; //Row 2 EOL

    data.write_u16::<LittleEndian>(1)?; //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1)?; //Row 1, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF)?; //Row 1, run 1 data
    data.write_u32::<LittleEndian>(0)?; //Row 2 EOL
    Ok(data.into_inner())
}

fn example_art_mul(static_data: &[u8]) -> ArtReader<Cursor<Vec<u8>>> {
    let mut padded = vec![];
    for _i in 0..STATIC_OFFSET {
        padded.push((vec![], 0, 0));
    }
    padded.push((static_data.into(), 0, 0));

    let mul_reader = simple_from_vecs(padded);
    ArtReader::from_mul(mul_reader)
}

#[test]
fn test_read_static() {
    let mut reader = example_art_mul(&raw_static().unwrap());
    let stat = reader.read_static(0).unwrap();
    assert_eq!(stat.size, 0);
    assert_eq!(stat.trigger, 1);
    assert_eq!(stat.width, 3);
    assert_eq!(stat.height, 3);
}

#[test]
fn test_serialize_static() {
    let raw = raw_static().unwrap();
    let mut reader = example_art_mul(&raw);
    let s = reader.read_static(0).unwrap();
    let serialized = s.serialize();
    assert_eq!(raw, serialized);
}

#[test]
#[cfg(feature = "image")]
fn test_static_to_image() {
    let mut reader = example_art_mul(&raw_static().unwrap());
    let stat = reader.read_static(0).unwrap();
    let image = stat.to_image();
    assert_eq!(image.width(), 3);
    assert_eq!(image.height(), 3);
    let transparent = [0, 0, 0, 0];
    let white = [255, 255, 255, 255];
    assert_eq!(image.get_pixel(0, 0).channels(), transparent);
    assert_eq!(image.get_pixel(1, 0).channels(), white);
    assert_eq!(image.get_pixel(2, 0).channels(), transparent);

    assert_eq!(image.get_pixel(0, 1).channels(), white);
    assert_eq!(image.get_pixel(1, 1).channels(), white);
    assert_eq!(image.get_pixel(2, 1).channels(), white);

    assert_eq!(image.get_pixel(0, 2).channels(), transparent);
    assert_eq!(image.get_pixel(1, 2).channels(), white);
    assert_eq!(image.get_pixel(2, 2).channels(), transparent);
}
