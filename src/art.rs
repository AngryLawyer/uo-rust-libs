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

    let mut index:uint = 0;
    while (reader.eof() != true) {
        let item: option::option<mul_reader::mul_record> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::mul_record = option::get(item);
            let record_header = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 0, 3));
            //Apparently, these flag values represent whether something is a tile or not
            //Others are not convinced, and think that index is all that matters

            //if (record_header > 0xFFFF || record_header == 0) {
            if (index < 0x4000) {
                /*let maybe_map_tile: option::option<map_tile> = parse_map_tile(unwrapped);
                if option::is_some(maybe_map_tile) {
                    vec::push(map_tiles, maybe_map_tile.get());
                }*/
            } else if (index < 0x8000){
                let maybe_static_tile: option::option<static_tile> = parse_static_tile(unwrapped);
                if option::is_some(maybe_static_tile) {
                    vec::push(static_tiles, maybe_static_tile.get());
                }
            }
        }
        index += 1;
    }

    ret (map_tiles, static_tiles);
}

fn parse_map_tile(record: mul_reader::mul_record) -> option::option<map_tile> { //Interestingly, pixels seem to be 565, rather than 555

    if (vec::len(record.data) != expected_tile_size) {
        ret option::none;
    }

    let record_header = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 0, 3));

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
    let data_size: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 0, 1)) as u16; //Might not be size :P
    let trigger: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 2, 3)) as u16;

    let width: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 4, 5)) as u16;
    let height: u16 = byte_helpers::bytes_to_le_uint(vec::slice(record.data, 6, 7)) as u16;
    let mut image: ~[u16] = ~[];

    if (width == 0 || height == 0 || width > 1024 || height > 1024) {
        ret option::none;
    }

    //Stuff all of the Offsets into an array
    let mut offsets: ~[u16] = ~[];

    for uint::range(0, height as uint) |i| {    
        //Ze plan - read the offset, then loop while we're reading out runs 
        vec::push(offsets, byte_helpers::bytes_to_le_uint(vec::slice(record.data, 8 + (2 * i), 9 + (2 * i))) as u16);
    };

    let run_data_start = (height * 2) + 8;

    for offsets.each |offset| {
        let mut run_start = run_data_start + (offset * 2);
        let mut current_width = 0;

        loop {
            let run_padding = byte_helpers::bytes_to_le_uint(vec::slice(record.data, run_start as uint, run_start as uint + 1)) as u16;
            let run_length = byte_helpers::bytes_to_le_uint(vec::slice(record.data, run_start as uint + 2, run_start as uint + 3)) as u16;
            if run_length == 0 && run_padding == 0 {
                break;
            }
            current_width += (run_padding + run_length);
            if (current_width > width) {
                ret option::none;
            }

            if (current_width > 2048) {
                ret option::none;
            }
            //Add the padding!
            vec::grow(image, run_padding as uint, transparent);
            //Read ze pixels!
            let run_data = vec::slice(record.data, run_start as uint + 4, run_start as uint + 4 + (run_length as uint * 2));
            
            vec::push_all(image, byte_helpers::u8vec_to_u16vec(run_data));
            
            //io::println(#fmt("%u, %u", run_padding as uint, run_length as uint));
            run_start += 4 + (run_length * 2);
        }
        //io::println("ROW END");
        vec::grow(image, (width - current_width) as uint, transparent);
        //Pad the end of the row
        
    };
    //io::println("Image end");
    assert vec::len(image) == (width as uint) * (height as uint);

    ret option::some({
        header: 0,// record_header as u32,
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
