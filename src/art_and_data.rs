use art::{ArtReader, Tile, Static};
use tiledata::{TileDataReader, MapTileData, StaticTileData};
use std::io::Result;
use std::path::Path;

pub struct ArtAndDataReader {
    art_reader: ArtReader,
    data_reader: TileDataReader
}

impl ArtAndDataReader {
    pub fn new(index_path: &Path, mul_path: &Path, data_path: &Path) -> Result<ArtAndDataReader> {
        let art_reader = try!(ArtReader::new(index_path, mul_path));
        let data_reader = try!(TileDataReader::new(data_path));
        Ok(ArtAndDataReader {
            art_reader: art_reader,
            data_reader: data_reader
        })
    }

    pub fn read_tile(&mut self, id: u32) -> (Result<Tile>, Result<MapTileData>) {
        let tile = self.art_reader.read_tile(id);
        let data = self.data_reader.read_map_tile_data(id);
        (tile, data)
    }

    pub fn read_static(&mut self, id: u32)-> (Result<Static>, Result<StaticTileData>) {
        let tile = self.art_reader.read_static(id);
        let data = self.data_reader.read_static_tile_data(id);
        (tile, data)
    }
}
