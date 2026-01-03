//! Methods for reading animated characters out of anim.mul/anim.idx
#[cfg(feature = "image")]
use crate::color::Color;
use crate::color::Color16;
use crate::mul_reader::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "image")]
use image::error::{DecodingError, ImageError, ImageFormatHint};
#[cfg(feature = "image")]
use image::{Delay, Frame, Frames, Rgba, RgbaImage};
use std::fs::File;
use std::io::{Cursor, Read, Result, Seek, SeekFrom};
use std::path::Path;
#[cfg(feature = "image")]
use std::time::Duration;

const PALETTE_SIZE: usize = 256;
const IMAGE_COMPLETE: u32 = 0x7FFF7FFF;

const OFFSET_MASK: i32 = (0x200 << 22) | (0x200 << 12);

pub struct Row {
    pub header: u32,
    pub image_data: Vec<u8>,
}

impl Row {
    pub fn x_offset(&self, image_centre_x: i16) -> i32 {
        (((self.header as i32 ^ OFFSET_MASK) >> 22) & 0x3FF) + image_centre_x as i32 - 0x200
    }

    pub fn y_offset(&self, image_centre_y: i16, height: u32) -> i32 {
        (((self.header as i32 ^ OFFSET_MASK) >> 12) & 0x3FF) + image_centre_y as i32 + height as i32
            - 0x200
    }
}

pub struct AnimFrame {
    pub image_centre_x: i16,
    pub image_centre_y: i16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<Row>,
}

pub struct AnimGroup {
    pub palette: [Color16; 256],
    pub frame_count: u32,
    pub frames: Vec<AnimFrame>,
}

impl AnimGroup {
    #[cfg(feature = "image")]
    pub fn to_frames(&self) -> Frames<'_> {
        Frames::new(Box::new(self.frames.iter().map(move |anim_frame| {
            if anim_frame.width == 0 || anim_frame.height == 0 {
                return Err(ImageError::Decoding(DecodingError::from_format_hint(
                    ImageFormatHint::Name("UO AnimFrame".to_string()),
                )));
            }
            // TODO: Figure out what to do with image_centre_x and y, and sort out offsets
            let mut buffer = RgbaImage::new(anim_frame.width as u32, anim_frame.height as u32);
            for row in &anim_frame.data {
                let x = row.x_offset(anim_frame.image_centre_x);
                let y = row.y_offset(anim_frame.image_centre_y, anim_frame.height as u32);
                if (x < 0 || y < 0) && (anim_frame.width == 0 || anim_frame.height == 0) {
                    return Err(ImageError::Decoding(DecodingError::from_format_hint(
                        ImageFormatHint::Name("UO AnimFrame".to_string()),
                    )));
                }
                for i in 0..row.image_data.len() {
                    if (x + i as i32 > anim_frame.width as i32 || y > anim_frame.height as i32)
                        && (anim_frame.width == 0 || anim_frame.height == 0)
                    {
                        return Err(ImageError::Decoding(DecodingError::from_format_hint(
                            ImageFormatHint::Name("UO AnimFrame".to_string()),
                        )));
                    }
                    let (r, g, b, a) = self.palette[row.image_data[i] as usize].to_rgba();
                    buffer.put_pixel(x as u32 + i as u32, y as u32, Rgba([r, g, b, a]));
                }
            }
            Ok(Frame::from_parts(
                buffer,
                0,
                0,
                Delay::from_saturating_duration(Duration::from_millis(0)),
            ))
        })))
    }
}

pub struct AnimReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

fn read_frame<T: Read + Seek>(reader: &mut T) -> Result<AnimFrame> {
    let image_centre_x = reader.read_i16::<LittleEndian>()?;
    let image_centre_y = reader.read_i16::<LittleEndian>()?;
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
        image_centre_x,
        image_centre_y,
        width,
        height,
        data,
    })
}

impl AnimReader<File> {
    pub fn new(index_path: &Path, mul_path: &Path) -> Result<AnimReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(AnimReader { mul_reader })
    }
}

impl<T: Read + Seek> AnimReader<T> {
    pub fn from_mul(reader: MulReader<T>) -> AnimReader<T> {
        AnimReader { mul_reader: reader }
    }

    pub fn read(&mut self, id: u32) -> Result<AnimGroup> {
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
