//! Methods for reading texture data out of texmaps.mul
//!
//! Texmaps are used when non-flat surfaces need to be drawn
use crate::color::{Color, Color16};
use crate::mul_reader::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
use image::{Rgba, RgbaImage};
use std::fs::File;
use std::io::{Cursor, Read, Result, Seek};
use std::path::Path;

const LARGE_TILE: usize = 0x8000;

pub struct TexMap {
    pub data: Vec<Color16>,
}

impl TexMap {
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

pub struct TexMapsReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

impl TexMapsReader<File> {
    pub fn new(index_path: &Path, mul_path: &Path) -> Result<TexMapsReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(TexMapsReader { mul_reader })
    }
}

impl<T: Read + Seek> TexMapsReader<T> {
    pub fn from_mul(reader: MulReader<T>) -> TexMapsReader<T> {
        TexMapsReader { mul_reader: reader }
    }

    pub fn read(&mut self, id: u32) -> Result<TexMap> {
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
