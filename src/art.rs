//NOTE: apparently, when looking up statics by ID, they're offset by 0x4000.
export map_tile;
export load_tiles;
export map_tile_to_bitmap;

type map_tile = {
    header: u32,
    image: ~[u16] //TODO: Consider making Pixel a type
};

type static_tile = {
    header: u32,
    width: u16,
    height: u16,
    rows: ~[~[run]]
};

type run = {
    offset: u16,
    length: u16,
    data: ~[u8]
};

const transparent: u16 = 0b1000000000000000;

fn load_tiles(root_path: ~str) -> (~[map_tile], ~[static_tile]) { //TODO: Find a better return type for this
    let reader:mul_reader::mul_reader = mul_reader::mul_reader(root_path, ~"artidx.mul", ~"art.mul");

    let mut map_tiles: ~[map_tile] = ~[];
    let mut static_tiles: ~[static_tile] = ~[];

    while (reader.eof() != true) {
        let item: option::option<mul_reader::mul_record> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::mul_record = option::get(item);
            let record_header = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 0, 3));
            //Apparently, these flag values represent whether something is a tile or not
            //Others are not convinced, and think that index is all that matters
            //TODO: provide a check against incorrect lengths, as this causes problems
            if (record_header > 0xFFFF || record_header == 0) {
                vec::push(map_tiles, parse_map_tile(unwrapped));
            } else {
            }
        }
    }

    ret (map_tiles, static_tiles);
}

fn parse_map_tile(record: mul_reader::mul_record) -> map_tile {

    let record_header = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 0, 3));
    let mut image: ~[u16] = ~[];
    let data_slice: ~[u8] = vec::slice(record.data, 4, vec::len(record.data));

    let mut data_pointer: uint = 0;
    for uint::range(0, 44) |i| {
        
        let slice: uint = if (i >= 22) {(44 - i) * 2} else {(i + 1) * 2};
        vec::grow(image, (22 - slice), transparent);

        let slice_data: ~[u8] = vec::slice(data_slice, data_pointer, data_pointer + (slice * 2));

        vec::push_all(image, byte_helpers::u8vec_to_u16vec(slice_data));

        vec::grow(image, (22 - slice), transparent);
        data_pointer += (slice * 2);
    };

    ret {
        header: record_header as u32,
        image: image 
    };
}
/*fn parse_static_tile(record: mul_reader::mul_record) -> static_tile {
    let record_header: u32 = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 0, 3)) as u32;
    let width: u16 = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 4, 5)) as u16;
    let height: u16 = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 6, 7)) as u16;

    let mut rows: ~[ mut ~[run]] = ~[mut];

    for uint::range(0, height) |i| {    
        //Ze plan - read a row, starting from position 8, all the way to height
        let offset: u16 = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 8 + (2 * i), 9 + (2 * i)), 2) as u16;
        
    }
}*/



fn to_bitmap(width: u32, height: u32, data: ~[u16]) -> ~[u8] { //TODO: Make this take arbitrary pixel depths
    let signature: ~[u8] = ~[0x42, 0x4D];
    let file_size: ~[u8] = byte_helpers::uint_to_le_bytes((width * height * 2) + 14 + 40, 4);
    let reserved: ~[u8] = ~[0, 0, 0, 0];
    let data_offset: ~[u8] = byte_helpers::uint_to_le_bytes(54, 4);

    let header_size: ~[u8] = byte_helpers::uint_to_le_bytes(40, 4);
    let width: ~[u8] = byte_helpers::uint_to_le_bytes(width, 4); //FIXME: should be signed?
    let height: ~[u8] = byte_helpers::uint_to_le_bytes(height, 4);
    let colour_panes: ~[u8] = ~[1, 0];
    let depth: ~[u8] = ~[16, 0];
    let compression: ~[u8] = ~[0,0,0,0];
    let image_size: ~[u8] = ~[0,0,0,0];
    let horizontal_res: ~[u8] = ~[0, 0, 0, 0];
    let vertical_res: ~[u8] = ~[0, 0, 0, 0];
    let palette_count: ~[u8] = ~[0, 0, 0, 0];
    let important_colours: ~[u8] = ~[0, 0, 0, 0];

    //54 bytes so far
    //44 pixels per row, each 3 bytes
    //44 columns
    
    //Here's where it gets crazy - image data is stored as 2, 4, 8,  up to 44, then 44 down again
    //TODO: explode the image vector, iterate backwards, turn it into bytes
     
    //vec::grow(pixels, 44 * 44 * 4, 0x7f);

    ret vec::concat(~[
        signature,
        file_size,
        reserved,
        data_offset,

        header_size,
        width,
        height,
        colour_panes,
        depth,
        compression,
        image_size,
        horizontal_res,
        vertical_res,
        palette_count,
        important_colours

        //vec::concat(pixel_rows)
    ]);
}
