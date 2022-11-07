use serde::{Serialize, Deserialize};
use crate::tile::TriangulizedTile;
use crate::building_representations::triangulized_walls::TrianguizedWalls;
use crate::building_representations::polygon_walls::PolygonWalls;
use crate::request_for_isolation::Request;
use crate::general_geometry::Polygon;
use crate::building_representations::converters;

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub building: TrianguizedWalls,
    pub tiles: Vec<TriangulizedTile>,
}


pub fn generate_plan(request: Request) -> Plan {
    Plan {
        building: triangulized_wall_building_from_request(request),
        tiles: vec![]
    }
}

fn triangulized_wall_building_from_request(request: Request) -> TrianguizedWalls {
    let mut walls: Vec<Polygon> = vec![];
    
    let poly_walls = PolygonWalls::new(walls);
    converters::polygon_walls_to_triangulized_walls(poly_walls)
}