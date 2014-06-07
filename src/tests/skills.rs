use skills::Skills;

#[test]
fn test_load_skills() {
    match Skills::new(&Path::new("./src/tests/testdata/test_skills.idx"), &Path::new("./src/tests/testdata/test_skills.mul")) {
        Ok(skills) => {
        },
        Err(message) => fail!(message)
    }
}
