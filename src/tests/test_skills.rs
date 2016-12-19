use std::io::Cursor;

use byteorder::{LittleEndian, WriteBytesExt};

use mul_reader::{simple_from_vecs};
use skills::{Skills, Skill};

#[test]
fn test_load_skills() {
    let mut mul_reader = simple_from_vecs(vec![
        vec![1, 'S' as u8, 'a' as u8, 'n' as u8, 'd' as u8, 'w' as u8, 'i' as u8, 'c' as u8, 'h' as u8, 0],
        vec![0, 'B' as u8, 'u' as u8, 'r' as u8, 'g' as u8, 'e' as u8, 'r' as u8, 0],
    ]);

    let skills = Skills::from_mul(&mut mul_reader);
    assert_eq!(skills.skills.len(), 2);
    let ref skill = skills.skills[0];
    assert_eq!(skill.clickable, true);
    assert_eq!(&skill.name, "Sandwich");
    let ref skill = skills.skills[1];
    assert_eq!(skill.clickable, false);
    assert_eq!(&skill.name, "Burger");
}

#[test]
fn test_serialize() {
    let in_string = "Sandwich";
    let skill = Skill::new(true, in_string.to_string()).serialize();
    assert_eq!(skill[0], 1u8);
    assert_eq!(skill[1], 'S' as u8);
    assert_eq!(skill.len(), in_string.len() + 2) //One for the clickable prefix, one for string terminal
}
