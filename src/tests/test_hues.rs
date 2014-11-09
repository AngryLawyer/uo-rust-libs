use hues::Hue;

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
    let hue = Hue::new(color_table, 1, 2, "Hoojama".to_string()).serialize().unwrap();
    assert_eq!(hue.len(), 64 + 2 + 2 + 20);
    assert_eq!(hue[0], 0);
    assert_eq!(hue[63], 0);

    assert_eq!(hue[64], 1);
    assert_eq!(hue[65], 0);

    assert_eq!(hue[66], 2);
    assert_eq!(hue[67], 0);
    
    assert_eq!(hue.slice_or_fail(&68, &88), vec!['H'as u8, 'o' as u8, 'o' as u8, 'j' as u8, 'a' as u8, 'm' as u8, 'a' as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].as_slice());
}
