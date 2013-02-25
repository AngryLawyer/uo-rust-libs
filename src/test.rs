mod skills {
    use skills;
    use path;

    #[test]
    fn test_skills() {
        match ::skills::SkillReader(&path::Path(~"files/skills.idx"), &path::Path(~"files/skills.mul")) {
            result::Err(msg) => {
                fail!(msg);
            },
            result::Ok(skill_reader) => {
                match skill_reader.read_skill(0) {
                    option::Some(skill) => {
                        assert skill.name == ~"Alchemy";
                        assert skill.clickable == false;
                    },
                    option::None => {
                        fail!(~"No skill at 0");
                    }
                };
                //There should be a skill at #10
                match skill_reader.read_skill(10) {
                    option::Some(_skill) => {
                        ()
                    },
                    option::None => {
                        fail!(~"No skill at 10");
                    }
                };
                //There shouldn't be 100 skills
                match skill_reader.read_skill(100) {
                    option::Some(_skill) => {
                        fail!(~"Skill at 100");
                    },
                    option::None => ()
                };
            }
        }
    }

    #[test]
    fn test_read_all() {
        match ::skills::load_skills(&path::Path(~"files/skills.idx"), &path::Path(~"files/skills.mul")) {
            result::Err(msg) => {
                fail!(msg);
            },
            result::Ok(skill_list) => {
                assert skill_list.len() >= 49 //49 skills in the original UO
            }
        }
    }
}

mod map {
    use map;
    use path;

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
                                assert(Statics.len() > 0);
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

mod art {
    use art;
    use art::Tile;
    use path;

    #[test]
    fn test_read_tile() {
        match ::art::TileReader(&path::Path(~"files/artidx.mul"), &path::Path(~"files/art.mul")) {
            result::Err(msg) => {
                fail!(msg);
            },
            result::Ok(tileReader) => {
                match tileReader.read_tile(100) {
                    option::Some(tile) => {
                        assert tile.raw_image.len() == 1012; 
                        let bitmap = tile.with_transparency(0xF000);
                        assert bitmap.len() == (44*44);
                        assert bitmap[0] == 0xF000;
                        assert bitmap[21] != 0xF000;
                    },
                    option::None => {
                        fail!(~"Couldn't read tile 100")
                    }
                };
                match tileReader.read_tile(0x4000) {
                    option::Some(_tile) => {
                        fail!(~"Read static 0x4000 as tile")
                    },
                    option::None => ()
                };
            }
        }
    }

    #[test]
    fn test_read_static() {
        match ::art::TileReader(&path::Path(~"files/artidx.mul"), &path::Path(~"files/art.mul")) {
            result::Err(msg) => {
                fail!(msg);
            },
            result::Ok(tile_reader) => {
                match tile_reader.read_static(0x4000) {
                    option::Some(static_tile) => {
                        let bitmap = static_tile.with_transparency(0xF000);
                        //warn!("%u, %u, %u", bitmap.len(), static_tile.width as uint, static_tile.height as uint);
                        //assert bitmap.len() == (static_tile.width + static_tile.height) as uint;
                    },
                    option::None => {
                        fail!(~"Couldn't read tile 0x4000");
                    }
                };
            }
        }
    }
}

mod tiledata {
    use path;
    use tiledata;
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
                            out.push(copy tile_data.name)
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
                for uint::range(0, 512 * 32) |idx| {
                    match tile_data_reader.read_static_tile_data(idx) {
                        option::Some(tile_data) => {
                            if tile_data.flags & ::tiledata::Unknown2Flag as u32 == 0 {
                                out.push(copy tile_data.name)
                            }
                        },
                        option::None => {
                            fail!(fmt!("Couldn't read tile %u", idx))
                        }
                    };
                }
                /*io::print(~"STATIC DATA: (");
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
}
