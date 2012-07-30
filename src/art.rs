//NOTE: apparently, when looking up tiles by ID, they're offset by 0x4000.
export map_tile;
export load_tiles;
export map_tile_to_bitmap;

type map_tile = {
    header: u32,
    data: ~[u8]
};

fn load_tiles(root_path: ~str) -> ~[map_tile] {
    let reader:mul_reader::mul_reader = mul_reader::mul_reader(root_path, ~"artidx.mul", ~"art.mul");

    let mut result: ~[map_tile] = ~[];

    while (reader.eof() != true) {
        let item: option::option<mul_reader::mul_record> = reader.read();
        if option::is_some(item) {
            let unwrapped: mul_reader::mul_record = option::get(item);
            let record_header = byte_helpers::bytes_to_le_uint(vec::slice(unwrapped.data, 0, 3));

            //Apparently, these flag values represent whether something is a tile or not
            //Others are not convinced, and think that index is all that matters
            if (record_header > 0xFFFF || record_header == 0) {
                vec::push(result, {
                    header: record_header as u32,
                    data: vec::slice(unwrapped.data, 4, vec::len(unwrapped.data))
                });
            }
        }
    }

    ret result;
}

fn map_tile_to_bitmap(/*tile: map_tile*/) -> ~[u8] {
    let signature: ~[u8] = ~[0x42, 0x4D];
    let file_size: ~[u8] = byte_helpers::uint_to_le_bytes(7744 + 14 + 40, 4);
    let reserved: ~[u8] = ~[0, 0, 0, 0];
    let data_offset: ~[u8] = byte_helpers::uint_to_le_bytes(54, 4);

    let header_size: ~[u8] = byte_helpers::uint_to_le_bytes(40, 4);
    let width: ~[u8] = byte_helpers::uint_to_le_bytes(44, 4); //FIXME: should be signed
    let height: ~[u8] = byte_helpers::uint_to_le_bytes(44, 4);
    let colour_panes: ~[u8] = ~[1, 0];
    let depth: ~[u8] = ~[16, 0];
    let compression: ~[u8] = ~[0,0,0,0];
    let image_size: ~[u8] = ~[0,0,0,0];//byte_helpers::uint_to_le_bytes(7744, 4);
    let horizontal_res: ~[u8] = ~[0, 0, 0, 0];
    let vertical_res: ~[u8] = ~[0, 0, 0, 0];
    let palette_count: ~[u8] = ~[0, 0, 0, 0];
    let important_colours: ~[u8] = ~[0, 0, 0, 0];

    //54 bytes so far
    //44 pixels per row, each 3 bytes
    //44 columns
    
    //Here's where it gets crazy - image data is stored as 2, 4, 8,  up to 44, then 44 down again
    let mut pixels: ~[u8] = ~[];
    for uint::range(0, 44) |i| {
        vec::grow(pixels, 22 * 2, 0x00);
        vec::grow(pixels, 22 * 2, 0xFF);
        /*if (i < 22) {
            vec::grow(pixels, (22 - (i + 1)) * 3, 0xff);
            vec::grow(pixels, ((i + 1) * 2) * 3, 0);
            vec::grow(pixels, (22 - (i + 1)) * 3, 0xff);
        } else {
            vec::grow(pixels, ((i + 1) - 22) * 3, 0xff);
            vec::grow(pixels, (44 - ((i + 1) * 2)) * 3, 0);
            vec::grow(pixels, ((i + 1) - 22) * 3, 0xff);
        }*/
    }
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
        important_colours,

        pixels
    ]);
}
