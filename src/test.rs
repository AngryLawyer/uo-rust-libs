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
            result::Ok(skillReader) => {
                match skillReader.read_skill(0) {
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
                match skillReader.read_skill(10) {
                    option::Some(_skill) => {
                        ()
                    },
                    option::None => {
                        error!("No skill at 10");
                        fail;
                    }
                };
                //There shouldn't be 100 skills
                match skillReader.read_skill(100) {
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
            result::Ok(skillList) => {
                assert skillList.len() >= 49 //49 skills in the original UO
            }
        }
    }
}
