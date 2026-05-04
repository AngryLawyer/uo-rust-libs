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

mod tiledata {
    use std::path;
    use std::io;
    use std::result;
    use tiledata;
    use std::uint;
    use std::option;
    #[test]
    fn test_read_map_tile_data() {
        match ::tiledata::TileDataReader::new(&path::Path(~"files/tiledata.mul")) {
            result::Err(msg) => {
                fail!(msg);
            },
            result::Ok(tile_data_reader) => {
                let mut out = ~[];
                for uint::range(0, 128) |idx| {
                    match tile_data_reader.read_map_tile_data(idx) {
                        option::Some(tile_data) => {
                            out.push(tile_data.name)
                        },
                        option::None => {
                            fail!(fmt!("Couldn't read tile %u", idx))
                        }
                    };
                }
                /*io::print(~"TILE DATA: (");
                let mut first = false;
                for out.each |name| {
                    if !first {
                        first = true
                    } else {
                        io::print(~",")
                    }
                    io::print(*name);
                }
                io::println(")");*/
            }
        }
    }

    #[test]
    fn test_read_map_static_data() {
        match ::tiledata::TileDataReader::new(&path::Path(~"files/tiledata.mul")) {
            result::Err(msg) => {
                fail!(msg);
            },
            result::Ok(tile_data_reader) => {
                let mut out = ~[];
                for uint::range(0, 512) |idx| {
                    match tile_data_reader.read_static_tile_data(idx) {
                        option::Some(tile_data) => {
                            if tile_data.flags & ::tiledata::Unknown2Flag as u32 == 0 {
                                out.push(tile_data.name)
                            }
                        },
                        option::None => {
                            fail!(fmt!("Couldn't read tile %u", idx))
                        }
                    };
                }
                io::print(~"STATIC DATA: (");
                let mut first = false;
                for out.iter().advance |name| {
                    if !first {
                        first = true
                    } else {
                        io::print(~",")
                    }
                    io::print(*name);
                }
                io::println(")");
            }
        }
    }
}*/
