use serde::{Serialize, Deserialize};
use crate::building_representations::triangulized_walls::TrianguizedWalls;
use crate::building_representations::polygon_walls::PolygonWalls;
use crate::request_for_isolation::Request;
use crate::general_geometry::{Polygon};
use crate::building_representations::converters;
use crate::tiling::{Tile, TriangulizedTiles};

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub building: TrianguizedWalls,
    pub tiles: TriangulizedTiles,
}

pub fn generate_plan(request: &Request) -> Plan {
    let building: PolygonWalls = polygon_walls_from_request(request);

    Plan {
        building: converters::polygon_walls_to_triangulized_walls(building),
        tiles: triangulized_tiles(get_tiling(request))
    }
}

fn polygon_walls_from_request(request: &Request) -> PolygonWalls {
    let mut walls: Vec<Polygon> = vec![];

    for wall in request.data() {
        walls.push(wall.polygon().clone())
    }
    
    PolygonWalls::new(walls)
}

fn triangulized_tiles(tiles: Vec<Tile>) -> TriangulizedTiles {
    TriangulizedTiles::from_tiles(tiles)
}

fn get_tiling(request: &Request) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];
    let mut i: usize = 0;

    let building = polygon_walls_from_request(request);

    while i < request.data().len() {
        match request.data()[i].isolation() {
            Option::Some(detail) => {
                tiles.append(&mut get_tiles_from_wall_in_building(i, request));
            },
            Option::None => {

            }
        }

        i+=1;
    }
    tiles
}

fn get_tiles_from_wall_in_building(ind: usize, request: &Request) -> Vec<Tile> {
    let res: Vec<Tile> = vec![];

    let mut i = 0;
    let rim = request.data()[i].polygon().rim();
    let rl = rim.len();

    while i < rl {
        let tmp = &rim[i];
        let next = &rim[(i+1)%rl];

        i+=1;
    }

    res
}
