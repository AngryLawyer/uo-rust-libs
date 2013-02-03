use mul_reader;

pub struct HueEntry {
    color_table: ~[u16],
    table_start: u16,
    table_end: u16,
    name: ~str
}

pub struct HueGroup {
    header: i32,
    entries: ~[HueEntry]
}

pub struct HueReader {
    data_reader: io::Reader
}

//8 entries to a group, plus a 4 byte header. 708 bytes.
const GROUP_SIZE = 708;
//A hue_entry is (32 * 2) + 2 + 2 + 20 bytes = 88 bytes
const ENTRY_SIZE = 88;

impl HueReader {
    fn read_hue_group(&self, id: uint) -> option::Option<HueGroup> {
        self.data_reader.seek((id * GROUP_SIZE) as int, io::SeekSet);
        let group_reader = self.data_reader as io::ReaderUtil;

        let header: i32 = group_reader.read_le_i32();
        let mut entries: ~[HueEntry] = ~[];
        for uint::range(0, 8) |_entry| {
            let mut color_table: ~[u16] = ~[];
            for uint::range(0, 32) |_table_idx| {
                color_table.push(group_reader.read_le_u16())
            }
            entries.push(HueEntry{
                color_table: color_table,
                table_start: group_reader.read_le_u16(),
                table_end: group_reader.read_le_u16(),
                string: /* READ 20 bytes */ 
            });
        }
        option::Some(HueGroup {
            header: header,
            entries: entries
        })
    }
}

pub fn HueReader(hues_path: &path::Path) -> result::Result<HueReader, ~str> {
    match io::file_reader(&hues_path) {
        result::Ok(data_reader) => {
            result::Ok(HueReader {
                data_reader: data_reader
            })
        },
        result::Err(error_message) => {
            result::Err(error_message)
        }
    }
}
