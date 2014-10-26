/*extern crate uorustlibs;
use uorustlibs::hues::HueEntry;

#[test]
fn test_load_hues() {
}

#[test]
fn test_serialize() {
    let color_table = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let name = "Hoojama";
    let hue = HueEntry::new(color_table, 1, 2, name.to_string()).serialize();
    assert_eq!(hue.len(), 64 + 2 + 2 + 20);
    assert_eq!(hue.get(0), 0u16);
    assert_eq!(hue.get(63), 0u16);

    assert_eq!(hue.get(64), 1u16);
    assert_eq!(hue.get(65), 0u16);

    assert_eq!(hue.get(66), 1u16);
    assert_eq!(hue.get(67), 0u16);
    
    assert_eq!(hue.slice(68, 88), ['H'as u8, 'o' as u8, 'o' as u8, 'j' as u8, 'a' as u8, 'm' as u8, 'a' as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}*/
