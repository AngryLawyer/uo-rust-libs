//! Methods for reading tile and static data out of art.mul
//!
//! Tiles and Statics are both traditionally stored in art.mul/artidx.mul
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::color::{Color, Color16};
use crate::mul_reader::MulReader;
use std::fs::File;
use std::io::{Cursor, Error, Read, Result, Seek, SeekFrom, Write};
use std::path::Path;
use crate::utils::MEMWRITER_ERROR;

use image::{Rgba, RgbaImage};

pub trait Art {
    /**
     * Convert to a 32bit array
     *
     * Returns (width, height, colors)
     */
    fn serialize(&self) -> Vec<u8>;

    fn to_image(&self) -> RgbaImage;
}

pub const TILE_SIZE: u32 = 2048;
pub const STATIC_OFFSET: u32 = 0x4000;

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

/// A Map Tile, 44px by 44px
///
/// Map tiles are stored as
/// |Header:u32|Pixels:u16|
/// Where pixels represents a list of rows, length 2, 4, 6, 8 .. 42, 44, 44, 42 .. 8, 6, 4, 2 and are drawn
/// centred
pub struct Tile {
    /// Header information for a tile.  Unused
    pub header: u32,
    /// Individual pixels. These work as rows, of length 2, 4, 6, 8 .. 42, 44, 44, 42 .. 8, 6, 4, 2
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
            out.write_u16::<LittleEndian>(0)
                .expect(MEMWRITER_ERROR);
            out.write_u16::<LittleEndian>(0)
                .expect(MEMWRITER_ERROR);
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

pub struct Static {
    pub size: u16,
    pub trigger: u16,
    pub width: u16,
    pub height: u16,
    pub rows: Vec<StaticRow>,
}

pub enum TileOrStatic {
    Tile(Box<Tile>),
    Static(Static),
}

pub struct ArtReader<T: Read + Seek> {
    mul_reader: MulReader<T>,
}

impl ArtReader<File> {
    pub fn new(index_path: &Path, mul_path: &Path) -> Result<ArtReader<File>> {
        let mul_reader = MulReader::new(index_path, mul_path)?;
        Ok(ArtReader {
            mul_reader,
        })
    }
}

impl<T: Read + Seek> ArtReader<T> {
    pub fn from_mul(reader: MulReader<T>) -> ArtReader<T> {
        ArtReader { mul_reader: reader }
    }

    pub fn read(&mut self, id: u32) -> Result<TileOrStatic> {
        let raw = self.mul_reader.read(id)?;
        let mut reader = Cursor::new(raw.data);
        if id >= STATIC_OFFSET {
            //It's a static, so deal with accordingly
            let size = reader.read_u16::<LittleEndian>()?;
            let trigger = reader.read_u16::<LittleEndian>()?;
            let width = reader.read_u16::<LittleEndian>()?;
            let height = reader.read_u16::<LittleEndian>()?;
            if width == 0 || width >= 1024 || height == 0 || height >= 1024 {
                Err(Error::other(
                    format!("Got invalid width and height of {}, {}", width, height),
                ))
            } else {
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

                Ok(TileOrStatic::Static(Static {
                    size,
                    trigger,
                    width,
                    height,
                    rows,
                }))
            }
        } else {
            //It's a map tile
            if raw.length > TILE_SIZE {
                Err(Error::other(
                    format!("Got tile size of {}, expected {}", raw.length, TILE_SIZE),
                ))
            } else {
                let header = reader.read_u32::<LittleEndian>()?;
                let mut body = [0; 1022];
                for cell in &mut body {
                    *cell = reader.read_u16::<LittleEndian>().unwrap_or(0);
                }
                Ok(TileOrStatic::Tile(Box::new(Tile {
                    header,
                    image_data: body,
                })))
            }
        }
    }

    pub fn read_tile(&mut self, id: u32) -> Result<Tile> {
        match self.read(id) {
            Ok(TileOrStatic::Tile(tile)) => Ok(*tile),
            Ok(_) => Err(Error::other("Index out of bounds")),
            Err(e) => Err(e),
        }
    }

    pub fn read_static(&mut self, id: u32) -> Result<Static> {
        match self.read(id + STATIC_OFFSET) {
            Ok(TileOrStatic::Static(stat)) => Ok(stat),
            Ok(_) => Err(Error::other("Index out of bounds")),
            Err(e) => Err(e),
        }
    }
}
