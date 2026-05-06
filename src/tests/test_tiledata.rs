use crate::tiledata::{Flags, TileDataReader};
use byteorder::{LittleEndian, WriteBytesExt};
use std::ffi::CString;
use std::io::{Cursor, Result, Write};

fn raw_map_tiledata() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>((Flags::BackgroundFlag | Flags::ImpassableFlag).bits())?; // Flags
    data.write_u16::<LittleEndian>(0x1)?; // Texture ID
    data.write_all(CString::new("A tile").unwrap().as_bytes_with_nul())?;
    data.write_all(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])?;
    let unwrapped = data.into_inner();
    assert_eq!(unwrapped.len(), 26);
    Ok(unwrapped)
}

fn raw_map_group() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    let raw_tile = raw_map_tiledata()?;
    data.write_u32::<LittleEndian>(0x0)?; // Unknown
    for _ in 0..32 {
        data.write_all(&raw_tile)?;
    }
    Ok(data.into_inner())
}

fn raw_static_tiledata() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    data.write_u32::<LittleEndian>(
        (Flags::PrefixAFlag | Flags::StackableFlag | Flags::AnimatedFlag).bits(),
    )?; // Flags
    data.write_u8(0x1)?; // Weight
    data.write_u8(0x0)?; // Quality
    data.write_u16::<LittleEndian>(0x0)?; // Unknown
    data.write_u8(0x0)?; // Unknown
    data.write_u8(0x5)?; // Quantity
    data.write_u16::<LittleEndian>(0x0)?; // Anim id
    data.write_u8(0x0)?; // Unknown
    data.write_u8(0x7)?; // Hue
    data.write_u16::<LittleEndian>(0x0)?; // Unknown
    data.write_u8(0x0)?; // Height
    data.write_all(CString::new("stack of coins").unwrap().as_bytes_with_nul())?;
    data.write_all(&[0, 0, 0, 0, 0])?;
    let unwrapped = data.into_inner();
    assert_eq!(unwrapped.len(), 37);
    Ok(unwrapped)
}

fn raw_static_group() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    let raw_tile = raw_static_tiledata()?;
    data.write_u32::<LittleEndian>(0x0)?; // Unknown
    for _ in 0..32 {
        data.write_all(&raw_tile)?;
    }
    Ok(data.into_inner())
}

fn raw_mul() -> Result<Vec<u8>> {
    let mut data = Cursor::new(vec![]);
    let map_group = raw_map_group()?;
    for _ in 0..512 {
        data.write_all(&map_group)?;
    }
    data.write_all(&raw_static_group()?)?;
    Ok(data.into_inner())
}

#[test]
fn test_read_map_tiledata() {
    let data = Cursor::new(raw_mul().unwrap());
    let mut reader = TileDataReader::from_readable(data);
    let map_tile = reader.read_map_tile_data(33).unwrap();
    assert_eq!(
        map_tile.flags,
        Flags::BackgroundFlag | Flags::ImpassableFlag
    );
    assert_eq!(map_tile.texture_id, 1);
    assert_eq!(map_tile.name, "A tile");
}
#[test]
fn test_read_static_tiledata() {
    let data = Cursor::new(raw_mul().unwrap());
    let mut reader = TileDataReader::from_readable(data);
    let static_tile = reader.read_static_tile_data(0).unwrap();
    assert_eq!(
        static_tile.flags,
        (Flags::PrefixAFlag | Flags::StackableFlag | Flags::AnimatedFlag)
    );
    assert_eq!(static_tile.weight, 1);
    assert_eq!(static_tile.quality_layer_light_id, 0);
    assert_eq!(static_tile.quantity_weapon_class_armor_class, 5);
    assert_eq!(static_tile.anim_id, 0);
    assert_eq!(static_tile.hue, 7);
    assert_eq!(static_tile.height_capacity, 0);
}
