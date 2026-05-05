//! Methods for reading texture data out of texmaps.mul
//!
//! Texmaps are used when non-flat surfaces need to be drawn
//!
//! Texmaps exist as a sequence of colours, and are either 128x128, or 64x64
#[cfg(feature = "image")]
use crate::color::Color;
use crate::color::Color16;
use crate::error::MulReaderResult;
use crate::mul::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "image")]
use image::{Rgba, RgbaImage};
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::Path;

pub const LARGE_TILE: usize = 0x8000;

/// A texture. They are always either 128x128 or 64x64
pub struct TexMap {
    pub data: Vec<Color16>,
}

#[cfg(feature = "image")]
impl TexMap {
    /// Convert this asset into a standarized image format
    pub fn to_image(&self) -> RgbaImage {
        let tile_width = if self.data.len() * 2 >= LARGE_TILE {
            128
        } else {
            64
        };

        let mut buffer = RgbaImage::new(tile_width, tile_width);
        for (idx, pixel) in self.data.iter().enumerate() {
            let x = idx as u32 % tile_width;
            let y = idx as u32 / tile_width;
            let (r, g, b, a) = pixel.to_rgba();
            buffer.put_pixel(x, y, Rgba([r, g, b, a]));
        }
        buffer
    }
}

/// A struct to help read out TexMap data
pub struct TexMapReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

impl TexMapReader<File> {
    /// Create a new TexMapReader from an index and mul path
    pub fn new(index_path: &Path, mul_path: &Path) -> MulReaderResult<TexMapReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(TexMapReader { mul_reader })
    }
}

impl<T: Read + Seek> TexMapReader<T> {
    /// Create a TexMapReader from an existing mul reader
    pub fn from_mul(reader: MulReader<T>) -> TexMapReader<T> {
        TexMapReader { mul_reader: reader }
    }

    /// Read a single texmap
    pub fn read(&mut self, id: u32) -> MulReaderResult<TexMap> {
        let raw = self.mul_reader.read(id)?;
        let len = raw.data.len();
        let mut reader = Cursor::new(raw.data);
        let mut data = vec![];
        for _idx in 0..len / 2 {
            data.push(reader.read_u16::<LittleEndian>()?);
        }
        Ok(TexMap { data })
    }
}
