use crate::error::{MulReaderError, MulReaderResult};
use crate::mul::MulReader;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub const BLOCK_SIZE: usize = 196;

/// An individual tile on a map
#[derive(Clone, Copy)]
pub struct Cell {
    /// A lookup into art.mul as to what the floor tile should be
    pub graphic: u16,
    /// How high the floor tile is.
    /// When rendering, the reference point is the bottom right, the remaining corners are altered
    /// to match the surrounding tiles
    pub altitude: i8,
}

/// A block. Blocks contain a grid of 8x8 cells.
///
/// Blocks are stored in Maps in columns, top to bottom, left to right
#[derive(Clone, Copy)]
pub struct Block {
    /// Unused
    pub checksum: u32,
    /// An 8x8 grid of cells, stored in rows.
    /// The first row is the top of the block, working downwards
    pub cells: [Cell; 64],
}

/// The location of a fixed map prop, relative to a block
#[derive(Clone, Copy)]
pub struct StaticLocation {
    /// A lookup into art.mul, as to what the static should be
    pub object_id: u16,
    /// The x location of the prop
    pub x: u8,
    /// The y location of the prop (increasing y means further downwards)
    pub y: u8,
    /// How high to render this item
    pub altitude: i8,
    /// Unused
    pub checksum: u16,
}

impl StaticLocation {
    /// Find the position in RadarCol that contains the color to render in the minimap
    pub fn color_idx(&self) -> u16 {
        self.object_id + 16384
    }
}

/// Read a block from a map, or a diff file
pub fn read_block<T: Read + Seek>(data_reader: &mut T, id: u32) -> MulReaderResult<Block> {
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

/// Read static locations from a map, or from a diff file
pub fn read_block_statics<T: Read + Seek>(
    mul_reader: &mut MulReader<T>,
    id: u32,
) -> MulReaderResult<Vec<StaticLocation>> {
    let raw = mul_reader.read(id)?;
    let len = raw.data.len();
    if len % 7 != 0 {
        return Err(MulReaderError::UnexpectedSize {
            expected: (len + (len % 7)) as u32,
            found: len as u32,
        });
    }
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
