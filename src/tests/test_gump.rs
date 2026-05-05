use crate::gump::GumpReader;
use crate::mul::tests::simple_from_vecs;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Result};

#[cfg(feature = "image")]
use image::Pixel;

#[cfg(feature = "image")]
fn example_gump_mul() -> Result<(Vec<u8>, u16, u16)> {
    let mut data = Cursor::new(vec![]);

    data.write_u32::<LittleEndian>(3)?; //Row 1 offset
    data.write_u32::<LittleEndian>(6)?; //Row 2 offset
    data.write_u32::<LittleEndian>(7)?; //Row 3 offset

    data.write_u16::<LittleEndian>(0)?; //Black
    data.write_u16::<LittleEndian>(1)?; //1 pixel

    data.write_u16::<LittleEndian>(0xFFFF)?; //White
    data.write_u16::<LittleEndian>(1)?; //1 pixel

    data.write_u16::<LittleEndian>(0)?; //Black
    data.write_u16::<LittleEndian>(1)?; //1 pixel

    data.write_u16::<LittleEndian>(0xFFFF)?; //White
    data.write_u16::<LittleEndian>(3)?; //3 pixels

    data.write_u16::<LittleEndian>(0)?; //Black
    data.write_u16::<LittleEndian>(1)?; //1 pixel

    data.write_u16::<LittleEndian>(0xFFFF)?; //White
    data.write_u16::<LittleEndian>(1)?; //1 pixel

    data.write_u16::<LittleEndian>(0)?; //Black
    data.write_u16::<LittleEndian>(1)?; //1 pixel
    Ok((data.into_inner(), 3, 3))
}

#[test]
fn test_read() {
    let mul_reader = simple_from_vecs(vec![example_gump_mul().unwrap()]);
    let mut reader = GumpReader::from_mul(mul_reader);
    let gump = reader.read(0).unwrap();
    assert_eq!(gump.width, 3);
    assert_eq!(gump.height, 3);
    assert_eq!(gump.data.len(), 3);
    assert_eq!(gump.data[0].len(), 3);
    assert_eq!(gump.data[1].len(), 1);
    assert_eq!(gump.data[2].len(), 3);
}

#[test]
#[cfg(feature = "image")]
fn test_to_image() {
    let mul_reader = simple_from_vecs(vec![example_gump_mul().unwrap()]);
    let mut reader = GumpReader::from_mul(mul_reader);
    let gump = reader.read(0).unwrap();
    let image = gump.to_image().unwrap();
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
