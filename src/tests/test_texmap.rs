use crate::mul::tests::simple_from_vecs;
use crate::texmap::TexMapReader;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Result};

#[cfg(feature = "image")]
use image::Pixel;

fn example_large_texmap() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..(128 * 128) {
        data.write_u16::<LittleEndian>(0x7C00)?; // Red
    }
    Ok(data.into_inner())
}
fn example_small_texmap() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for _ in 0..(64 * 64) {
        data.write_u16::<LittleEndian>(0x1F)?; // Blue
    }
    Ok(data.into_inner())
}

#[test]
fn test_read() {
    let mul_reader = simple_from_vecs(vec![
        (example_large_texmap().unwrap(), 0, 0),
        (example_small_texmap().unwrap(), 0, 0),
    ]);
    let mut reader = TexMapReader::from_mul(mul_reader);
    let large = reader.read(0).unwrap();
    let small = reader.read(1).unwrap();
    assert_eq!(large.data.len(), 128 * 128);
    assert_eq!(small.data.len(), 64 * 64);
}

#[test]
#[cfg(feature = "image")]
fn test_to_image() {
    let mul_reader = simple_from_vecs(vec![
        (example_large_texmap().unwrap(), 0, 0),
        (example_small_texmap().unwrap(), 0, 0),
    ]);
    let mut reader = TexMapReader::from_mul(mul_reader);
    let large = reader.read(0).unwrap().to_image();
    let small = reader.read(1).unwrap().to_image();
    assert_eq!(large.width(), 128);
    assert_eq!(large.height(), 128);
    assert_eq!(small.width(), 64);
    assert_eq!(small.height(), 64);
    for pixel in large.pixels() {
        assert_eq!(pixel.channels(), [255, 0, 0, 255]);
    }
    for pixel in small.pixels() {
        assert_eq!(pixel.channels(), [0, 0, 255, 255]);
    }
}
