use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle, Polygon, PolygonPointsOnSides};
use crate::triangulation::PolygonForTriangulation;

#[derive(Debug)]
pub struct Tile {
    base_polygon: PolygonPointsOnSides,
    surface_polygon: PolygonPointsOnSides,
}

impl Tile {
    pub fn new(base_polygon: PolygonPointsOnSides, surface_polygon: PolygonPointsOnSides) -> Tile {
        Tile {
            base_polygon, surface_polygon
        }
    }

    fn base_polygon(&self) -> &PolygonPointsOnSides {
        &self.base_polygon
    }

    fn surface_polygon(&self) -> &PolygonPointsOnSides {
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
        i+=1;
    }

    res.push(Polygon::from_polygon_points_on_sides(tile.base_polygon().clone()));
    res.push(Polygon::from_polygon_points_on_sides(tile.surface_polygon().clone()));
    
    Polygon::merge_multiple_polygons(&res)
}

fn parallel_rims_to_polygons(base_rim: &Vec<Point>, surface_rim: &Vec<Point>) -> Vec<Polygon> {
    let mut res = vec![];

    let mut i: usize = 0;
    let rl = base_rim.len();

    while i < rl {
        if Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            i+=1; 
            continue;
        }

        if !Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && !Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            res.push(Polygon::new(vec![base_rim[i].clone(), base_rim[(i+1)%rl].clone(), surface_rim[(i+1)%rl].clone(), surface_rim[i].clone()], vec![]));
        }

        if Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && !Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            res.push(Polygon::new(vec![base_rim[i].clone(), surface_rim[(i+1)%rl].clone(), surface_rim[i].clone()], vec![]));
        }

        if !Point::are_points_simmilar(&base_rim[i], &base_rim[(i+1)%rl]) && Point::are_points_simmilar(&surface_rim[i], &surface_rim[(i+1)%rl]) {
            res.push(Polygon::new(vec![base_rim[i].clone(), base_rim[(i+1)%rl].clone(), surface_rim[(i+1)%rl].clone()], vec![]));
        }

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
