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

#[test]
fn dump_art() {
    use mul_reader::MulWriter;
    let mul_writer = MulWriter::new(&Path::new("./testdata/test_art.idx"), &Path::new("./testdata/test_art.mul")).ok().expect("Can't open files for writing");
}
