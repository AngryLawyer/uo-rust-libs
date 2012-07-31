fn main() {
    let path = ~"../uo-aos/";
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

    /*let pixel = 0b0111110000000000;
    let mut image_data: ~[u16] = ~[];
    vec::grow(image_data, 44 * 44 * 2, pixel);
    let bmp_data = art::to_bitmap(44, 44, image_data);

    let maybe_writer = io::file_writer(~"./output/tiletest.bmp", ~[io::create, io::truncate]);

    if result::is_err::<io::writer, ~str>(maybe_writer) {
        io::println(#fmt("%s", result::get_err(maybe_writer)));
        assert false;
    }

    let writer: io::writer = result::unwrap(maybe_writer);
   
    writer.write(bmp_data);*/
}
