//! Art objects represent both tiles and static graphics.

use mul_reader::MulReader;
use std::io::{IoResult, MemReader, IoError, OtherIoError, SeekStyle};
use color::{Color, Color16, Color32};
use utils::DataBuffer;
use utils::MEMWRITER_ERROR;

pub trait Art {
    /**
     * Convert to a 32bit array
     *
     * Returns (width, height, colors)
     */
    fn to_32bit(&self) -> (u32, u32, Vec<Color32>);
    fn serialize(&self) -> Vec<u8>;
}

pub const TILE_SIZE: u32 = 2048;
pub const STATIC_OFFSET: u32 = 0x4000;

pub struct RunPair {
    pub offset: u16,
    pub run: Vec<Color16>
}

impl RunPair {
    fn serialize(&self) -> Vec<u8> {
        let mut writer = vec![];

        writer.write_le_u16(self.offset).ok().expect(MEMWRITER_ERROR);
        writer.write_le_u16(self.run.len() as u16 * 2).ok().expect(MEMWRITER_ERROR);
        for &color in self.run.iter() {
            writer.write_le_u16(color).ok().expect(MEMWRITER_ERROR);
        }
        writer
    }
}

pub type StaticRow = Vec<RunPair>;

pub struct Tile {
    pub header: u32,
    pub image_data: [Color16, ..1022]
}

impl Art for Tile {
    fn to_32bit(&self) -> (u32, u32, Vec<Color32>) {
        let mut image: Vec<Color32> = vec![];

        let mut reader = DataBuffer::new(&self.image_data);

        for i in range(0, 44) {
            
            let slice_size = if i >= 22 {
                (44 - i) * 2
            } else {
                (i + 1) * 2
            };

            image.grow((22 - (slice_size / 2)), 0);
            for _pixel_idx in range(0, slice_size) {
                let (r, g, b, a) = reader.read().to_rgba();
                image.push(Color::from_rgba(r, g, b, a));
            }
            image.grow((22 - (slice_size / 2)), 0);
        };
        (44, 44, image)
    }

    fn serialize(&self) -> Vec<u8> {
        let mut writer = vec![];
        writer.write_le_u32(self.header).ok().expect(MEMWRITER_ERROR);
        for &pixel in self.image_data.iter() {
            writer.write_le_u16(pixel).ok().expect(MEMWRITER_ERROR);
        }
        writer
    }
}

impl Art for Static {
    fn to_32bit(&self) -> (u32, u32, Vec<Color32>) {
        let mut image: Vec<Color32> = vec![];

        for row in self.rows.iter() {
            let mut current_width = 0;
            for run_pair in row.iter() {
                image.grow(run_pair.offset as uint, 0);
                for pixel in run_pair.run.iter() {
                    let (r, g, b, a) = pixel.to_rgba();
                    image.push(Color::from_rgba(r, g, b, a));
                }
                current_width += run_pair.offset as uint + run_pair.run.len();
                assert!(current_width <= self.width as uint)
            }
            if current_width < self.width as uint {
                image.grow((self.width as uint) - current_width, 0)
            }
        };

        (self.width as u32, self.height as u32, image)
    }

    fn serialize(&self) -> Vec<u8> {
        let mut writer = vec![];
        writer.write_le_u16(self.size).ok().expect(MEMWRITER_ERROR);
        writer.write_le_u16(self.trigger).ok().expect(MEMWRITER_ERROR);
        writer.write_le_u16(self.width).ok().expect(MEMWRITER_ERROR);
        writer.write_le_u16(self.height).ok().expect(MEMWRITER_ERROR);

        let mut rows = vec![];

        //Write our rows
        for row in self.rows.iter() {
            let mut out = vec![];
            for pair in row.iter() {
                out.write(pair.serialize().as_slice()).ok().expect(MEMWRITER_ERROR);
            }
            //We write a "newline" after each out
            out.write_le_u16(0).ok().expect(MEMWRITER_ERROR);
            out.write_le_u16(0).ok().expect(MEMWRITER_ERROR);
            rows.push(out);
        }

        let mut lookup_table = vec![];
        let mut last_position = 8u;
        //Generate a lookup table
        for row in rows.iter() {
            lookup_table.write_le_u16(last_position as u16).ok().expect(MEMWRITER_ERROR);
            last_position += row.len();
        }
        writer.write(lookup_table.as_slice()).ok().expect(MEMWRITER_ERROR);
        for row in rows.iter() {
            writer.write(row.as_slice()).ok().expect(MEMWRITER_ERROR);
        }

        writer
    }
}

pub struct Static { 
    pub size: u16,
    pub trigger: u16,
    pub width: u16,
    pub height: u16,
    pub rows: Vec<StaticRow>
}

pub enum TileOrStatic {
    Tile(Tile),
    Static(Static)
}

pub struct ArtReader {
    mul_reader: MulReader
}

impl ArtReader {
    pub fn new(index_path: &Path, mul_path: &Path) -> IoResult<ArtReader> {
        let mul_reader = try!(MulReader::new(index_path, mul_path));
        Ok(ArtReader {
            mul_reader: mul_reader
        })
    }

    pub fn read(&mut self, id: u32) -> IoResult<TileOrStatic> {
        let raw = try!(self.mul_reader.read(id));
        let mut reader = MemReader::new(raw.data);
        if id >= STATIC_OFFSET {
            //It's a static, so deal with accordingly
            let size = try!(reader.read_le_u16());
            let trigger = try!(reader.read_le_u16());
            let width = try!(reader.read_le_u16());
            let height = try!(reader.read_le_u16());
            if width == 0 || height >= 1024 || height == 0 || height >= 1024 {
                Err(IoError{
                    kind: OtherIoError,
                    desc: "Invalid image dimensions",
                    detail: Some(format!("Got invalid width and height of {}, {}", width, height))
                })
            } else {
                //Load our offset table
                let mut offset_table = vec![];
                for _index in range(0, height) {
                    offset_table.push(try!(reader.read_le_u16()));
                }
    
                let data_start_pos = try!(reader.tell());
                let mut rows = vec![];

                for &offset in offset_table.iter() {
                    try!(reader.seek((data_start_pos + offset as u64 * 2) as i64, SeekStyle::SeekSet));
                    let mut current_row_width = 0;
                    let mut row = vec![];

                    loop {
                        let x_offset = try!(reader.read_le_u16());
                        let run_length = try!(reader.read_le_u16());
                        if x_offset + run_length == 0 {
                            break
                        } else {
                            let mut run = vec![];
                            for _index in range(0, run_length) {
                                run.push(try!(reader.read_le_u16()));
                            }

                            row.push(RunPair {
                                offset: x_offset,
                                run: run
                            });
                            current_row_width += x_offset + run_length;
                            //assert!(current_row_width < width);
                        }
                    }
                    rows.push(row);
                }

                Ok(TileOrStatic::Static(Static {
                    size: size,
                    trigger: trigger,
                    width: width,
                    height: height,
                    rows: rows
                }))
            }
        } else {
            //It's a map tile
            if raw.length != TILE_SIZE {
                Err(IoError{
                    kind: OtherIoError,
                    desc: "Invalid tile size",
                    detail: Some(format!("Got tile size of {}, expected {}", raw.length, TILE_SIZE))
                })
            } else {
                let header = try!(reader.read_le_u32());
                let mut body = [0, ..1022];
                for idx in range(0, 1022) {
                    body[idx] = try!(reader.read_le_u16());
                }
                Ok(TileOrStatic::Tile(Tile {
                    header: header, image_data: body
                }))
            }
        }
    }

    pub fn read_tile(&mut self, id: u32) -> IoResult<Tile> {
        match self.read(id) {
            Ok(TileOrStatic::Tile(tile)) => Ok(tile),
            Ok(_) => Err(IoError {
                kind: OtherIoError,
                desc: "Index out of bounds",
                detail: None
            }),
            Err(e) => Err(e)
        }
    }

    pub fn read_static(&mut self, id: u32)-> IoResult<Static> {
        match self.read(id + STATIC_OFFSET) {
            Ok(TileOrStatic::Static(stat)) => Ok(stat),
            Ok(_) => Err(IoError {
                kind: OtherIoError,
                desc: "Index out of bounds",
                detail: None
            }),
            Err(e) => Err(e)
        }
    }
}
