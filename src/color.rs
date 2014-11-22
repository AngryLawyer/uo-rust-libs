pub trait Color {
    fn to_rgba(&self) -> (u8, u8, u8, u8);
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self;
}

pub type Color16 = u16;
pub type Color32 = u32;

impl Color for Color16 {
    fn to_rgba(&self) -> (u8, u8, u8, u8) {
        let r = (((*self >> 10) & 0x1F) * 0xFF / 0x1F) as u8;
        let g = (((*self >> 5) & 0x1F) * 0xFF / 0x1F) as u8;
        let b = ((*self & 0x1F) * 0xFF / 0x1F) as u8;
        (r, g, b, 255)
    }

    fn from_rgba(r: u8, g: u8, b: u8, _a: u8) -> Color16 {
        ((r as u16 >> 3) << 10) + ((g as u16 >> 3) << 5) + (b as u16 >> 3)
    }
}

impl Color for Color32 {
    fn to_rgba(&self) -> (u8, u8, u8, u8) {
        let r = ((*self >> 24) & 0xFF) as u8;
        let g = ((*self >> 16) & 0xFF) as u8;
        let b = ((*self >> 8) & 0xFF) as u8;
        let a = (*self & 0xFF) as u8;
        (r, g, b, a)
    }

    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color32 {
        ((r as u32) << 24) + ((g as u32) << 16) + ((b as u32) << 8) + (a as u32)
    }
}
