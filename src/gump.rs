//! Methods for reading data out of gumpart.mul and gumpidx.mul
//!
//! Gumps represent GUI elements, and actually use the opt fields in the index mul:
//! `opt1` representing width, and `opt2` representing height
//!
//! The gump itself is stored as such:
//! `|offsets:[u16..height]|rows:[row..height]|`
//!
//! The offsets are also used to calculate the length of a given row
//!
//! A row is defined as a number of RLE pairs:
//!
//! `|color:Color16|count:u16|`
use crate::color::Color16;
#[cfg(feature = "image")]
use crate::color::{BLACK_16, Color};
#[cfg(feature = "image")]
use crate::error::ToImageError;
use crate::error::{MulReaderError, MulReaderResult};
use crate::mul::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "image")]
use image::error::{DecodingError, ImageFormatHint};
#[cfg(feature = "image")]
use image::{ImageError, Rgba, RgbaImage};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

/// An RLE pair
#[derive(Clone, Copy)]
pub struct GumpPair {
    pub color: Color16,
    pub count: u16,
}

/// A user-interface element.
#[derive(Clone)]
pub struct Gump {
    pub width: u16,
    pub height: u16,
    /// Rows of RLE pairs
    pub data: Vec<Vec<GumpPair>>,
}

#[cfg(feature = "image")]
impl Gump {
    /// Convert this asset into a standarized image format
    pub fn to_image(&self) -> Result<RgbaImage, ImageError> {
        let mut buffer = RgbaImage::new(self.width as u32, self.height as u32);
        for (y, row) in self.data.iter().enumerate() {
            let mut x = 0;
            for run_pair in row {
                // Check for overflow
                if x + run_pair.count > self.width {
                    return Err(ImageError::Decoding(DecodingError::new(
                        ImageFormatHint::Name("UO Gump".to_string()),
                        ToImageError::PixelOutOfBounds {
                            x: (x + run_pair.count) as i64,
                            y: y as i64,
                        },
                    )));
                }

                // Pure black is Transparent in Gumps
                if run_pair.color != BLACK_16 {
                    let (r, g, b, a) = run_pair.color.to_rgba();
                    for i in 0..run_pair.count {
                        buffer.put_pixel(x as u32 + i as u32, y as u32, Rgba([r, g, b, a]));
                    }
                }
                x += run_pair.count;
            }
        }
        Ok(buffer)
    }
}

/// A struct to help read out Gump data
pub struct GumpReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

impl GumpReader<File> {
    /// Create a new GumpReader from an index and mul path
    pub fn new(index_path: &Path, mul_path: &Path) -> MulReaderResult<GumpReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(GumpReader { mul_reader })
    }
}

impl<T: Read + Seek> GumpReader<T> {
    /// Create a GumpReader from an existing mul reader
    pub fn from_mul(reader: MulReader<T>) -> GumpReader<T> {
        GumpReader { mul_reader: reader }
    }

    /// Read a single gump element
    pub fn read(&mut self, index: u32) -> MulReaderResult<Gump> {
        let raw = self.mul_reader.read(index)?;
        let mut output = vec![];
        let len = raw.data.len();

        if len % 4 != 0 {
            return Err(MulReaderError::UnexpectedSize {
                found: len as u32,
                expected: (len + (len % 4)) as u32,
            });
        }

        let mut reader = Cursor::new(raw.data);
        let mut row_offsets = vec![];
        // Load all of our offsets. They're measured from the start of the file
        for _i in 0..raw.opt1 {
            row_offsets.push(reader.read_u32::<LittleEndian>()?);
        }

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
