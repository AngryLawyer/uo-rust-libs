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
        unimplemented!();
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

/*
impl TileReader {

    pub fn read_static(&self, id: uint) -> option::Option<StaticTile> {
        match self.mul_reader.read(id) {    
            option::Some(record) => {
                let mut data_source = byte_helpers::Buffer::new(record.data);
                let data_size: u16 = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16; //Might not be size :P
                let trigger: u16 = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;
                let width: u16 = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;
                let height: u16 = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;

                if (width == 0 || height >= 1024 || height == 0 || height >= 1024) {
                    error!("Bad image dimensions found at %u", id);
                    return option::None;
                }

                //Read the offset table
                let mut offset_table: ~[u16] = ~[];
                for uint::range(0, height as uint) |_index| {
                    let offset = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;
                    offset_table.push(offset);
                }

                let data_start_pos = data_source.pos;
                let mut rows = ~[];

                for offset_table.iter().advance |offset| {
                    data_source.seek(data_start_pos as uint + (*offset as uint * 2));
                    let mut current_row_width: uint = 0;
                    let mut row = ~[];

                    loop {
                        let x_offset = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;
                        let run_length = byte_helpers::bytes_to_le_uint(data_source.read_items(2)) as u16;

                        if (x_offset + run_length == 0) {
                            break;
                        } else {
                            row.push(RunPair{
                                offset: x_offset,
                                run: byte_helpers::u8vec_to_u16vec(data_source.read_items((run_length as uint) * 2))
                            });
                            current_row_width += x_offset as uint + run_length as uint;
                            assert!(current_row_width <= width as uint);
                        }
                    }
                    rows.push(row);
                }

                option::Some(StaticTile {
                    data_size: data_size,
                    trigger: trigger,
                    width: width,
                    height: height,
                    raw_image_rows: rows
                })
            },
            option::None => option::None
        }
    }
}


fn parse_static_tile(record: mul_reader::MulRecord) -> option::Option<StaticTile> {

    let data_source = byte_helpers::ByteBuffer(copy record.data);

    let data_size: u16 = data_source.read_le_uint(2) as u16; //Might not be size :P
    let trigger: u16 = data_source.read_le_uint(2) as u16;
    let width: u16 = data_source.read_le_uint(2) as u16;
    let height: u16 = data_source.read_le_uint(2) as u16;

    if (width == 0 || height >= 1024 || height == 0 || height >= 1024) {
        io::println("Bad image dimensions found");
        return option::None;
    }

    let mut image: ~[u16] = ~[];

    //Read the offset table
    let mut offset_table: ~[u16] = ~[];
    for uint::range(0, height as uint) |_index| {
        let offset = data_source.read_le_uint(2) as u16;
        offset_table.push(offset);
    }

    let data_start_pos = data_source.pos;

    for offset_table.each |offset| {
        data_source.seek(data_start_pos as uint + (*offset as uint * 2));
        let mut current_row_width: uint = 0;

        loop {
            let x_offset = data_source.read_le_uint(2) as u16;
            let run_length = data_source.read_le_uint(2) as u16;

            if (x_offset + run_length == 0) {
                image.grow(width as uint - current_row_width, &transparent);
                break;
            } else {
                let run = byte_helpers::u8vec_to_u16vec(data_source.read_bytes((run_length as uint) * 2));
                image.grow(x_offset as uint, &transparent);
                image.push_all(run);
                current_row_width += x_offset as uint + run_length as uint;
                fail_unless!(current_row_width <= width as uint);
            }
        }
    }

    return option::Some({
        data_size: data_size,
        trigger: trigger,
        width: width,
        height: height,
        image: image
    });
}*/


/*
pub fn to_bitmap(width: u32, height: u32, data: ~[u16]) -> ~[u8] { //TODO: Make this take arbitrary pixel depths
    let signature: ~[u8] = ~[0x42, 0x4D];
    let file_size: ~[u8] = byte_helpers::uint_to_le_bytes(((width * height * 2) + 14 + 40) as u64, 4);
    let reserved: ~[u8] = ~[0, 0, 0, 0];
    let data_offset: ~[u8] = byte_helpers::uint_to_le_bytes(54, 4);

    let header_size: ~[u8] = byte_helpers::uint_to_le_bytes(40, 4);
    let width_bytes: ~[u8] = byte_helpers::uint_to_le_bytes(width as u64, 4); //FIXME: should be signed?
    let height_bytes: ~[u8] = byte_helpers::uint_to_le_bytes(height as u64, 4);
    let colour_panes: ~[u8] = ~[1, 0];
    let depth: ~[u8] = ~[16, 0];
    let compression: ~[u8] = ~[0,0,0,0];
    let image_size: ~[u8] = ~[0,0,0,0];
    let horizontal_res: ~[u8] = ~[0, 0, 0, 0];
    let vertical_res: ~[u8] = ~[0, 0, 0, 0];
    let palette_count: ~[u8] = ~[0, 0, 0, 0];
    let important_colours: ~[u8] = ~[0, 0, 0, 0];

    //54 bytes so far
    //TODO: explode the image vector, iterate backwards, turn it into bytes
    let mut rows: ~[~[u8]] = ~[];
    for uint::range(0, height as uint) |i| {
        let slice = vec::slice(data, i * (width as uint), (i+1) * (width as uint));
        let mut row: ~[u8] = ~[];
        for slice.each |sliced| {
            row.push_all(byte_helpers::uint_to_le_bytes(*sliced as u64, 2));
        }
        rows.push(row);
    }; 
    vec::reverse(rows);
    //vec::grow(pixels, 44 * 44 * 4, 0x7f);

    return vec::concat(~[
        signature,
        file_size,
        reserved,
        data_offset,

        header_size,
        width_bytes,
        height_bytes,
        colour_panes,
        depth,
        compression,
        image_size,
        horizontal_res,
        vertical_res,
        palette_count,
        important_colours,

        vec::concat(rows)
    ]);
}*/
