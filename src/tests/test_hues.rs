use std::io::{Cursor, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use hues::{Hue, HueGroup, HueReader};

#[test]
fn test_load_hues() {
    let color_table = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let hue = Hue::new(color_table, 1, 2, "Hoojama".to_string());
    let group = HueGroup::new(5, [hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone()]);

    let mut serialized: Cursor<Vec<u8>> = Cursor::new(vec![]);
    serialized.write(&group.serialize()).unwrap();

    let hue = Hue::new(color_table, 1, 2, "Llama".to_string());
    let group = HueGroup::new(5, [hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone()]);

    serialized.write(&group.serialize()).unwrap();

    let mut reader = HueReader::from_readable(serialized);

    let first = reader.read_hue_group(0).ok().expect("Couldn't read index 0");
    let second= reader.read_hue_group(1).ok().expect("Couldn't read index 1");
    assert_eq!(first.entries[0].name, "Hoojama".to_string());
    assert_eq!(second.entries[0].name, "Llama".to_string());
}

#[test]
fn test_serialize_hue() {
    let color_table = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let hue = Hue::new(color_table, 1, 2, "Hoojama".to_string()).serialize();
    assert_eq!(hue.len(), 64 + 2 + 2 + 20);
    assert_eq!(hue[0], 0);
    assert_eq!(hue[63], 0);

    assert_eq!(hue[64], 1);
    assert_eq!(hue[65], 0);

    assert_eq!(hue[66], 2);
    assert_eq!(hue[67], 0);

    assert_eq!(&hue[68..88], vec!['H' as u8, 'o' as u8, 'o' as u8, 'j' as u8, 'a' as u8, 'm' as u8, 'a' as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].as_slice());
}

#[test]
fn test_serialize_hue_group() {
    let color_table = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let hue = Hue::new(color_table, 1, 2, "Hoojama".to_string());
    let group = HueGroup::new(5, [hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone(), hue.clone()]);

    let serialized = group.serialize();
    assert_eq!(serialized.len(), ((64 + 2 + 2 + 20) * 8) + 4);
    assert_eq!(serialized[0], 5);
    assert_eq!(serialized[4 + 64], 1);
}
