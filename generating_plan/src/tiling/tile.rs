use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle, Polygon, PolygonPointsOnSides, Simmilar};
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

    fn width(&self) -> f64 {
        let base_data = self.base_polygon().to_polygon();
        let surface_data = self.surface_polygon().to_polygon();

        let p1 = Polygon::new(base_data.0, base_data.1).plane();
        let p2 = Polygon::new(surface_data.0, surface_data.1).plane();

        println!("{:?}", p1);
        println!("{:?}", p2);
        

        (p1.d() - p2.d()).abs() / p1.normal_vector().modulo()
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


fn split_into_tiles(tile: &Tile, unit_tile: &UnitTile) -> Option<Vec<Tile>> {
    // So how exactly could I do this?? 
    // And is it sensible to implement this first without glue, and then to somehow try to add glue into the story?? 
    // How do I even specify where should glue be? 
    // I am not sure if it will be that easy to just shove glue into the solution, to hack it.. but anyway, as always, I will 
    // of course not give any though to this problem, but, for the sake of getting the feel for the problem, will just strat hacking stuff. 
    // ..
    // Weeell, this doesn't seem to be that hard?? 
    // First of all I need unit tile to have one dimension equal to tile width? I could start philosophy on how it doesn't have to be like that, but.. 
    // KISS :D 

    None
}

pub fn are_tile_and_unit_tile_compatible(tile: &Tile, unit_tile: &UnitTile) -> bool {
    let tile_width = tile.width();
    println!("{}", tile_width);

    unit_tile.d.x.simmilar_to(tile_width, 0.0001) || 
    unit_tile.d.y.simmilar_to(tile_width, 0.0001) || 
    unit_tile.d.z.simmilar_to(tile_width, 0.0001)
}

pub struct UnitTile {
    d: Point
}

impl UnitTile {
    pub fn new(d: Point) -> Option<Self> {
        if !d.same_oktant(&Point::new(1., 1., 1.)) || d.close_to_zero() {
            None
        } else {
            Some(UnitTile { d })
        }
    }
}
