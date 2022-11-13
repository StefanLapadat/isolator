use serde::{Serialize, Deserialize};
use crate::building_representations::triangulized_walls::TrianguizedWalls;
use crate::building_representations::polygon_walls::PolygonWalls;
use crate::request_for_isolation::Request;
use crate::general_geometry::{Polygon, Point, PolygonPointsOnSides};
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
                tiles.append(&mut get_tiles_from_wall_in_building(i, request, 0.2));
            },
            Option::None => {

            }
        }

        i+=1;
    }

    tiles
}

fn get_tiles_from_wall_in_building(ind: usize, request: &Request, isolation_width: f64) -> Vec<Tile> {
    let mut res: Vec<Tile> = vec![];

    let borders = get_borders_for_wall(ind, request);
    let wall = request.data()[ind].polygon();

    let mut base_rim: Vec<Point> = vec![];
    let mut surface_rim: Vec<Point> = vec![];

    for border in borders {
        match border.wall_ind {
            Some(val) => {},
            None => {
                surface_rim.push(border.pointA.add(&wall.normal().normalize().multiply(isolation_width)));
                surface_rim.push(border.pointB.add(&wall.normal().normalize().multiply(isolation_width)));
                base_rim.push(border.pointA);
                base_rim.push(border.pointB);
            }
        }
    }

    res.push(Tile::new(PolygonPointsOnSides::new(base_rim, vec![]), PolygonPointsOnSides::new(surface_rim, vec![])));
    
    res
}

fn get_borders_for_wall(ind: usize, request: &Request) -> Vec<Border> {
    let mut res = vec![];

    let mut i = 0;
    let rim = request.data()[ind].polygon().rim();
    let rl = rim.len();

    while i < rl {
        let tmp = &rim[i];
        let next = &rim[(i+1)%rl];
        res.push(Border { 
            pointA: tmp.clone(),
            pointB: next.clone(),
            wall_ind: None
        });

        i+=1;
    }

    res
}

struct Border {
    pointA: Point,
    pointB: Point, 
    wall_ind: Option<usize>
}
