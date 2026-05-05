//! Methods for reading animated characters out of anim.mul/anim.idx
//!
//! Animations are stored as a sequence of frames, with offsets.
//! Inside these frames, a sequence rows. Rows store their offsets, allowing for compact
//! representations of the data
//!
//! The underlying raw data for AnimationGroup is defined as
//! `|palette:[u16..256]|frame_count:u32|frame_offsets:[u32..frame_count]|frames:[frame..frame_count]|`
//!
//! Frame offsets are read from the end of the palette to find individual frames
//!
//! The raw frame is defined as
//!
//! `|image_center_x:i16|image_center_y:i16|width:u16|height:u16|rows:[row..?]|`
//!
//! Each row has a header, which either contains offset information and length, or a special stop
//! value of `0x7FFF7FFF`
//!
//! `|header:u32|pixels:[u8..?]|`
//!
//!
#[cfg(feature = "image")]
use crate::color::Color;
use crate::color::Color16;
use crate::errors::MulReaderResult;
#[cfg(feature = "image")]
use crate::errors::ToImageError;
use crate::mul::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "image")]
use image::error::{DecodingError, ImageError, ImageFormatHint};
#[cfg(feature = "image")]
use image::{Delay, Frame, Frames, Rgba, RgbaImage};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;
#[cfg(feature = "image")]
use std::time::Duration;

const PALETTE_SIZE: usize = 256;
const IMAGE_COMPLETE: u32 = 0x7FFF7FFF;

const OFFSET_MASK: i32 = (0x200 << 22) | (0x200 << 12);

/// A single row of a frame
pub struct Row {
    /// Compacted header information.
    /// It contains the length of the associated data, plus offsets of where to plot, relative
    /// to the image center
    ///
    /// `|x_offset..10bits|y_offset..10bits|run_length..12bits|`
    ///
    /// The offsets are signed values
    pub header: u32,
    /// Individual pixels for the row, as lookups in the AnimGroup palette
    pub image_data: Vec<u8>,
}

impl Row {
    /// Get the x offset of where to start drawing this row, relative to a center point
    /// The resulting offset will be from the bottom left
    pub fn x_offset(&self, image_center_x: i16) -> i32 {
        (((self.header as i32 ^ OFFSET_MASK) >> 22) & 0x3FF) + image_center_x as i32 - 0x200
    }

    /// Get the y offset of where to start drawing this row, relative to a center point
    /// The resulting offset will be from the bottom left
    pub fn y_offset(&self, image_center_y: i16, height: u32) -> i32 {
        (((self.header as i32 ^ OFFSET_MASK) >> 12) & 0x3FF) + image_center_y as i32 + height as i32
            - 0x200
    }

    #[cfg(feature = "image")]
    /// Draw a row into an image buffer.
    ///
    /// This is typically called by to_frame, and you won't need to call it directly
    pub fn plot(
        &self,
        image_center_x: i16,
        image_center_y: i16,
        width: u16,
        height: u16,
        palette: &[u16],
        buffer: &mut RgbaImage,
    ) -> Result<(), ImageError> {
        let x = self.x_offset(image_center_x);
        let y = self.y_offset(image_center_y, height as u32);
        if x < 0 || y < 0 || y >= height as i32 || x as u16 + self.image_data.len() as u16 > width {
            return Err(ImageError::Decoding(DecodingError::new(
                ImageFormatHint::Name("UO AnimFrame".to_string()),
                ToImageError::PixelOutOfBounds {
                    x: x as i64,
                    y: y as i64,
                },
            )));
        }
        for i in 0..self.image_data.len() {
            let target_x = x + i as i32;
            if target_x > width as i32 {
                return Err(ImageError::Decoding(DecodingError::new(
                    ImageFormatHint::Name("UO AnimFrame".to_string()),
                    ToImageError::PixelOutOfBounds {
                        x: target_x as i64,
                        y: y as i64,
                    },
                )));
            }
            let (r, g, b, a) = palette[self.image_data[i] as usize].to_rgba();
            buffer.put_pixel(target_x as u32, y as u32, Rgba([r, g, b, a]));
        }
        Ok(())
    }
}

/// A frame of an animtion
pub struct AnimFrame {
    pub image_center_x: i16,
    pub image_center_y: i16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<Row>,
}

#[cfg(feature = "image")]
impl AnimFrame {
    /// Convert an individual frame to an Image frame
    pub fn to_frame(&self, palette: &[u16]) -> Result<Frame, ImageError> {
        if self.width == 0 || self.height == 0 {
            return Err(ImageError::Decoding(DecodingError::from_format_hint(
                ImageFormatHint::Name("UO AnimFrame".to_string()),
            )));
        }
        let mut buffer = RgbaImage::new(self.width as u32, self.height as u32);
        for row in &self.data {
            row.plot(
                self.image_center_x,
                self.image_center_y,
                self.width,
                self.height,
                palette,
                &mut buffer,
            )?;
        }
        Ok(Frame::from_parts(
            buffer,
            0,
            0,
            Delay::from_saturating_duration(Duration::from_millis(0)),
        ))
    }
}

/// An animation sequence
pub struct AnimGroup {
    pub palette: [Color16; 256],
    pub frame_count: u32,
    pub frames: Vec<AnimFrame>,
}

impl AnimGroup {
    #[cfg(feature = "image")]
    /// Convert an AnimGroup into Image-based frames.
    ///
    /// Currently, this doesn't produce values around delays
    pub fn to_frames(&self) -> Frames<'_> {
        Frames::new(Box::new(
            self.frames
                .iter()
                .map(move |anim_frame| anim_frame.to_frame(&self.palette)),
        ))
    }
}

/// A struct to allow reading of animations from data muls
pub struct AnimReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

fn read_frame<T: Read + Seek>(reader: &mut T) -> MulReaderResult<AnimFrame> {
    let image_center_x = reader.read_i16::<LittleEndian>()?;
    let image_center_y = reader.read_i16::<LittleEndian>()?;
    let width = reader.read_u16::<LittleEndian>()?;
    let height = reader.read_u16::<LittleEndian>()?;

    let mut data = vec![];
    loop {
        let header = reader.read_u32::<LittleEndian>()?;
        if header == IMAGE_COMPLETE {
            break;
        }
        let run_length = header & 0xFFF;
        let mut image_data = vec![];
        for _i in 0..run_length {
            image_data.push(reader.read_u8()?);
        }
        data.push(Row { header, image_data });
    }

    // Read data
    Ok(AnimFrame {
        image_center_x,
        image_center_y,
        width,
        height,
        data,
    })
}

impl AnimReader<File> {
    /// Create an animation reader from paths to an index mul and a data mul
    pub fn new(index_path: &Path, mul_path: &Path) -> MulReaderResult<AnimReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(AnimReader { mul_reader })
    }
}

impl<T: Read + Seek> AnimReader<T> {
    /// Create an animation reader from an existing Mul
    pub fn from_mul(reader: MulReader<T>) -> AnimReader<T> {
        AnimReader { mul_reader: reader }
    }

    /// Read an animation group by id
    pub fn read(&mut self, id: u32) -> MulReaderResult<AnimGroup> {
        let raw = self.mul_reader.read(id)?;
        let mut reader = Cursor::new(raw.data);
        // Read the palette
        let mut palette = [0; PALETTE_SIZE];
        for cell in &mut palette {
            *cell = reader.read_u16::<LittleEndian>()?;
        }

        let frame_count = reader.read_u32::<LittleEndian>()?;
        let mut frame_offsets = vec![];
        for _ in 0..frame_count {
            frame_offsets.push(reader.read_u32::<LittleEndian>()?);
        }

        let mut frames = vec![];
        for offset in frame_offsets {
            reader.seek(SeekFrom::Start((PALETTE_SIZE as u32 * 2 + offset) as u64))?;
            frames.push(read_frame(&mut reader)?);
        }

        Ok(AnimGroup {
            palette,
            frame_count,
            frames,
        })
    }
}
