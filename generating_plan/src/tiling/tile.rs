use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle, Polygon};
use crate::triangulation::PolygonForTriangulation;

pub struct Tile {
    base_polygon: Polygon,
    surface_polygon: Polygon,
}

impl Tile {
    pub fn new(base_polygon: Polygon, surface_polygon: Polygon) -> Tile {
        Tile {
            base_polygon, surface_polygon
        }
    }

    fn base_polygon(&self) -> &Polygon {
        &self.base_polygon
    }

    fn surface_polygon(&self) -> &Polygon {
        &self.surface_polygon
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulizedTile {
    triangles: Vec<Triangle>
}

impl TriangulizedTile {
    fn new(triangles: Vec<Triangle>) -> Self {
        Self {
            triangles
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulizedTiles {
    tiles: Vec<TriangulizedTile>,
    wireframe: Vec<Vec<Point>>
}

impl TriangulizedTiles {
    pub fn from_tiles(tiles: Vec<Tile>) -> TriangulizedTiles {
        let mut triangulized_tiles: Vec<TriangulizedTile> = vec![];
        let mut wireframe: Vec<Vec<Point>> = vec![];

        for tile in tiles {
            let mut triangulized_tile_with_wireframe = tile_to_triangulized_tile(&tile);
            triangulized_tiles.push(triangulized_tile_with_wireframe.0);
            wireframe.append(&mut triangulized_tile_with_wireframe.1);
        }

        TriangulizedTiles {
            tiles: triangulized_tiles,
            wireframe: wireframe
        }
    }
}

fn tile_to_polygons(tile: &Tile) -> Vec<Polygon> {
    let mut res = vec![];

    res.append(&mut parallel_rims_to_polygons(tile.base_polygon().rim(), tile.surface_polygon().rim()));
    let mut i = 0;
    while i < tile.base_polygon().holes().len() {
        res.append(&mut parallel_rims_to_polygons(&tile.base_polygon().holes()[i], &tile.surface_polygon().holes()[i]));
    }

    res
}

fn parallel_rims_to_polygons(base_rim: &Vec<Point>, surface_rim: &Vec<Point>) -> Vec<Polygon> {
    let mut res = vec![];

    let mut i: usize = 0;
    let rl = base_rim.len();

    while i < rl {
        let tmp_b = base_rim[i].clone();
        let next_b = base_rim[(i+1)%rl].clone();
        let tmp_s = surface_rim[i].clone();
        let next_s = surface_rim[(i+1)%rl].clone();

        res.push(Polygon::new(vec![base_rim[i].clone(), base_rim[(i+1)%rl].clone(), surface_rim[(i+1)%rl].clone(), surface_rim[i].clone()], vec![]));

        i+=1;
    }

    res
}

fn tile_to_triangulized_tile(tile: &Tile) -> (TriangulizedTile, Vec<Vec<Point>>) {
    let mut triangles: Vec<Triangle> = vec![];
    let mut wireframe: Vec<Vec<Point>> = vec![];

    for side in tile_to_polygons(tile) {
        for triangle in PolygonForTriangulation::from_polygon(&side).triangulate_3d() {
            triangles.push(triangle)
        }

        wireframe.append(&mut side.wireframe());
    }

    (TriangulizedTile::new(triangles), wireframe)
}

