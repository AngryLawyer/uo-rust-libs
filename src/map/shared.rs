use crate::mul_reader::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Read, Result, Seek, SeekFrom};

pub const BLOCK_SIZE: usize = 196;
pub const OFFSET: u32 = 4;
pub const MAP0_SIZE: u32 = 393216;

pub mod map_size {
    pub const SOSARIA: (u32, u32) = (7168, 4096);
    pub const ILSHENAR: (u32, u32) = (2304, 1600);
    pub const MALAS: (u32, u32) = (2560, 2048);
    pub const TOKUNO: (u32, u32) = (1448, 1448);
    pub const TER_MUR: (u32, u32) = (1280, 4096);
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub graphic: u16,
    pub altitude: i8,
}

#[derive(Copy, Clone)]
pub struct Block {
    pub checksum: u32, //Not actually used
    pub cells: [Cell; 64],
}
/*
impl Clone for Block {
    fn clone(&self) -> Self {
        let mut cells = [Cell {
            graphic: 0,
            altitude: 0,
        }; 64];
        for i in 0..64 {
            cells[i] = self.cells[i].clone();
        }
        Block {
            checksum: self.checksum,
            cells: cells,
        }
    }
}*/

#[derive(Clone, Copy)]
pub struct StaticLocation {
    pub object_id: u16,
    pub x: u8,
    pub y: u8,
    pub altitude: i8,
    pub checksum: u16, //Not actually used
}

impl StaticLocation {
    pub fn color_idx(&self) -> u16 {
        self.object_id + 16384
    }
}

/**
 * Read a specific block from a map
 */
pub fn read_block(data_reader: &mut File, id: u32) -> Result<Block> {
    //Cycle to id * 192
    data_reader.seek(SeekFrom::Start((id * BLOCK_SIZE as u32) as u64))?;
    //Read the header
    let mut block = Block {
        checksum: data_reader.read_u32::<LittleEndian>()?,
        cells: [Cell {
            graphic: 0,
            altitude: 0,
        }; 64],
    };
    //Read 64 cells
    for i in 0..64 {
        block.cells[i] = Cell {
            graphic: data_reader.read_u16::<LittleEndian>()?,
            altitude: data_reader.read_i8()?,
        };
    }
    Ok(block)
}

pub fn read_block_statics<T: Read + Seek>(
    mul_reader: &mut MulReader<T>,
    id: u32,
) -> Result<Vec<StaticLocation>> {
    let raw = mul_reader.read(id)?;
    let len = raw.data.len();
    assert!(len % 7 == 0);
    let mut reader = Cursor::new(raw.data);
    let mut statics = vec![];
    for _i in 0..(len / 7) {
        let object_id = reader.read_u16::<LittleEndian>()?;
        let x = reader.read_u8()?;
        let y = reader.read_u8()?;
        let altitude = reader.read_i8()?;
        let checksum = reader.read_u16::<LittleEndian>()?;
        statics.push(StaticLocation {
            object_id,
            x,
            y,
            altitude,
            checksum,
        });
    }
    Ok(statics)
}
