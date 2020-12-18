//! Methods for reading font data out of fonts.mul
//!
//! Fonts are represented in a continuous, unindexed file as groups
//! `|header: u8|characters: [Character..224]`
//!
//! Individual Characters are defined as
//! `|width: u8|height: u8|unknown: u8|pixels: [Color16..width*height]`
//!
use std::fs::{File};
use std::io::{Cursor, Result, SeekFrom, Seek, Read, Write};
use color::{Color16, Color};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::path::Path;
use image::{Rgba, RgbaImage};

#[derive(Clone)]
pub struct Character {
    pub width: u8,
    pub height: u8,
    pub unknown: u8,
    pub data: Vec<Color16>
}

impl Character {
    pub fn to_image(&self) -> RgbaImage {
        let mut buffer = RgbaImage::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.data[(y as usize * self.width as usize) + x as usize];
                let (r, g, b, a) = pixel.to_rgba();
                // Black is transparent
                if r != 0 || g != 0 || b != 0 {
                    buffer.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
                }
            }
        }
        buffer
    }
}


#[derive(Clone)]
pub struct Font {
    pub header: u8,
    pub characters: Vec<Character>
}

pub struct FontReader<T: Read + Seek> {
    data_reader: T
}

impl FontReader<File> {
    pub fn new(font_path: &Path) -> Result<FontReader<File>> {
        let data_reader = File::open(font_path)?;

        Ok(FontReader {
            data_reader: data_reader
        })
    }
}

impl<T: Read + Seek> FontReader<T> {
    /**
     * If we've already got a file-like object, wrap it
     * */
    pub fn from_readable(data_reader: T) -> FontReader<T> {
        FontReader {
            data_reader: data_reader
        }
    }

    pub fn read_fonts(&mut self) -> Result<Vec<Font>> {
        let mut out = vec![];
        loop {
            let header = self.data_reader.read_u8()?;
            // A 0 in the header means it's not a valid font
            if header == 0 {
                break;
            }
            let current = self.data_reader.seek(SeekFrom::Current(-1))?;
            out.push(self.read_font()?);
        }
        Ok(out)
    }

    fn read_font(&mut self) -> Result<Font> {
        let header = self.data_reader.read_u8()?;
        let mut chars = vec![];
        for _ in 0..224 {
            chars.push(self.read_character()?);
        }
        Ok(Font {
            header,
            characters: chars
        })
    }

    fn read_character(&mut self) -> Result<Character> {
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
            data: pixels
        })
    }
}
