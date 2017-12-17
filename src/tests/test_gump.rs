use mul_reader::{simple_from_vecs};
use std::io::{Cursor};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use gump::GumpReader;
use image::Pixel;

fn example_gump_mul() -> GumpReader<Cursor<Vec<u8>>> {
    let mut data = Cursor::new(vec![]);

    data.write_u32::<LittleEndian>(3).unwrap();  //Row 1 offset
    data.write_u32::<LittleEndian>(6).unwrap();  //Row 2 offset
    data.write_u32::<LittleEndian>(7).unwrap();  //Row 3 offset

    data.write_u16::<LittleEndian>(0).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer

    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 1, run 1 data
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 number of pixels

    data.write_u16::<LittleEndian>(0).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer

    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 2, run 1 data
    data.write_u16::<LittleEndian>(3).unwrap();  //Row 2 EOL

    data.write_u16::<LittleEndian>(0).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer

    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 1, run 1 data
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 number of pixels

    data.write_u16::<LittleEndian>(0).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer

    let mul_reader = simple_from_vecs(vec![data.into_inner()], 3, 0);
    GumpReader::from_mul(mul_reader)
}

#[test]
fn test_gump() {
    let mut reader = example_gump_mul();
    match reader.read_gump(0) {
        Ok(gump) => {
            let image = gump.to_image();
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
        Err(err) => panic!("{}", err)
    };
}
