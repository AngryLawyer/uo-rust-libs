use crate::mul_reader::simple_from_vecs;
use crate::skills::{Skill, Skills};

#[test]
fn test_load_skills() {
    let mut mul_reader = simple_from_vecs(
        vec![
            vec![1, b'S', b'a', b'n', b'd', b'w', b'i', b'c', b'h', 0],
            vec![0, b'B', b'u', b'r', b'g', b'e', b'r', 0],
        ],
        0,
        0,
    );

    let skills = Skills::from_mul(&mut mul_reader).unwrap();
    assert_eq!(skills.skills.len(), 2);
    let skill = &skills.skills[0];
    assert!(skill.clickable);
    assert_eq!(&skill.name, "Sandwich");
    let skill = &skills.skills[1];
    assert!(!skill.clickable);
    assert_eq!(&skill.name, "Burger");
}

#[test]
fn test_serialize() {
    let in_string = "Sandwich";
    let skill = Skill::new(true, in_string.to_string()).serialize();
    assert_eq!(skill[0], 1u8);
    assert_eq!(skill[1], b'S');
    assert_eq!(skill.len(), in_string.len() + 2) //One for the clickable prefix, one for string terminal
}
