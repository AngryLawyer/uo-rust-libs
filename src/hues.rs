//! Methods for reading hue data out of hues.mul
//!
//! Hues are represented in a continuous, unindexed file as groups -
//! `|header: u32|hues: [HueEntry..8]`
//!
//! Individual HueEntries are defined as
//! `|color_table:[u16..32]|table_start:u16|table_end:u16|name:[u8..20]|`
//!
use std::io::{Cursor, Result, SeekFrom, Seek, Read, Write};
use std::fs::{File};
use std::path::Path;
use std::str::{from_utf8};
use std::ascii::AsciiExt;
use utils::MEMWRITER_ERROR;
use color::Color16;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/**
 * An individual Hue
 */
pub struct Hue {
    ///32 color values
    pub color_table: [Color16; 32],
    ///The first hue value in the table
    pub table_start: Color16,
    ///The last hue value in the table
    pub table_end: Color16,
    ///A label for the hue
    pub name: String
}

impl Clone for Hue {

    fn clone(&self) -> Hue {
        Hue {
            color_table: self.color_table,
            table_start: self.table_start,
            table_end: self.table_end,
            name: self.name.clone()
        }
    }
}

impl Hue {
    pub fn new(color_table: [Color16; 32], table_start: Color16, table_end: Color16, name: String) -> Hue {
        Hue {
            color_table: color_table,
            table_start: table_start,
            table_end: table_end,
            name: name
        }
    }

    /**
     * Convert a hue back into its canonical mul representation
     */
    pub fn serialize(&self) -> Vec<u8> {
        let mut writer = vec![];
        for color in self.color_table.iter() {
            writer.write_u16::<LittleEndian>(*color).expect(MEMWRITER_ERROR);
        }
        writer.write_u16::<LittleEndian>(self.table_start).expect(MEMWRITER_ERROR);
        writer.write_u16::<LittleEndian>(self.table_end).expect(MEMWRITER_ERROR);

        writer.write(self.name.as_bytes()).expect(MEMWRITER_ERROR);
        writer.write(vec![0; 20 - self.name.len()].as_slice()).expect(MEMWRITER_ERROR);

        assert_eq!(writer.len(), ENTRY_SIZE as usize);

        writer
    }
}

/**
 * A collection of 8 hues
 */
pub struct HueGroup {
    ///Unknown usage
    pub header: u32,
    pub entries: [Hue; 8]
}

impl HueGroup {

    pub fn new(header: u32, entries: [Hue; 8]) -> HueGroup {
        HueGroup {
            header: header,
            entries: entries
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut writer = Cursor::new(vec![]);
        writer.write_u32::<LittleEndian>(self.header).expect(MEMWRITER_ERROR);
        for hue in self.entries.iter() {
            writer.write(hue.serialize().as_slice()).expect(MEMWRITER_ERROR);
        }
        writer.into_inner()
    }
}

//A hue_entry is (32 * 2) + 2 + 2 + 20 bytes = 88 bytes
const ENTRY_SIZE: u32 = 88;
//8 entries to a group, plus a 4 byte header. 708 bytes.
const GROUP_SIZE: u32 = (ENTRY_SIZE * 8) + 4;

pub struct HueReader<T: Read + Seek> {
    data_reader: T
}

impl HueReader<File> {
    pub fn new(hues_path: &Path) -> Result<HueReader<File>> {
        let data_reader = try!(File::open(hues_path));

        Ok(HueReader {
            data_reader: data_reader
        })
    }
}

impl<T: Read + Seek> HueReader<T> {
    /**
     * If we've already got a file-like object, wrap it
     * */
    pub fn from_readable(data_reader: T) -> HueReader<T> {
        HueReader {
            data_reader: data_reader
        }
    }

    /**
     * Read the given indexed group
     */
    pub fn read_hue_group(&mut self, id: u32) -> Result<HueGroup> {
        try!(self.data_reader.seek(SeekFrom::Start((id * GROUP_SIZE) as u64)));

        let header = try!(self.data_reader.read_u32::<LittleEndian>());

        let entries: [Hue; 8] = [
            try!(self.read_hue()),
            try!(self.read_hue()),
            try!(self.read_hue()),
            try!(self.read_hue()),
            try!(self.read_hue()),
            try!(self.read_hue()),
            try!(self.read_hue()),
            try!(self.read_hue())
        ];

        Ok(HueGroup {
            header: header,
            entries: entries
        })
    }

    fn read_hue(&mut self) -> Result<Hue> {
        let mut color_table = [0u16; 32];
        for idx in 0..32 {
            color_table[idx] = try!(self.data_reader.read_u16::<LittleEndian>());
        }

        let table_start = try!(self.data_reader.read_u16::<LittleEndian>());
        let table_end = try!(self.data_reader.read_u16::<LittleEndian>());

        let mut raw_name = [0; 20];
        try!(self.data_reader.read_exact(&mut raw_name));

        //Slice it down into a normal string size
        let trimmed_name: Vec<u8> = raw_name.into_iter().take_while(|&element| *element != 0).cloned().collect();

        let name = match from_utf8(trimmed_name.as_slice()) {
            Ok(s) => {
                if s.is_ascii() {
                    s.to_string()
                } else {
                    "Error".to_string()
                }
            },
            Err(_) => "Error".to_string()
        };

        Ok(Hue::new(
            color_table,
            table_start,
            table_end,
            name
        ))

    }
}
