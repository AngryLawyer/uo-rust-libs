
fn example_gump_mul() -> GumpReader<Cursor<Vec<u8>>> {
    let mut data = Cursor::new(vec![]);

    data.write_u32::<LittleEndian>(0).unwrap();  //Row 1 offset
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 2 offset
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 3 offset

    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 1, run 1 data
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 1 EOL

    data.write_u16::<LittleEndian>(0).unwrap();  //Row 2, run 1 spacer
    data.write_u16::<LittleEndian>(3).unwrap();  //Row 2, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 2, run 1 data
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 2, run 1 data
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 2, run 1 data
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 2 EOL

    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 spacer
    data.write_u16::<LittleEndian>(1).unwrap();  //Row 1, run 1 number of pixels
    data.write_u16::<LittleEndian>(0xFFFF).unwrap();  //Row 1, run 1 data
    data.write_u32::<LittleEndian>(0).unwrap();  //Row 2 EOL

    let mut padded = vec![];
    for _i in 0..STATIC_OFFSET {
        padded.push(vec![]);
    }
    padded.push(data.into_inner());

    let mul_reader = simple_from_vecs(padded, 3, 0);
    ArtReader::from_mul(mul_reader)
}
