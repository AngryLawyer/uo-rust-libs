use color::{Color, Color16, Color32};

#[test]
fn test_color32_to_rgba() {
    let colors: Vec<(Color32, (u8, u8, u8, u8))> = vec![
        (0xffffffff, (0xff, 0xff, 0xff, 0xff)),
        (0xff0000ff, (0xff, 0x00, 0x00, 0xff)),
        (0x00ff00ff, (0x00, 0xff, 0x00, 0xff)),
        (0xf6ff44ff, (0xf6, 0xff, 0x44, 0xff)),
    ];

    for &(color, (expected_r, expected_g, expected_b, expected_a)) in colors.iter() {
        let (r, g, b, a) = color.to_rgba();
        assert_eq!(r, expected_r);
        assert_eq!(g, expected_g);
        assert_eq!(b, expected_b);
        assert_eq!(a, expected_a);
    }

}

#[test]
fn test_color32_from_rgba() {
    let colors: Vec<(Color32, (u8, u8, u8, u8))> = vec![
        (0xffffffff, (0xff, 0xff, 0xff, 0xff)),
        (0xff0000ff, (0xff, 0x00, 0x00, 0xff)),
        (0x00ff00ff, (0x00, 0xff, 0x00, 0xff)),
        (0xf6ff44ff, (0xf6, 0xff, 0x44, 0xff)),
    ];

    for &(expected_color, (r, g, b, a)) in colors.iter() {
        let color: Color32 = Color::from_rgba(r, g, b, a);
        assert_eq!(color, expected_color);
    }
}

#[test]
fn test_color16_to_rgba() {
    let colors: Vec<(Color16, (u8, u8, u8, u8))> = vec![
        (0x7fff, (0xff, 0xff, 0xff, 0xff)),
        (0x7c00, (0xff, 0x00, 0x00, 0xff)),
        (0x03e0, (0x00, 0xff, 0x00, 0xff)),
    ];

    for &(color, (expected_r, expected_g, expected_b, expected_a)) in colors.iter() {
        let (r, g, b, a) = color.to_rgba();
        assert_eq!(r, expected_r);
        assert_eq!(g, expected_g);
        assert_eq!(b, expected_b);
        assert_eq!(a, expected_a);
    }
}

#[test]
fn test_color16_from_rgba() {
    let colors: Vec<(Color16, (u8, u8, u8, u8))> = vec![
        (0x7fff, (0xff, 0xff, 0xff, 0xff)),
        (0x7c00, (0xff, 0x00, 0x00, 0xff)),
        (0x03e0, (0x00, 0xff, 0x00, 0xff)),
    ];

    for &(expected_color, (r, g, b, a)) in colors.iter() {
        let color: Color16 = Color::from_rgba(r, g, b, a);
        assert_eq!(color, expected_color);
    }
}
