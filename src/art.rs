//! Art objects represent both tiles and static graphics.

use mul_reader::MulReader;
use std::io::{IoResult, MemReader, IoError, OtherIoError};
use color::{Color, Color16, Color32};
use utils::DataBuffer;

//NOTE: apparently, when looking up statics by ID, they're offset by 0x4000.

/*impl Tile for MapTile {
    fn with_transparency(&self, transparency_color: pixel) -> ~[pixel] {
        let mut image: ~[pixel] = ~[];
        let mut data_source = byte_helpers::Buffer::new(self.raw_image.clone());

        for uint::range(0, 44) |i| {
            
            let slice_size: uint = if (i >= 22) {(44 - i) * 2} else {(i + 1) * 2};
            image.grow((22 - (slice_size / 2)), &transparency_color);
            let slice_data = data_source.read_items(slice_size);
            image.push_all(slice_data);
            image.grow((22 - (slice_size / 2)), &transparency_color);
        };
        image
    }
}

impl Tile for StaticTile {
    fn with_transparency(&self, transparency_color: pixel) -> ~[pixel] {
        let mut image: ~[pixel] = ~[];

        for self.raw_image_rows.iter().advance |row| {
            let mut current_width = 0;
            for row.iter().advance |run_pair| {
                image.grow(run_pair.offset as uint, &transparency_color);
                image.push_all(run_pair.run);
                current_width += run_pair.offset as uint + run_pair.run.len();
                assert!(current_width <= self.width as uint)
            }
            if current_width < self.width as uint {
                image.grow((self.width as uint) - current_width, &transparency_color)
            }
        }
        image 
    }
}
*/

pub trait Art {
    /**
     * Convert to a 32bit array
     *
     * Returns (width, height, colors)
     */
    fn to_32bit(&self) -> (u32, u32, Vec<Color32>);
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
            
            let slice_size = if (i >= 22) {
                (44 - i) * 2
            } else {
                (i + 1) * 2
            };

            image.grow((22 - (slice_size / 2)), 0);
            for pixel_idx in range(0, slice_size) {
                let (r, g, b, a) = reader.read().to_rgba();
                image.push(Color::from_rgba(r, g, b, a));
            }
            image.grow((22 - (slice_size / 2)), 0);
        };
        (44, 44, image)
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
            //let size = try!(reader.read_le_u16());
            //let trigger = try!(reader.read_le_u16());
            panic!("Not yet implemented");
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
}

/*
impl TileReader {

    pub fn read_tile(&self, id: uint) -> option::Option<MapTile> {
        match self.mul_reader.read(id) {
            option::Some(record) => {
                if (record.data.len() != expected_tile_size) {
                    io::println(fmt!("%u BAD SIZE AT %u VERSUS %u", id, record.data.len(), expected_tile_size));
                    return option::None;
                }

                let mut data_source = byte_helpers::Buffer::new(record.data);
                let record_header = byte_helpers::bytes_to_le_uint(data_source.read_items(4));
                let raw_image: ~[pixel] = byte_helpers::u8vec_to_u16vec(data_source.read_items(1012 * 2));

                option::Some(MapTile{
                    header: record_header as u32,
                    raw_image: raw_image 
                })
            },
            option::None => option::None
        }
    }

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

pub fn TileReader(index_path: &path::Path, mul_path: &path::Path) -> result::Result<TileReader, ~str> {
    match mul_reader::MulReader::new(index_path, mul_path) {
        result::Err(message) => result::Err(message),
        result::Ok(mul_reader) => {
            result::Ok(TileReader{
                mul_reader: mul_reader
            })
        }
    }
}

/*pub fn load_tiles(root_path: ~str) -> (~[(uint, MapTile)], ~[(uint, StaticTile)]) { //TODO: Find a better return type for this
    match mul_reader::reader(root_path, ~"artidx.mul", ~"art.mul") {
        result::Err(message) => {
            io::println(fmt!("Error reading art tiles - %s", message));
            fail
        },
        result::Ok(reader) => {
            let mut map_tiles: ~[(uint, MapTile)] = ~[];
            let mut static_tiles: ~[(uint, StaticTile)] = ~[];

            let mut index:uint = 0;
            while (reader.eof() != true) {
                let item: option::Option<mul_reader::MulRecord> = reader.read_bytes();
                if option::is_some(&item) {
                    let unwrapped: mul_reader::MulRecord = option::unwrap(item);
                    //let record_header = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 0, 3));

                    if (index < 0x4000) {
                        let maybe_map_tile: option::Option<MapTile> = parse_map_tile(unwrapped);
                        if option::is_some(&maybe_map_tile) {
                            let tuple = (index, option::unwrap(maybe_map_tile));
                            map_tiles.push(tuple);
                        }
                    } else if (index < 0x8000){
                        let maybe_static_tile: option::Option<StaticTile> = parse_static_tile(unwrapped);
                        if option::is_some(&maybe_static_tile) {
                            let tuple = (index, option::unwrap(maybe_static_tile));
                            static_tiles.push(tuple);
                        }
                    }
                }
                index += 1;
            }

            (map_tiles, static_tiles)
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
}*/*/
