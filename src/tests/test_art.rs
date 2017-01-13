use std::io::{Cursor};

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
    ]);
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
fn test_tile_to_32bit() {
    let tile = Tile {
        header: 0,
        image_data: [0xFFFF; 1022]
    };
    let (width, height, data) = tile.to_32bit();
    assert_eq!(width, 44);
    assert_eq!(height, 44);
    assert_eq!(data.len(), 44 * 44);

    // Check the first row
    for i in 0..44 {
        if i == 21 || i == 22 {
            assert_eq!(data[i], 0xFFFFFFFF);
        } else {
            assert_eq!(data[i], 0);
        }
    }

    // Check the middle row
    for i in 0..44 {
        assert_eq!(data[i + (22 * 44)], 0xFFFFFFFF);
    }
}

#[test]
#[cfg(feature = "use-sdl2")]
fn test_tile_to_surface() {
    let tile = Tile {
        header: 0,
        image_data: [0xFFFF; 1022]
    };
    let surface = tile.to_surface();
    assert_eq!(surface.width(), 44);
    assert_eq!(surface.height(), 44);
    let data = surface.without_lock().unwrap();
    assert_eq!(data.len(), 44 * 44 * 4);

    // Check the first row
    for i in 0..44 {
        if i == 21 || i == 22 {
            assert_eq!(data[(i * 4)], 0xFF);
            assert_eq!(data[(i * 4) + 1], 0xFF);
            assert_eq!(data[(i * 4) + 2], 0xFF);
            assert_eq!(data[(i * 4) + 3], 0xFF);
        } else {
            assert_eq!(data[(i * 4)], 0);
            assert_eq!(data[(i * 4) + 1], 0);
            assert_eq!(data[(i * 4) + 2], 0);
            assert_eq!(data[(i * 4) + 3], 0);
        }
    }

    // Check the middle row
    for i in 0..44 {
        assert_eq!(data[((i + (22 * 44)) * 4)], 0xFF);
        assert_eq!(data[((i + (22 * 44)) * 4) + 1], 0xFF);
        assert_eq!(data[((i + (22 * 44)) * 4) + 2], 0xFF);
        assert_eq!(data[((i + (22 * 44)) * 4) + 3], 0xFF);
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

    let mul_reader = simple_from_vecs(padded);
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
            let (width, height, data) = stat.to_32bit();
            assert_eq!(stat.width, width as u16);
            assert_eq!(stat.height, height as u16);
            assert_eq!(data, vec![
                0, 0xFFFFFFFF, 0,
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                0, 0xFFFFFFFF, 0
            ]);
        },
        Ok(_) => {
            panic!("Got Tile instead of Static");
        },
        Err(err) => panic!("{}", err)
    };
}

#[test]
#[cfg(feature = "use-sdl2")]
fn test_static_to_surface() {
    let mut reader = example_art_mul();
    match reader.read(STATIC_OFFSET) {
        Ok(TileOrStatic::Static(stat)) => {
            let surface = stat.to_surface();
            assert_eq!(surface.width(), 3);
            assert_eq!(surface.height(), 3);
            let data = surface.without_lock().expect("Failed to get surface data");
            let expectation = [
                0, 0, 0, 0,           255, 255, 255, 255,   0, 0, 0, 0,
                255, 255, 255, 255,   255, 255, 255, 255,   255, 255, 255, 255,
                0, 0, 0, 0,           255, 255, 255, 255,   0, 0, 0, 0,
            ];
            for (left, right) in data.iter().zip(expectation.iter()) {
                assert_eq!(left, right);
            }
        },
        Ok(_) => {
            panic!("Got Tile instead of Static");
        },
        Err(err) => panic!("{}", err)
    };
}

/*#[test]
fn dump_art() {
    use mul_reader::MulWriter;
    use art::{Art, Tile};
    use std::io::FileMode;
    let dummy_tile = Tile {header: 0, image_data: [0xFFFF, ..1022]};
    let mut mul_writer = MulWriter::new(&Path::new("./testdata/test_art.idx"), &Path::new("./testdata/test_art.mul"), FileMode::Truncate).ok().expect("Can't open files for writing");

    let serialized = dummy_tile.serialize();

    for i in range(0, 0x4000u32) {
        mul_writer.append(&serialized, None, None).ok().expect("Can't write white tile");
    }
}*/
