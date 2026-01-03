#[cfg(feature = "image")]
use image::Pixel;

#[cfg(feature = "image")]
fn example_gump_mul() -> GumpReader<Cursor<Vec<u8>>> {
    let mut data = Cursor::new(vec![]);

    data.write_u32::<LittleEndian>(3).unwrap(); //Row 1 offset
    data.write_u32::<LittleEndian>(6).unwrap(); //Row 2 offset
    data.write_u32::<LittleEndian>(7).unwrap(); //Row 3 offset

    data.write_u16::<LittleEndian>(0).unwrap(); //Black
    data.write_u16::<LittleEndian>(1).unwrap(); //One pixel

    data.write_u16::<LittleEndian>(0xFFFF).unwrap(); //White
    data.write_u16::<LittleEndian>(1).unwrap(); //1 pixel

    data.write_u16::<LittleEndian>(0).unwrap(); //Black
    data.write_u16::<LittleEndian>(1).unwrap(); //1 pixel

    data.write_u16::<LittleEndian>(0xFFFF).unwrap(); //White
    data.write_u16::<LittleEndian>(3).unwrap(); //3 pixels

    data.write_u16::<LittleEndian>(0).unwrap(); //Black
    data.write_u16::<LittleEndian>(1).unwrap(); //One pixel

    data.write_u16::<LittleEndian>(0xFFFF).unwrap(); //White
    data.write_u16::<LittleEndian>(1).unwrap(); //1 pixel

    data.write_u16::<LittleEndian>(0).unwrap(); //Black
    data.write_u16::<LittleEndian>(1).unwrap(); //1 pixel

    let mul_reader = simple_from_vecs(vec![data.into_inner()], 3, 3);
    GumpReader::from_mul(mul_reader)
}

#[test]
#[cfg(feature = "image")]
fn test_to_image() {
    let mut reader = example_gump_mul();
    match reader.read_gump(0) {
        Ok(gump) => {
            let image = gump.to_image();
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
        Err(err) => panic!("{}", err),
    };
}
