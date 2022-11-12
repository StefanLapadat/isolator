use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle, Polygon};
use crate::triangulation::PolygonForTriangulation;

pub struct Tile {
    polygon: Polygon,
    normal: Point,
    width: f64
}

impl Tile {
    pub fn new(polygon: Polygon, normal: Point, width: f64) -> Tile {
        Tile {
            polygon, normal: polygon.normal(), width
        }
    }

    fn polygon(&self) -> &Polygon {
        &self.polygon
    }

    fn normal(&self) -> &Point {
        &self.normal
    }

    fn width(&self) -> f64 {
        self.width
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

    let inc = tile.normal().normalize().multiply(tile.width());

    let base_polygon = tile.polygon().clone();

    res.push(base_polygon.translate(&inc));
    res.append(&mut base_polygon.rim_extrusion(&inc));
    res.append(&mut base_polygon.holes_extrusion(&inc));


    res.push(base_polygon);

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

