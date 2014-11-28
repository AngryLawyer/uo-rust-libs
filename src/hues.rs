//! Methods for reading hue data out of hues.rs
//!
//! Hues are represented in a continuous, unindexed file as groups - 
//! `|header: u32|hues: [HueEntry..8]`
//!
//! Individual HueEntries are defined as
//! `|color_table:[u16..32]|table_start:u16|table_end:u16|name:[u8..20]|`
//!
use std::io::{File, Open, Read, IoResult, SeekSet, MemWriter};
use utils::MEMWRITER_ERROR;
use color::Color16;

/**
 * An individual Hue
 */
pub struct Hue {
    ///32 color values
    pub color_table: [Color16, ..32],
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
    pub fn new(color_table: [Color16, ..32], table_start: Color16, table_end: Color16, name: String) -> Hue {
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
        let mut writer = MemWriter::new();
        for color in self.color_table.iter() {
            writer.write_le_u16(*color).ok().expect(MEMWRITER_ERROR);
        }
        writer.write_le_u16(self.table_start).ok().expect(MEMWRITER_ERROR);
        writer.write_le_u16(self.table_end).ok().expect(MEMWRITER_ERROR);

        let raw_string = self.name.clone().to_c_str();
        
        writer.write(raw_string.as_bytes()).ok().expect(MEMWRITER_ERROR);
        writer.write(Vec::from_elem(20 - raw_string.len() - 1, 0).as_slice()).ok().expect(MEMWRITER_ERROR);

        let output = writer.into_inner();
        assert_eq!(output.len(), ENTRY_SIZE);

        output
    }
}

/**
 * A collection of 8 hues
 */
pub struct HueGroup {
    ///Unknown usage
    pub header: u32,
    pub entries: [Hue, ..8]
}

impl HueGroup {

    pub fn new(header: u32, entries: [Hue, ..8]) -> HueGroup {
        HueGroup {
            header: header,
            entries: entries
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut writer = MemWriter::new();
        writer.write_le_u32(self.header).ok().expect(MEMWRITER_ERROR);
        for hue in self.entries.iter() {
            writer.write(hue.serialize().as_slice()).ok().expect(MEMWRITER_ERROR);
        }
        writer.into_inner()
    }
}

//A hue_entry is (32 * 2) + 2 + 2 + 20 bytes = 88 bytes
const ENTRY_SIZE: uint = 88;
//8 entries to a group, plus a 4 byte header. 708 bytes.
const GROUP_SIZE: uint = (ENTRY_SIZE * 8) + 4;

pub struct HueReader {
    data_reader: File
}

impl HueReader {
    pub fn new(hues_path: &Path) -> IoResult<HueReader> {
        let data_reader = try!(File::open_mode(hues_path, Open, Read));

        Ok(HueReader {
            data_reader: data_reader
        })
    }

    /**
     * Read the given indexed group
     */
    pub fn read_hue_group(&mut self, id: u32) -> IoResult<HueGroup> {
        try!(self.data_reader.seek((id as uint * GROUP_SIZE) as i64, SeekSet));

        let header = try!(self.data_reader.read_le_u32());

        let entries: [Hue, ..8] = [
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

    fn read_hue(&mut self) -> IoResult<Hue> {
        let mut color_table = [0u16, ..32];
        for idx in range(0u, 32) {
            color_table[idx] = try!(self.data_reader.read_le_u16());
        }

        let table_start = try!(self.data_reader.read_le_u16());
        let table_end = try!(self.data_reader.read_le_u16());

        let raw_name = try!(self.data_reader.read_exact(20));

        //Slice it down into a normal string size
        let trimmed_name: Vec<u8> = raw_name.into_iter().take_while(|&element| element != 0).collect();
        
        let maybe_name = trimmed_name.into_ascii_opt();
        
        let name = match maybe_name {
            Some(name) => name.into_string(),
            None => "Error".to_string()
        };

        Ok(Hue::new(
            color_table,
            table_start,
            table_end,
            name
        ))

    }
}
