use art::ArtReader;

#[test]
fn test_load_tile() {
    let mut reader = ArtReader::new(&Path::new("./testdata/test_art.idx"), &Path::new("./testdata/test_art.mul")).ok().expect("Couldn't load test_art.mul");
}
