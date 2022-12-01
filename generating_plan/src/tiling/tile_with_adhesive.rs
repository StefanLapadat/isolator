use super::{Tile, TriangulizedTiles};
use serde::{Serialize, Deserialize};
use general_geometry::{Point};

#[derive(Debug)]
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

    pub fn surface_rim(&self) -> &Vec<Point> {
        self.styro_tile.surface_polygon().rim()
    }

    pub fn base_rim(&self) -> &Vec<Point> {
        self.adhesive_tile.base_polygon().rim()
    }

    pub fn styro_tile(&self) -> &Tile {
        &self.styro_tile
    }

    pub fn adhesive_tile(&self) -> &Tile {
        &self.adhesive_tile
    }

    pub fn average_point(&self) -> Point {
        let mut p2 = Point::new(0., 0., 0.);
        let mut i = 0;
        while i < self.surface_rim().len() {
            p2 = p2.add(&self.surface_rim()[i]);
            i+=1;
        }
        
        p2.divide(self.surface_rim().len() as f64)
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
            triangulized_tiles: TriangulizedTiles::from_tiles(&styro_tiles), 
            triangulized_adhesive: TriangulizedTiles::from_tiles(&adhesive_tiles),
        }
    }

    pub fn triangulized_tiles(&self) -> &TriangulizedTiles {
        &self.triangulized_tiles       
    }

    pub fn triangulized_adhesive(&self) -> &TriangulizedTiles {
        &self.triangulized_adhesive       
    }
}