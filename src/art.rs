//NOTE: apparently, when looking up statics by ID, they're offset by 0x4000.
export map_tile;
export load_tiles;
export to_bitmap;
export parse_map_tile;

type map_tile = {
    header: u32,
    image: ~[u16] //TODO: Consider making Pixel a type
};

type static_tile = {
    header: u32,
    width: u16,
    height: u16,
    image: ~[u16]
};

const transparent: u16 = 0b1000000000000000;
const expected_tile_size: uint = 2048;

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
            
            if (record_header > 0xFFFF || record_header == 0) {
                let maybe_map_tile: option::option<map_tile> = parse_map_tile(unwrapped);
                if option::is_some(maybe_map_tile) {
                    vec::push(map_tiles, maybe_map_tile.get());
                }
            } else {
                let maybe_static_tile: option::option<static_tile> = parse_static_tile(unwrapped);
                if option::is_some(maybe_static_tile) {
                    vec::push(static_tiles, maybe_static_tile.get());
                }
            }
        }
    }

    ret (map_tiles, static_tiles);
}

fn parse_map_tile(record: mul_reader::mul_record) -> option::option<map_tile> { //Interestingly, pixels seem to be 565, rather than 555

    let record_header = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 0, 3));

    if (vec::len(record.data) == expected_tile_size) {
        ret option::none;
    }
    let mut image: ~[u16] = ~[];
    let data_slice: ~[u8] = vec::slice(record.data, 4, vec::len(record.data));

    let mut data_pointer: uint = 0;
    for uint::range(0, 44) |i| {
        
        let slice: uint = if (i >= 22) {(44 - i) * 2} else {(i + 1) * 2};
        vec::grow(image, (22 - (slice / 2)), transparent);
        let slice_data: ~[u8] = vec::slice(data_slice, data_pointer, data_pointer + (slice * 2));
        vec::push_all(image, byte_helpers::u8vec_to_u16vec(slice_data));
        vec::grow(image, (22 - (slice / 2)), transparent);
        data_pointer += (slice * 2);
    };

    ret option::some({
        header: record_header as u32,
        image: image 
    });
}
fn parse_static_tile(record: mul_reader::mul_record) -> option::option<static_tile> {
    let record_header: u32 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 0, 3)) as u32;
    let width: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 4, 5)) as u16;
    let height: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 6, 7)) as u16;
    let mut image: ~[u16] = ~[];

    if (width == 0 || height == 0 || width >= 2048 || height >= 2048) {
        ret option::none;
    }

    //Stuff all of the Offsets into an array
    let mut offsets: ~[u16] = ~[];
    
    for uint::range(0, height as uint) |i| {    
        //Ze plan - read the offset, then loop while we're reading out runs 
        vec::push(offsets, byte_helpers::bytes_to_le_uint(vec::slice(record.data, 8 + (2 * i), 9 + (2 * i))) as u16);
    };

    for offsets.each |offset| {
        //Possible that offsets are pixels
        let byte_offset = offset * 2;
        let mut byte_pos = 0;
        loop {
            //Read run padding - u16
            let run_padding = byte_helpers::bytes_to_le_uint(vec::slice(record.data, (byte_offset + byte_pos) as uint, ((byte_offset + byte_pos) + 1) as uint)) as u16;
            //Read run length - u16
            let run_length = byte_helpers::bytes_to_le_uint(vec::slice(record.data, (byte_offset + byte_pos + 2) as uint, (byte_offset + byte_pos + 3) as uint)) as u16;
            io::println(#fmt("Offset %u, padding %u, length %u",byte_offset as uint, run_padding as uint, run_length as uint));
            if (run_padding == 0 && run_length == 0) {
                break;
            }
            //Check expected length
            if (run_padding + run_length > 2048) {
                io::println("Unexpected length!");
                ret option::none;
            }
            let run_pixels: ~[u16] = byte_helpers::u8vec_to_u16vec(vec::slice(record.data, (byte_offset + byte_pos + 4) as uint, (byte_offset + byte_pos+ 4 + (run_length * 2)) as uint));
            //increase by offset
            vec::grow(image, run_padding as uint, transparent); //Add the padding
            vec::push_all(image, run_pixels);
            io::println(#fmt("Pixels read - %u", vec::len(run_pixels) as uint));
            byte_pos += 4 + (run_length * 2);
        }
        
    };

    

    /*for uint::range(0, height as uint) |i| {    
        //Ze plan - read the offset, then loop while we're reading out runs 
        let mut offset: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 8 + (2 * i), 9 + (2 * i))) as u16;
        io::println(#fmt("Offset %u", offset as uint));

        //Read the run - u16 offset (number of transparent pixels to write) - u16 run length (pixels to read?) - u16 pixel data
        let mut row_length: uint = 0;
        io::println(#fmt("Start row %u", i));
        loop {
            let padding: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, offset as uint, (offset + 1) as uint)) as u16;
            let length: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, (offset + 2) as uint, (offset + 3) as uint)) as u16;
            io::println(#fmt("Padding %u, length %u", padding as uint, length as uint));
            if (padding == 0 && length == 0) {
                break;
            }
            if (row_length + (padding as uint) + (length as uint) >= 2048) { //Corrupt image
                ret option::none;
            }
            vec::grow(image, padding as uint, transparent); //Add the padding
            
            let run_pixels: ~[u16] = byte_helpers::u8vec_to_u16vec(vec::slice(record.data, (offset + 4) as uint, (offset + 4 + (length * 2)) as uint));
            io::println(#fmt("Pixels read - %u", vec::len(run_pixels) as uint));
            vec::push_all(image, run_pixels);
            offset += (vec::len(run_pixels) * 2) as u16;
            row_length += (padding + length) as uint;
            io::println(#fmt("Row length %u", row_length as uint));
        }
        //Write blanks until we reach width
        assert (width as uint) >= (row_length);
        vec::grow(image, (width as uint - row_length), transparent);
    }*/
    ret option::some({
        header: record_header as u32,
        width: width,
        height: height,
        image: image
    });
}



fn to_bitmap(width: u32, height: u32, data: ~[u16]) -> ~[u8] { //TODO: Make this take arbitrary pixel depths
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
    let mut rows: ~[mut ~[u8]] = ~[mut];
    for uint::range(0, height as uint) |i| {
        let slice = vec::slice(data, i * (width as uint), (i+1) * (width as uint));
        let mut row: ~[u8] = ~[];
        for slice.each |sliced| {
            vec::push_all(row, byte_helpers::uint_to_le_bytes(sliced as u64, 2));
        }
        vec::push(rows, row);
    }; 
    vec::reverse(rows);
    //vec::grow(pixels, 44 * 44 * 4, 0x7f);

    ret vec::concat(~[
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
}
