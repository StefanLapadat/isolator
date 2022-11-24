use super::{Tile, TriangulizedTiles};
use serde::{Serialize, Deserialize};

pub struct TileWithAdhesive {
    styro_tile: Tile,
    adhesive_tile: Tile
}

impl TileWithAdhesive {
    pub fn new(styro_tile: Tile, adhesive_tile: Tile) -> Self {
        Self {
            styro_tile, adhesive_tile
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriangulizedTilesWithAdhesive {
    triangulized_tiles: TriangulizedTiles,
    triangulized_adhesive: TriangulizedTiles
}

impl TriangulizedTilesWithAdhesive {
    pub fn from_tiles(tiles: Vec<TileWithAdhesive>) -> TriangulizedTilesWithAdhesive {
        let styro_tiles = tiles.iter().map(|tile| tile.styro_tile.clone()).collect::<Vec<_>>();
        let adhesive_tiles = tiles.iter().map(|tile| tile.adhesive_tile.clone()).collect::<Vec<_>>();

        TriangulizedTilesWithAdhesive { 
            triangulized_tiles: TriangulizedTiles::from_tiles(styro_tiles), 
            triangulized_adhesive: TriangulizedTiles::from_tiles(adhesive_tiles),
        }
    }

    pub fn triangulized_tiles(&self) -> &TriangulizedTiles {
        &self.triangulized_tiles       
    }

    pub fn triangulized_adhesive(&self) -> &TriangulizedTiles {
        &self.triangulized_adhesive       
    }
}