/*

mod map {
    use map;
    use std::path;
    use std::option;
    use std::result;

    #[test]
    fn test_read_map_statics() {
        match ::map::StaticReader(&path::Path(~"files/staidx0.mul"), &path::Path(~"files/statics0.mul")) {
            result::Err(msg) => {
                fail!(msg);
            },
            result::Ok(staticReader) => {
                match staticReader.read_block(0) {
                    option::None => {
                        match staticReader.read_block(200 + (200 * 512)) {
                            option::Some(Statics) => {
                                assert!(Statics.len() > 0);
                            },
                            option::None => {
                                fail!(~"Expected tile at 200x200 but found none");
                            }
                        }
                    },
                    option::Some(_) => {
                        fail!(~"Found unexpected statics at 0");
                    }
                }
            }
        }
    }
}
*/
