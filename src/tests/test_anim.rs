use byteorder::{LittleEndian, WriteBytesExt};
#[cfg(feature = "image")]
use image::Pixel;
use std::io::{Cursor, Result, Write};

use crate::{anim::AnimReader, mul::tests::simple_from_vecs};

fn raw_palette() -> Vec<u16> {
    (0..256)
        .map(|idx| {
            let channel = (31.0 * ((1.0 / 256.0) * (idx + 1) as f32)) as u16;
            channel | (channel << 5) | (channel << 10)
        })
        .collect()
}

fn raw_animation() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    for cell in raw_palette() {
        data.write_u16::<LittleEndian>(cell)?;
    }
    data.write_u32::<LittleEndian>(0x1)?; // Frame count
    data.write_u32::<LittleEndian>(0x8)?; // One frame, offset by 8 from the palette                                         
    data.write_u16::<LittleEndian>(0x1)?; // Center is 1 pixel in
    data.write_u16::<LittleEndian>(0x1)?; // Center is 1 pixel in
    data.write_u16::<LittleEndian>(0x3)?; // Width
    data.write_u16::<LittleEndian>(0x3)?; // Height
    // Draw a white cross on a transparent background
    data.write_u32::<LittleEndian>((0x3FE << 12) | 0x1)?; // 1 pixel run, offset by y - 1
    data.write_u8(0xFF)?; // White pixel
    data.write_u32::<LittleEndian>((0x3FF << 22) | (0x3FD << 12) | 0x3)?; // 3 pixel run, offset by x - 1
    data.write_all(&[0xFF, 0xFF, 0xFF])?; // White pixels
    data.write_u32::<LittleEndian>((0x3FC << 12) | 0x1)?; // 3 pixel run, offset by y + 1
    data.write_u8(0xFF)?; // White pixel
    data.write_u32::<LittleEndian>(0x7FFF7FFF)?; // Stop value
    Ok(data.into_inner())
}

#[test]
fn test_read_anim() {
    let mul_reader = simple_from_vecs(vec![(raw_animation().unwrap(), 0, 0)]);
    let mut reader = AnimReader::from_mul(mul_reader);
    let anim = reader.read(0).unwrap();
    assert_eq!(anim.palette.len(), 256);
    assert_eq!(anim.frame_count, 1);
    assert_eq!(anim.frames.len(), anim.frame_count as usize);
    let frame = &anim.frames[0];
    assert_eq!(frame.image_center_x, 1);
    assert_eq!(frame.image_center_y, 1);
    assert_eq!(frame.width, 3);
    assert_eq!(frame.height, 3);
    assert_eq!(frame.data.len(), 3);

    assert_eq!(frame.data[0].x_offset(1), 1);
    assert_eq!(frame.data[0].y_offset(1, 3), 2);
    assert_eq!(frame.data[0].image_data, &[255]);
    assert_eq!(frame.data[1].x_offset(1), 0);
    assert_eq!(frame.data[1].y_offset(1, 3), 1);
    assert_eq!(frame.data[1].image_data, &[255, 255, 255]);
    assert_eq!(frame.data[2].x_offset(1), 1);
    assert_eq!(frame.data[2].y_offset(1, 3), 0);
    assert_eq!(frame.data[2].image_data, &[255]);
}

#[cfg(feature = "image")]
#[test]
fn test_to_frames() {
    let mul_reader = simple_from_vecs(vec![(raw_animation().unwrap(), 0, 0)]);
    let mut reader = AnimReader::from_mul(mul_reader);
    let anim = reader.read(0).unwrap();
    let frames = anim.to_frames().collect_frames().unwrap();
    assert_eq!(frames.len(), 1);
    let white = [255, 255, 255, 255];
    let transparent = [0, 0, 0, 0];
    let image = &frames[0].buffer();
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
