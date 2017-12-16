use std::io::{Cursor};
use image::Pixel;

use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

use mul_reader::{simple_from_vecs};
use art::{ArtReader, TileOrStatic, Art, Tile, Static, RunPair, STATIC_OFFSET};

#[test]
fn test_load_tile() {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(0).unwrap();  // Header
    for _i in 0..1022 {
        data.write_u16::<LittleEndian>(0xFFFF).unwrap();
    }

    let mul_reader = simple_from_vecs(vec![
        data.into_inner(),
    ], 0, 0);
    let mut reader = ArtReader::from_mul(mul_reader);
    match reader.read(0) {
        Ok(TileOrStatic::Tile(tile)) => {
            assert_eq!(tile.header, 0);
            assert_eq!(tile.image_data[0], 0xFFFF);
        },
        Ok(_) => {
            panic!("Got Static instead of Tile");
        },
        Err(err) => panic!("{}", err)
    };
}

#[test]
fn test_tile_to_image() {
    let tile = Tile {
        header: 0,
        image_data: [0xFFFF; 1022]
    };
    let image = tile.to_image();
    assert_eq!(image.width(), 44);
    assert_eq!(image.height(), 44);

    // Check the first row
    for i in 0..44 {
        if i == 21 || i == 22 {
            assert_eq!(image.get_pixel(i, 0).channels4(), (0xFF, 0xFF, 0xFF, 0xFF));
        } else {
            assert_eq!(image.get_pixel(i, 0).channels4(), (0, 0, 0, 0));
        }
    }

    // Check the middle row
    for i in 0..44 {
        assert_eq!(image.get_pixel(i, 22).channels4(), (0xFF, 0xFF, 0xFF, 0xFF));
    }
}

fn example_art_mul() -> ArtReader<Cursor<Vec<u8>>> {
    let mut data = Cursor::new(vec![]);

    data.write_u16::<LittleEndian>(0).unwrap();  //Size, unused
    data.write_u16::<LittleEndian>(1).unwrap();  //Trigger, unknown
    data.write_u16::<LittleEndian>(3).unwrap();  //Width in pixels
    data.write_u16::<LittleEndian>(3).unwrap();  //Height in pixels

    data.write_u16::<LittleEndian>(0).unwrap();  //Row 1 offset
    data.write_u16::<LittleEndian>(5).unwrap();  //Row 2 offset
    data.write_u16::<LittleEndian>(12).unwrap();  //Row 3 offset

    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 1, run 1 data
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 1 EOL

    data.write_u16::<LittleEndian>(0).unwrap();  //Row 2, run 1 spacer
    data.write_u16::<LittleEndian>(3).unwrap();  //Row 2, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 2, run 1 data
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 2, run 1 data
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 2, run 1 data
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 2 EOL

    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 1, run 1 data
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 2 EOL

    let mut padded = vec![];
    for _i in 0..STATIC_OFFSET {
        padded.push(vec![]);
    }
    padded.push(data.into_inner());

    let mul_reader = simple_from_vecs(padded, 0, 0);
    ArtReader::from_mul(mul_reader)
}

#[test]
fn test_load_static() {
    let mut reader = example_art_mul();
    match reader.read(STATIC_OFFSET) {
        Ok(TileOrStatic::Static(stat)) => {
            assert_eq!(stat.size, 0);
            assert_eq!(stat.trigger, 1);
            assert_eq!(stat.width, 3);
            assert_eq!(stat.height, 3);
        },
        Ok(_) => {
            panic!("Got Tile instead of Static");
        },
        Err(err) => panic!("{}", err)
    };
}

#[test]
fn test_static_to_image() {
    let mut reader = example_art_mul();
    match reader.read(STATIC_OFFSET) {
        Ok(TileOrStatic::Static(stat)) => {
            let image = stat.to_image();
            assert_eq!(image.width(), 3);
            assert_eq!(image.height(), 3);
            let transparent = (0, 0, 0, 0);
            let white = (255, 255, 255, 255);
            assert_eq!(image.get_pixel(0, 0).channels4(), transparent);
            assert_eq!(image.get_pixel(1, 0).channels4(), white);
            assert_eq!(image.get_pixel(2, 0).channels4(), transparent);

            assert_eq!(image.get_pixel(0, 1).channels4(), white);
            assert_eq!(image.get_pixel(1, 1).channels4(), white);
            assert_eq!(image.get_pixel(2, 1).channels4(), white);

            assert_eq!(image.get_pixel(0, 2).channels4(), transparent);
            assert_eq!(image.get_pixel(1, 2).channels4(), white);
            assert_eq!(image.get_pixel(2, 2).channels4(), transparent);
        },
        Ok(_) => {
            panic!("Got Tile instead of Static");
        },
        Err(err) => panic!("{}", err)
    };
}
