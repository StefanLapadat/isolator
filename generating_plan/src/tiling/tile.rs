use serde::{Serialize, Deserialize};
use crate::general_geometry::polygon2d::Rectangle;
use crate::general_geometry::{Point, Triangle, Polygon, PolygonPointsOnSides, Simmilar, Polygon2D, Point2D};
use crate::triangulation::PolygonForTriangulation;

#[derive(Debug, Clone)]
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

        // println!("{:?} {:?}", self.base_polygon(), self.surface_polygon());

        let base_data = self.base_polygon().to_polygon();
        let surface_data = self.surface_polygon().to_polygon();

        let p1 = Polygon::new(base_data.0, base_data.1).plane();
        let p2 = Polygon::new(surface_data.0, surface_data.1).plane();

        // println!("{:?} {:?}", p1, p2);

        (p1.d() - p2.d()).abs() / p1.normal_vector().modulo()
    }

    fn width_vec(&self) -> Point {
        let p_args = self.base_polygon().to_polygon();
        Polygon::new(p_args.0, p_args.1).normal().normalize().multiply(self.width())
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

pub fn split_into_tiles(tile: &Tile, unit_tile: &UnitTile) -> Option<Vec<Tile>> {
    let (unit_tile_width, unit_tile_height);
    match are_tile_and_unit_tile_compatible(tile, unit_tile) {
        Some((unit_tile_w, unit_tile_h)) => {
            (unit_tile_width, unit_tile_height) = (unit_tile_w, unit_tile_h);
        }, None => {
            return None;
        }
    }

    let args_base= tile.base_polygon.to_polygon();
    let args_surface = tile.surface_polygon.to_polygon();

    let base = Polygon::new(args_base.0, args_base.1);
    let surface = Polygon::new(args_surface.0, args_surface.1);
    
    let system = base.coordinate_system_xy_parallel_to_self();
    
    let base_2d = base.to_2d(&system);
    let surface_2d = surface.to_2d(&system);

    let base_union_box = Polygon2D::union_box_many(vec![base_2d, surface_2d]);
    let base_splitted = split_2d_surrounding_boxes(&base_union_box, unit_tile_width, unit_tile_height);
    // let base_splitted = vec![base_union_box];

    let base_union_boxes_3d = base_splitted.iter().map(|b| b.to_3d(&system)).collect::<Vec<_>>();
    let surface_union_boxes_3d = base_union_boxes_3d.iter().map(|b| b.translate(&tile.width_vec())).collect::<Vec<_>>();

    let base_comps = base_union_boxes_3d.into_iter().map(|b| b.destruct_to_components()).collect::<Vec<_>>();
    let surface_comps = surface_union_boxes_3d.into_iter().map(|b| b.destruct_to_components()).collect::<Vec<_>>();

    if base_comps.len() != surface_comps.len() {
        panic!("Somethings fishy here");
    }

    let it = base_comps.iter().zip(surface_comps.iter());
    let mut res = vec![];
    for (i, (x, y)) in it.enumerate() {
        let t = Tile::new(PolygonPointsOnSides::new(x.0.clone(), x.1.clone()),
        PolygonPointsOnSides::new(y.0.clone(), y.1.clone()));
        res.push(t);
    }

    // Option::Some(vec![tile.clone()])
    Option::Some(res)
}

fn split_2d_surrounding_boxes(r: &Rectangle, unit_tile_width: f64, unit_tile_height: f64) -> Vec<Rectangle> {
    let mut res = vec![];
    println!("DUzine sirine {:?} {:?} {:?} {:?}", r.width(), r.height(), unit_tile_width, unit_tile_height);


    let (start_x, start_y, end_x, end_y) = (r.low_left().x(), r.low_left().y(), r.up_right().x(), r.up_right().y());
    println!("Poceci krajevi {:?} {:?} {:?} {:?}",start_x, start_y, end_x, end_y);

    let mut tmp_x = start_x;
    while !tmp_x.simmilar_to(end_x, 0.001) && tmp_x < end_x {
        let mut tmp_y = start_y;

        while !tmp_y.simmilar_to(end_y, 0.001) && tmp_y < end_y {
            println!("**** {} {}", tmp_x, tmp_y);
            // std::thread::sleep(std::time::Duration::new(0, 1_00_000_000));
            res.push(Rectangle::new(Point2D::new(tmp_x, tmp_y), Point2D::new(tmp_x + unit_tile_width, tmp_y + unit_tile_height)));
            tmp_y += unit_tile_height;
        }
        tmp_x += unit_tile_width;
        println!("tmp_x je {}", tmp_x);
    }

    res
}

pub fn are_tile_and_unit_tile_compatible(tile: &Tile, unit_tile: &UnitTile) -> Option<(f64, f64)> {
    let tile_width = tile.width();

    // println!("{:?} {:?}", tile_width, unit_tile.d);

    if unit_tile.d.x.simmilar_to(tile_width, 0.0001) {
        return Some((unit_tile.d.y, unit_tile.d.z));
    }

    if unit_tile.d.y.simmilar_to(tile_width, 0.0001) {
        return Some((unit_tile.d.x, unit_tile.d.z));
    }

    if unit_tile.d.z.simmilar_to(tile_width, 0.0001) {
        return Some((unit_tile.d.x, unit_tile.d.y))
    }

    None
}

#[derive(Debug, Serialize, Deserialize)]
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
