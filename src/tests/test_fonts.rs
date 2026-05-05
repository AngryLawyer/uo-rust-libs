use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Result, Write};

#[cfg(feature = "image")]
use crate::fonts::Character;
use crate::fonts::FontReader;
#[cfg(feature = "image")]
use image::Pixel;

fn raw_character() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u8(3)?; // Width
    data.write_u8(3)?; // Height
    data.write_u8(0)?; // Unknown
    for i in 0..9 {
        data.write_u16::<LittleEndian>(if i % 2 == 0 { 0x7FFF } else { 0x0 })?;
    }
    Ok(data.into_inner())
}

fn raw_font_data() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u8(0x0)?; // Empty header
    for _ in 0..224 {
        data.write_all(&raw_character()?)?;
    }
    Ok(data.into_inner())
}

fn raw_fonts_data() -> Result<Vec<u8>> {
    let mut data = vec![];
    for _ in 0..10 {
        data.append(&mut (raw_font_data()?));
    }
    Ok(data)
}

#[test]
fn test_read_fonts() {
    let file_reader = Cursor::new(raw_fonts_data().unwrap());
    let mut reader = FontReader::from_readable(file_reader);
    let fonts = reader.read_fonts().unwrap();
    assert_eq!(fonts.len(), 10);
    let font_1 = &fonts[0];
    assert_eq!(font_1.characters.len(), 224);
    let character_1 = &font_1.characters[0];
    assert_eq!(character_1.width, 3);
    assert_eq!(character_1.height, 3);
    assert_eq!(
        character_1.data,
        [0x7FFF, 0x0, 0x7FFF, 0x0, 0x7FFF, 0x0, 0x7FFF, 0x0, 0x7FFF]
    );
}

#[cfg(feature = "image")]
#[test]
fn test_character_to_image() {
    let character = Character {
        width: 3,
        height: 3,
        unknown: 0,
        data: vec![0x7FFF, 0x0, 0x7FFF, 0x0, 0x7FFF, 0x0, 0x7FFF, 0x0, 0x7FFF],
    };
    let image = character.to_image();
    assert_eq!(image.width(), 3);
    assert_eq!(image.height(), 3);
    let transparent = [0, 0, 0, 0];
    let white = [255, 255, 255, 255];
    assert_eq!(image.get_pixel(0, 0).channels(), white);
    assert_eq!(image.get_pixel(1, 0).channels(), transparent);
    assert_eq!(image.get_pixel(2, 0).channels(), white);

    assert_eq!(image.get_pixel(0, 1).channels(), transparent);
    assert_eq!(image.get_pixel(1, 1).channels(), white);
    assert_eq!(image.get_pixel(2, 1).channels(), transparent);

    assert_eq!(image.get_pixel(0, 2).channels(), white);
    assert_eq!(image.get_pixel(1, 2).channels(), transparent);
    assert_eq!(image.get_pixel(2, 2).channels(), white);
}
