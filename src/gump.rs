#[cfg(feature = "image")]
use crate::color::Color;
use crate::color::Color16;
use crate::mul_reader::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "image")]
use image::{Rgba, RgbaImage};
use std::fs::File;
use std::io::{Cursor, Read, Result, Seek, SeekFrom};
use std::path::Path;

#[derive(Clone, Copy)]
pub struct GumpPair {
    pub color: Color16,
    pub count: u16,
}

#[derive(Clone)]
pub struct Gump {
    pub width: u16,
    pub height: u16,
    pub data: Vec<Vec<GumpPair>>,
}

#[cfg(feature = "image")]
impl Gump {
    // TODO: This should be a Result as it can overflow
    pub fn to_image(&self) -> RgbaImage {
        let mut buffer = RgbaImage::new(self.width as u32, self.height as u32);
        for (y, row) in self.data.iter().enumerate() {
            let mut x = 0;
            for run_pair in row {
                let (r, g, b, a) = run_pair.color.to_rgba();
                // Pure black is Transparent in Gumps
                if r != 0 || g != 0 || b != 0 {
                    for i in 0..run_pair.count {
                        buffer.put_pixel(x + i as u32, y as u32, Rgba([r, g, b, a]));
                    }
                }
                x += run_pair.count as u32;
            }
        }
        buffer
    }
}

pub struct GumpReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

impl GumpReader<File> {
    pub fn new(index_path: &Path, mul_path: &Path) -> Result<GumpReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(GumpReader { mul_reader })
    }
}

impl<T: Read + Seek> GumpReader<T> {
    pub fn from_mul(reader: MulReader<T>) -> GumpReader<T> {
        GumpReader { mul_reader: reader }
    }

    pub fn read_gump(&mut self, index: u32) -> Result<Gump> {
        let raw = self.mul_reader.read(index)?;
        let mut output = vec![];
        let len = raw.data.len();
        assert!(len % 4 == 0);
        let mut reader = Cursor::new(raw.data);
        let mut row_offsets = vec![];
        // Load all of our offsets
        for _i in 0..raw.opt1 {
            row_offsets.push(reader.read_u32::<LittleEndian>()?);
        }
        // Unsure if the offset is from start of file, or start of data

        // FIXME: The RLE stuff in here and in art should probably be abstracted
        for (row_idx, offset) in row_offsets.iter().enumerate() {
            let row_length = if row_idx == row_offsets.len() - 1 {
                (len / 4) as u32 - offset
            } else {
                let next_row = row_offsets[row_idx + 1];
                next_row - offset
            };
            reader.seek(SeekFrom::Start((*offset as u64) * 4))?;
            let mut row = vec![];
            for _i in 0..row_length {
                let color = reader.read_u16::<LittleEndian>()?;
                let count = reader.read_u16::<LittleEndian>()?;
                row.push(GumpPair { color, count });
            }
            output.push(row);
        }
        Ok(Gump {
            height: raw.opt1,
            width: raw.opt2,
            data: output,
        })
    }
}
