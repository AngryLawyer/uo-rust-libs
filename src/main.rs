fn main() {
    //let path = ~"../uo-aos/";
    let path = ~"/home/tony/Ubuntu One/";
    /*let skills: ~[skills::skill] = skills::load_skills(path);

    for skills.each |skill| {
        io::println(str::from_bytes(skill.name)); 
    }
*/
    let (map_tiles, static_tiles) = art::load_tiles(path);

    let mut i:uint = 0;
    /*for map_tiles.each |tile| {
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
    }*/

    for static_tiles.each |tile| {
        let bmp_data: ~[u8] = art::to_bitmap(tile.width as u32, tile.height as u32, tile.image);

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

}
