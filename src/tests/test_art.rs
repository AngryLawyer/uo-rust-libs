use std::path::Path;
use art::{ArtReader, TileOrStatic};

#[test]
fn test_load_tile() {
    let mut reader = ArtReader::new(&Path::new("./testdata/test_art.idx"), &Path::new("./testdata/test_art.mul")).ok().expect("Couldn't load test_art.mul");
    match reader.read(0) {
        Ok(TileOrStatic::Tile(tile)) => {
            //ok
        },
        Ok(_) => {
            panic!("Got Static instead of Tile");
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
