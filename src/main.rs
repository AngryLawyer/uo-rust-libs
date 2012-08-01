fn main() {
    let path = ~"/home/tony/Ubuntu One/";
    /*let skills: ~[skills::skill] = skills::load_skills(path);

    for skills.each |skill| {
        io::println(str::from_bytes(skill.name)); 
    }
*/
    let (map_tiles, static_tiles) = art::load_tiles(path);

    let mut i:uint = 0;
    for map_tiles.each |tile| {
        let bmp_data: ~[u8] = art::to_bitmap(44, 44, tile.image);
        

        //Test writing bitmap
        let maybe_writer = io::file_writer(#fmt("./output/tile%u.bmp", i), ~[io::create, io::truncate]);

        if result::is_err::<io::writer, ~str>(maybe_writer) {
            io::println(#fmt("%s", result::get_err(maybe_writer)));
            assert false;
        }

        let writer: io::writer = result::unwrap(maybe_writer);
       
        writer.write(bmp_data);
        i += 1;
    }

    /*let red_pixel = ~[0b00000000, 0b01111100];
    let green_pixel = ~[0b00000011, 0b11100000];
    let blue_pixel = ~[0b01111100, 0b00000000];
    let white_pixel = ~[0b11111111, 0b11111111];

    let pixels = ~[red_pixel, green_pixel, blue_pixel, white_pixel];

    let mut i = 0;
    for pixels.each |pixel| {
        let mut tile_data: ~[u8] = ~[0, 0, 0, 0];

        for uint::range(0, 1022) |x| {
            vec::push_all(tile_data, pixel);
        }
        io::println(#fmt("%u", vec::len(tile_data)));
        assert vec::len(tile_data) == 2048;

        let tile = art::parse_map_tile({data: tile_data, opt1: 0, opt2: 0});

        let bmp_data = art::to_bitmap(44, 44, tile.image);

        let maybe_writer = io::file_writer(#fmt("./output/tiletest%u.bmp", i), ~[io::create, io::truncate]);

        if result::is_err::<io::writer, ~str>(maybe_writer) {
            io::println(#fmt("%s", result::get_err(maybe_writer)));
            assert false;
        }

        let writer: io::writer = result::unwrap(maybe_writer);
       
        writer.write(bmp_data);
        i += 1;
    }*/

}
