pub trait Color {
    fn to_tuple(&self) -> (u8, u8, u8);
}

pub type Color16 = u16;

impl Color for Color16 {
    fn to_tuple(&self) -> (u8, u8, u8) {
        let r = (((*self >> 10) & 0x1F) * 0xFF / 0x1F) as u8;
        let g = (((*self >> 5) & 0x1F) * 0xFF / 0x1F) as u8;
        let b = ((*self & 0x1F) * 0xFF / 0x1F) as u8;
        (r, g, b)
    }
}
