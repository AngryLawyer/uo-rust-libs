use art::{ArtReader, TileWrapper};

#[test]
fn test_load_tile() {
    let mut reader = ArtReader::new(&Path::new("./testdata/test_art.idx"), &Path::new("./testdata/test_art.mul")).ok().expect("Couldn't load test_art.mul");
    match reader.read(0) {
        Ok(TileWrapper(tile)) => {
            //ok
        },
        Ok(_) => {
            panic!("Got Static instead of Tile");
        },
        Err(err) => panic!("{}", err)
    };
}
