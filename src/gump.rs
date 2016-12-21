use mul_reader::MulReader;
use color::Color16;
use std::io::{Result, Error, ErrorKind, Cursor, SeekFrom, Seek, Write, Read};
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};
use std::path::Path;

#[cfg(feature = "use-sdl2")]
use sdl2::surface::Surface;
#[cfg(feature = "use-sdl2")]
use sdl2::pixels::PixelFormatEnum;
#[cfg(feature = "use-sdl2")]
use utils::{SURFACE_ERROR};

#[derive(Clone, Copy)]
pub struct GumpPair {
    color: Color16,
    count: u16
}

#[derive(Clone)]
pub struct Gump {
    width: u16,
    height: u16,
    data: Vec<GumpPair>
}

impl Gump {

    #[cfg(feature = "use-sdl2")]
    pub fn to_surface(&self) -> Surface {
        let mut surface = Surface::new(self.width as u32, self.height as u32, PixelFormatEnum::RGBA8888).expect(SURFACE_ERROR);
        /*surface.with_lock_mut(|bitmap| {
            let mut read_idx = 0;

            for y in 0..44 {

                let slice_width = if y >= 22 {
                    (44 - y) * 2
                } else {
                    (y + 1) * 2
                };

                let offset_left = 22 - (slice_width / 2);
                for pixel_idx in 0..slice_width {
                    let x = offset_left + pixel_idx;
                    let (r, g, b, a) = self.image_data[read_idx].to_rgba();
                    let target = ((y * 44) + x) * 4;
                    bitmap[target] = a;
                    bitmap[target + 1] = b;
                    bitmap[target + 2] = g;
                    bitmap[target + 3] = r;
                    read_idx += 1;
                }
            };
        });*/
        surface
    }
}

pub struct GumpReader<T: Read + Seek> {
    mul_reader: MulReader<T>
}

impl GumpReader<File> {

    pub fn new(index_path: &Path, mul_path: &Path) -> Result<GumpReader<File>> {
        let mul_reader = try!(MulReader::new(index_path, mul_path));
        Ok(GumpReader {
            mul_reader: mul_reader
        })
    }
}

impl<T: Read + Seek> GumpReader<T> {

    pub fn read_gump(&mut self, index: u32) -> Result<Gump> {
        let raw = try!(self.mul_reader.read(index));
        let mut output = vec![];
        let len = raw.data.len();
        assert!(len % 4 == 0);
        let mut reader = Cursor::new(raw.data);
        for _i in 0..(len / 4) {
            let color = try!(reader.read_u16::<LittleEndian>());
            let count = try!(reader.read_u16::<LittleEndian>());
            output.push(GumpPair {
                color: color,
                count: count
            });
        };
        Ok(Gump {
            height: raw.opt1,
            width: raw.opt2,
            data: output
        })
    }
}
