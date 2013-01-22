mod skills {
    use skills;
    use path;

    #[test]
    fn test_skills() {
        match ::skills::SkillReader(&path::Path(~"files/skills.idx"), &path::Path(~"files/skills.mul")) {
            result::Err(msg) => {
                error!("%s", msg);
                fail;
            },
            result::Ok(skill_reader) => {
                match skill_reader.read_skill(0) {
                    option::Some(skill) => {
                        assert skill.name == ~"Alchemy";
                        assert skill.clickable == false;
                    },
                    option::None => {
                        error!("No skill at 0");
                        fail;
                    }
                };
                //There should be a skill at #10
                match skill_reader.read_skill(10) {
                    option::Some(_skill) => {
                        ()
                    },
                    option::None => {
                        error!("No skill at 10");
                        fail;
                    }
                };
                //There shouldn't be 100 skills
                match skill_reader.read_skill(100) {
                    option::Some(_skill) => {
                        error!("Skill at 100");
                        fail;
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
                error!("%s", msg);
                fail;
            },
            result::Ok(skill_list) => {
                assert skill_list.len() >= 49 //49 skills in the original UO
            }
        }
    }
}

mod art {
    use art;
    use path;

    #[test]
    fn test_read_tile() {
        match ::art::TileReader(&path::Path(~"files/artidx.mul"), &path::Path(~"files/art.mul")) {
            result::Err(msg) => {
                error!("%s", msg);
                fail;
            },
            result::Ok(tileReader) => {
                match tileReader.read_tile(400) {
                    option::Some(tile) => {
                        assert tile.raw_image.len() == 1012; 
                        let bitmap = tile.with_transparency(0xF000);
                        assert bitmap.len() == (44*44);
                        assert bitmap[0] == 0xF000;
                        assert bitmap[21] != 0xF000;
                    },
                    option::None => {
                        error!("Couldn't read tile 0")
                    }
                };
                match tileReader.read_tile(100) {
                    option::Some(tile) => {
                        assert tile.raw_image.len() == 1012; 
                        let bitmap = tile.with_transparency(0xF000);
                        assert bitmap.len() == (44*44);
                        assert bitmap[0] == 0xF000;
                        assert bitmap[21] != 0xF000;
                    },
                    option::None => {
                        error!("Couldn't read tile 100")
                        fail
                    }
                };
                match tileReader.read_tile(4000) {
                    option::Some(_tile) => {
                        error!("Read static 4000 as tile")
                        fail;
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
                error!("%s", msg);
                fail;
            },
            result::Ok(tile_reader) => {
                match tile_reader.read_static(16384) {
                    option::Some(static_tile) => {
                        let bitmap = static_tile.with_transparency(0xF000);
                        error!("%u, %u, %u", bitmap.len(), static_tile.width as uint, static_tile.height as uint);
                        assert bitmap.len() == (static_tile.width + static_tile.height) as uint;
                    },
                    option::None => {
                        error!("Couldn't read tile 16384");
                        fail
                    }
                };
            }
        }
    }
}
