//! Methods for reading tile and static data out of art.mul
//!
//! Tiles and Statics are both traditionally stored in art.mul/artidx.mul, and are packed into the
//! same files - all entried before 0x4000 are tiles, and the rest statics.
//!
//! Map tiles are stored as
//! `|header:u32|pixels:[u16..1022]|`
//! Where pixels represents a list of rows, length 2, 4, 6, 8 .. 42, 44, 44, 42 .. 8, 6, 4, 2
//!
//! Statics are stored as
//! `|size:u16|trigger:u16|width:u16|height:u16|offset_table:[u16..height]|rows:[row..height]`
//!
//! Rows are stored as
//! `|runs:[run_pair..?]|`
//! and are read until a stop value is found
//!
//! Run pairs are stored as
//! `|x_offset:u16|run_length:u16|pixels:[Color16..run_length]|`
//! where the x_offset defines how many transparent pixels should be left before drawing this run.
//!
//! A run pair with an offset and length of 0 denotes that the row is complete.
#[cfg(feature = "image")]
use crate::color::Color;
use crate::color::Color16;
use crate::error::{MEMWRITER_ERROR, MulReaderError, MulReaderResult};
use crate::mul::MulReader;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

#[cfg(feature = "image")]
use image::{Rgba, RgbaImage};

/// A shared trait for dealing with both static and tile data
pub trait Art {
    /// Convert this asset back to the raw, storable form
    fn serialize(&self) -> Vec<u8>;

    /// Convert this asset into a standarized image format
    #[cfg(feature = "image")]
    fn to_image(&self) -> RgbaImage;
}

pub const TILE_SIZE: u32 = 2048;
pub const STATIC_OFFSET: u32 = 0x4000;

/// A run pair contains an offset at which point to start drawing the run,
/// and the pixels to draw
pub struct RunPair {
    pub offset: u16,
    pub run: Vec<Color16>,
}

impl RunPair {
    fn serialize(&self) -> Vec<u8> {
        let mut writer = vec![];

        writer
            .write_u16::<LittleEndian>(self.offset)
            .expect(MEMWRITER_ERROR);
        writer
            .write_u16::<LittleEndian>(self.run.len() as u16)
            .expect(MEMWRITER_ERROR);
        for &color in self.run.iter() {
            writer
                .write_u16::<LittleEndian>(color)
                .expect(MEMWRITER_ERROR);
        }
        writer
    }
}

pub type StaticRow = Vec<RunPair>;

/// A map tile, 44px by 44px.
pub struct Tile {
    /// Header information for a tile.  Unused
    pub header: u32,
    /// Individual pixels. These work as rows, of length 2, 4, 6, 8 .. 42, 44, 44, 42 .. 8, 6, 4, 2
    /// These should be drawn centered to the 44px-wide tile
    pub image_data: [Color16; 1022],
}

impl Art for Tile {
    fn serialize(&self) -> Vec<u8> {
        let mut writer = vec![];
        writer
            .write_u32::<LittleEndian>(self.header)
            .expect(MEMWRITER_ERROR);
        for &pixel in self.image_data.iter() {
            writer
                .write_u16::<LittleEndian>(pixel)
                .expect(MEMWRITER_ERROR);
        }
        writer
    }

    #[cfg(feature = "image")]
    fn to_image(&self) -> RgbaImage {
        let mut buffer = RgbaImage::new(44, 44);
        let mut read_idx = 0;

        for y in 0..44 {
            let slice_size = if y >= 22 { (44 - y) * 2 } else { (y + 1) * 2 };

            let indent = 22 - (slice_size / 2);

            for x in 0..slice_size {
                let (r, g, b, a) = self.image_data[read_idx].to_rgba();
                buffer.put_pixel(indent + x, y, Rgba([r, g, b, a]));
                read_idx += 1;
            }
        }
        buffer
    }
}

impl Art for Static {
    fn serialize(&self) -> Vec<u8> {
        let mut writer = vec![];
        writer
            .write_u16::<LittleEndian>(self.size)
            .expect(MEMWRITER_ERROR);
        writer
            .write_u16::<LittleEndian>(self.trigger)
            .expect(MEMWRITER_ERROR);
        writer
            .write_u16::<LittleEndian>(self.width)
            .expect(MEMWRITER_ERROR);
        writer
            .write_u16::<LittleEndian>(self.height)
            .expect(MEMWRITER_ERROR);

        let mut rows = vec![];

        //Write our rows
        for row in self.rows.iter() {
            let mut out = vec![];
            for pair in row.iter() {
                out.write_all(pair.serialize().as_slice())
                    .expect(MEMWRITER_ERROR);
            }
            //We write a "newline" after each out
            out.write_u16::<LittleEndian>(0).expect(MEMWRITER_ERROR);
            out.write_u16::<LittleEndian>(0).expect(MEMWRITER_ERROR);
            rows.push(out);
        }

        let mut lookup_table = vec![];
        let mut last_position = 0;
        //Generate a lookup table
        for row in rows.iter() {
            lookup_table
                .write_u16::<LittleEndian>(last_position)
                .expect(MEMWRITER_ERROR);
            last_position += (row.len() / 2) as u16;
        }
        writer
            .write_all(lookup_table.as_slice())
            .expect(MEMWRITER_ERROR);
        for row in rows.iter() {
            writer.write_all(row.as_slice()).expect(MEMWRITER_ERROR);
        }

        writer
    }

    #[cfg(feature = "image")]
    fn to_image(&self) -> RgbaImage {
        let mut buffer = RgbaImage::new(self.width as u32, self.height as u32);
        for (y, row) in self.rows.iter().enumerate() {
            let mut x: u32 = 0;
            for run_pair in row.iter() {
                x += run_pair.offset as u32;
                for pixel in run_pair.run.iter() {
                    let (r, g, b, a) = pixel.to_rgba();
                    buffer.put_pixel(x, y as u32, Rgba([r, g, b, a]));
                    x += 1;
                }
            }
        }
        buffer
    }
}

/// A static image, typically used to render props
pub struct Static {
    /// The number of bytes this static is stored as
    pub size: u16,
    /// Trigger information. Yet unknown what this represents
    pub trigger: u16,
    /// The width of the image
    pub width: u16,
    /// The height of the image
    pub height: u16,
    /// The image data
    pub rows: Vec<StaticRow>,
}

/// A struct to help read out Tile and Static data
pub struct ArtReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

impl ArtReader<File> {
    /// Create a new ArtReader from an index and mul path
    pub fn new(index_path: &Path, mul_path: &Path) -> MulReaderResult<ArtReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(ArtReader { mul_reader })
    }
}

impl<T: Read + Seek> ArtReader<T> {
    /// Create an ArtReader from an existing mul reader
    pub fn from_mul(reader: MulReader<T>) -> ArtReader<T> {
        ArtReader { mul_reader: reader }
    }

    /// Read a single tile
    pub fn read_tile(&mut self, id: u32) -> MulReaderResult<Tile> {
        if id >= STATIC_OFFSET {
            return Err(MulReaderError::IndexOutOfBounds(id));
        }

        let raw = self.mul_reader.read(id)?;
        let mut reader = Cursor::new(raw.data);

        if raw.length > TILE_SIZE {
            return Err(MulReaderError::UnexpectedSize {
                found: raw.length,
                expected: TILE_SIZE,
            });
        }

        let header = reader.read_u32::<LittleEndian>()?;
        let mut body = [0; 1022];
        for cell in &mut body {
            *cell = reader.read_u16::<LittleEndian>().unwrap_or(0);
        }
        Ok(Tile {
            header,
            image_data: body,
        })
    }

    /// Read a single static.
    ///
    /// Statics are read with an offset, so 0 is the first static in the file.
    pub fn read_static(&mut self, id: u32) -> MulReaderResult<Static> {
        let offset_id = id + STATIC_OFFSET;

        let raw = self.mul_reader.read(offset_id)?;
        let mut reader = Cursor::new(raw.data);

        let size = reader.read_u16::<LittleEndian>()?;
        let trigger = reader.read_u16::<LittleEndian>()?;
        let width = reader.read_u16::<LittleEndian>()?;
        let height = reader.read_u16::<LittleEndian>()?;

        if width == 0 || width >= 1024 || height == 0 || height >= 1024 {
            return Err(MulReaderError::FailedParse(format!(
                "Got invalid width and height of {}, {}",
                width, height
            )));
        }

        //Load our offset table
        let mut offset_table = vec![];
        for _index in 0..height {
            offset_table.push(reader.read_u16::<LittleEndian>()?);
        }

        let data_start_pos = reader.position();
        let mut rows = vec![];

        for &offset in offset_table.iter() {
            reader.seek(SeekFrom::Start(data_start_pos + offset as u64 * 2))?;
            let mut row = vec![];

            loop {
                let x_offset = reader.read_u16::<LittleEndian>()?;
                let run_length = reader.read_u16::<LittleEndian>()?;
                if x_offset + run_length == 0 {
                    break;
                } else {
                    let mut run = vec![];
                    for _index in 0..run_length {
                        run.push(reader.read_u16::<LittleEndian>()?);
                    }

                    row.push(RunPair {
                        offset: x_offset,
                        run,
                    });
                }
            }
            rows.push(row);
        }

        Ok(Static {
            size,
            trigger,
            width,
            height,
            rows,
        })
    }
}
