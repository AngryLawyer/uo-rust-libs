//! Methods for reading font data out of fonts.mul
//!
//! There are 10 fonts in a file.
//!
//! Fonts are represented in a continuous, unindexed file as groups
//! `|header: u8|characters: [Character..224]`
//!
//! Individual Characters are defined as
//! `|width: u8|height: u8|unknown: u8|pixels: [Color16..width*height]`
//!
#[cfg(feature = "image")]
use crate::color::Color;
use crate::color::{BLACK_16, Color16};
use crate::errors::MulReaderResult;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "image")]
use image::{Rgba, RgbaImage};
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

/// An individual glyph in a font
#[derive(Clone)]
pub struct Character {
    pub width: u8,
    pub height: u8,
    pub unknown: u8,
    pub data: Vec<Color16>,
}

#[cfg(feature = "image")]
impl Character {
    pub fn to_image(&self) -> RgbaImage {
        let mut buffer = RgbaImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.data[(y as usize * self.width as usize) + x as usize];
                // Black is transparent here
                if pixel != BLACK_16 {
                    let (r, g, b, a) = pixel.to_rgba();
                    buffer.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
                }
            }
        }
        buffer
    }
}

/// A font. Fonts should always have 224 characters in them.
/// They map to ASCII, but skip the first 32 characters
#[derive(Clone)]
pub struct Font {
    pub header: u8,
    pub characters: Vec<Character>,
}

/// A struct to help reading fonts from a
pub struct FontReader<T: Read + Seek> {
    data_reader: T,
}

impl FontReader<File> {
    /// Create a new FontReader from a mul path
    pub fn new(font_path: &Path) -> MulReaderResult<FontReader<File>> {
        let data_reader = File::open(font_path)?;

        Ok(FontReader { data_reader })
    }
}

impl<T: Read + Seek> FontReader<T> {
    /// Create a FontReader from an existing readable object
    pub fn from_readable(data_reader: T) -> FontReader<T> {
        FontReader { data_reader }
    }

    /// Read all 10 fonts from the file.
    /// As fonts are variable-length (due to differing character sizes),
    /// it's not easy to read them individually
    pub fn read_fonts(&mut self) -> MulReaderResult<Vec<Font>> {
        let mut out = vec![];
        for _ in 0..10 {
            out.push(self.read_font()?);
        }
        Ok(out)
    }

    fn read_font(&mut self) -> MulReaderResult<Font> {
        let header = self.data_reader.read_u8()?;
        let mut chars = vec![];
        for _ in 0..224 {
            chars.push(self.read_character()?);
        }
        Ok(Font {
            header,
            characters: chars,
        })
    }

    fn read_character(&mut self) -> MulReaderResult<Character> {
        let width = self.data_reader.read_u8()?;
        let height = self.data_reader.read_u8()?;
        let unknown = self.data_reader.read_u8()?;
        let mut pixels = vec![];
        for _ in 0..(width as u32 * height as u32) {
            pixels.push(self.data_reader.read_u16::<LittleEndian>()?);
        }
        Ok(Character {
            width,
            height,
            unknown,
            data: pixels,
        })
    }
}
