use serde::{Serialize, Deserialize};
use crate::building_representations::triangulized_walls::TrianguizedWalls;
use crate::building_representations::polygon_walls::PolygonWalls;
use crate::request_for_isolation::Request;
use crate::general_geometry::{Polygon};
use crate::building_representations::converters;
use crate::tile::{Tile, TriangulizedTiles};

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub building: TrianguizedWalls,
    pub tiles: TriangulizedTiles,
}

pub fn generate_plan(request: &Request) -> Plan {
    Plan {
        building: triangulized_wall_building_from_request(request),
        tiles: triangulized_tiles_from_request(request)
    }
}

fn triangulized_wall_building_from_request(request: &Request) -> TrianguizedWalls {
    let mut walls: Vec<Polygon> = vec![];

    for wall in request.data() {
        walls.push(wall.polygon().clone())
    }
    
    let poly_walls = PolygonWalls::new(walls);
    converters::polygon_walls_to_triangulized_walls(poly_walls)
}

fn triangulized_tiles_from_request(request: &Request) -> TriangulizedTiles {
    let mut tiles: Vec<Tile> = vec![];
    for wall in request.data() {
        match wall.isolation() {
            Option::Some(detail) => {
                tiles.push(Tile::new(wall.polygon().clone(),wall.polygon_normal().clone(), detail.width()))
            },
            Option::None => {

            }
        }
    }

    TriangulizedTiles::from_tiles(tiles)
}
