use serde::{Serialize, Deserialize};
use general_geometry::polygon2d::Rectangle;
use general_geometry::{Point, Triangle, Polygon, PolygonPointsOnSides, Simmilar, Polygon2D, Point2D, Plane};
use crate::triangulation::PolygonForTriangulation;

#[derive(Debug, Clone)]
pub struct Tile {
    base_polygon: PolygonPointsOnSides,
    surface_polygon: PolygonPointsOnSides,
}

impl Tile {
    pub fn new(base_polygon: PolygonPointsOnSides, surface_polygon: PolygonPointsOnSides) -> Tile {
        let (base_polygon, surface_polygon) = Self::remove_duplicate_pairs_from_base_and_surface_polygons(&base_polygon, &surface_polygon);

        Tile {
            base_polygon, surface_polygon
        }
    }

    fn remove_duplicate_pairs_from_base_and_surface_polygons(base_polygon: &PolygonPointsOnSides, surface_polygon: &PolygonPointsOnSides) -> (PolygonPointsOnSides, PolygonPointsOnSides) {
        let mut i = 0;
        let (mut base_polygon_res, mut surface_polygon_res) = (vec![], vec![]);
        
        let rl = base_polygon.rim().len();

        while i < rl {
            if !Point::are_points_simmilar(&base_polygon.rim()[i], &base_polygon.rim()[(i+1)%rl]) || 
                !Point::are_points_simmilar(&surface_polygon.rim()[i], &surface_polygon.rim()[(i+1)%rl])  {
                    base_polygon_res.push(base_polygon.rim()[i].clone());
                    surface_polygon_res.push(surface_polygon.rim()[i].clone());
            }
            i+=1;
        }

        (PolygonPointsOnSides::new(base_polygon_res, vec![]), PolygonPointsOnSides::new(surface_polygon_res, vec![]))
    }

    pub fn from_base_polygon_and_width(base: PolygonPointsOnSides, width: Point) -> Self {
        let surface = base.translate(&width);

        Self::new(base, surface)
    }

    pub fn split_surface(&self, percent_width_from_base_to_surface: f64) -> PolygonPointsOnSides {
        let res_rim = Self::split_surface_rim(self.base_polygon.rim(), self.surface_polygon.rim(), percent_width_from_base_to_surface);
        let zipped_holes = self.base_polygon.holes().iter().zip(self.surface_polygon.holes().iter());
        let res_holes = zipped_holes.map(|hole_pair| Self::split_surface_rim(hole_pair.0, hole_pair.1, percent_width_from_base_to_surface)).collect::<Vec<_>>();

        PolygonPointsOnSides::new(res_rim, res_holes)
    }

    fn split_surface_rim(rim_base: &Vec<Point>, rim_surface: &Vec<Point>, percent_width_from_base_to_surface: f64) -> Vec<Point> {
        let mut res = vec![];
        let mut i = 0;
        while i < rim_base.len() {
            res.push(rim_base[i].add(&rim_surface[i].subtract(&rim_base[i]).multiply(percent_width_from_base_to_surface)));
            i+=1;
        }
        res
    }

    pub fn base_polygon(&self) -> &PolygonPointsOnSides {
        &self.base_polygon
    }

    pub fn surface_polygon(&self) -> &PolygonPointsOnSides {
        &self.surface_polygon
    }

    pub fn width(&self) -> f64 {
        let base_data = self.base_polygon().to_polygon();
        let surface_data = self.surface_polygon().to_polygon();

        let p1 = Polygon::new(base_data.0, base_data.1).plane();
        let p2 = Polygon::new(surface_data.0, surface_data.1).plane();

        let (p1, p2) = Plane::make_parallel_planes_have_same_params(&p1, &p2);

        (p1.d() - p2.d()).abs() / p1.normal_vector().modulo()
    }

    pub fn width_vec(&self) -> Point {
        let p_args = self.base_polygon().to_polygon();
        Polygon::new(p_args.0, p_args.1).normal().normalize().multiply(self.width())
    }

    pub fn translate(&self, inc: &Point) -> Tile {    
        Tile::new(self.base_polygon().translate(inc), self.surface_polygon().translate(inc))
    }

    pub fn average_point(&self) -> Point {
        let mut p2 = Point::new(0., 0., 0.);
        let mut i = 0;
        while i < self.surface_polygon.rim().len() {
            p2 = p2.add(&self.surface_polygon.rim()[i]);
            i+=1;
        }
        
        p2.divide(self.surface_polygon.rim().len() as f64)
    }


    fn to_polygons(&self) -> Vec<Polygon> {
        let mut res = vec![];
    
        res.append(&mut parallel_rims_to_polygons(self.base_polygon().rim(), self.surface_polygon().rim()));
    
        let mut i = 0;
        while i < self.base_polygon().holes().len() {
            res.append(&mut parallel_rims_to_polygons(&self.base_polygon().holes()[i], &self.surface_polygon().holes()[i]));
            i+=1;
        }
    
        res.push(Polygon::from_polygon_points_on_sides(self.base_polygon().clone()));
        res.push(Polygon::from_polygon_points_on_sides(self.surface_polygon().clone()));
    
        res
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

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulizedTiles {
    tiles: Vec<TriangulizedTile>,
    wireframe: Vec<Vec<Point>>
}

impl TriangulizedTiles {
    pub fn from_tiles(tiles: &Vec<Tile>) -> TriangulizedTiles {
        let mut triangulized_tiles: Vec<TriangulizedTile> = vec![];
        let mut wireframe: Vec<Vec<Point>> = vec![];

        for tile in tiles {
            let mut triangulized_tile_with_wireframe = tile_to_triangulized_tile(tile);
            triangulized_tiles.push(triangulized_tile_with_wireframe.0);
            wireframe.append(&mut triangulized_tile_with_wireframe.1);
        }

        TriangulizedTiles {
            tiles: triangulized_tiles,
            wireframe
        }
    }

    pub fn tiles(&self) -> &Vec<TriangulizedTile> {
        &self.tiles
    }

    pub fn wireframe(&self) -> &Vec<Vec<Point>> {
        &self.wireframe
    }
}

fn parallel_rims_to_polygons(base_rim: &Vec<Point>, surface_rim: &Vec<Point>) -> Vec<Polygon> {
    let mut res = vec![];

    let mut i: usize = 0;
    let rl = base_rim.len();

    while i < rl {
        res.push(base_seg_and_surface_seg_to_polygon(&base_rim[i], &base_rim[(i+1)%rl], &surface_rim[i], &surface_rim[(i+1)%rl]));
        i+=1;
    }

    res
}

fn base_seg_and_surface_seg_to_polygon(base_start: &Point, base_end: &Point, surface_start: &Point, surface_end: &Point) -> Polygon {
    let (b0, b1, s0, s1) = (base_start, base_end, surface_start, surface_end);

    if Point::are_points_simmilar(b0, b1) {
        return Polygon::new(vec![b0.clone(), s1.clone(), s0.clone()], vec![]);
    } 
    
    if Point::are_points_simmilar(s0, s1) {
        return Polygon::new(vec![b0.clone(), b1.clone(), s1.clone()], vec![]);
    } 
    
    return Polygon::new(vec![b0.clone(), b1.clone(), s1.clone(), s0.clone()], vec![]);
}

pub fn tile_to_triangulized_tile(tile: &Tile) -> (TriangulizedTile, Vec<Vec<Point>>) {
    let mut triangles: Vec<Triangle> = vec![];
    let mut wireframe: Vec<Vec<Point>> = vec![];

    for side in tile.to_polygons() {
        triangles.append(&mut PolygonForTriangulation::from_polygon(&side).triangulate_3d());
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

    let base= tile.base_polygon.to_polygon_true_type();
    let surface = tile.surface_polygon.to_polygon_true_type();

    let system = base.coordinate_system_xy_parallel_to_self();

    let base_2d = base.to_2d(&system);
    let surface_2d = surface.to_2d(&system);

    let base_and_surface_union_box = Polygon2D::union_box_many(vec![base_2d.clone(), surface_2d.clone()]);
    let union_box_splitted = split_2d_surrounding_boxes(&base_and_surface_union_box, unit_tile_width, unit_tile_height);

    let original_distance_from_origin_base = base.distance_from_origin();
    let original_distance_from_origin_surface = surface.distance_from_origin();

    let union_box_mini_tiles_2d_boxes = union_box_splitted.iter().map(|b| b.to_polygon_2d());

    let base_mini_tiles_2d_polys = union_box_mini_tiles_2d_boxes.clone().map(|t| t.intersection(&base_2d)).filter(|intersections| intersections.len() > 0).collect::<Vec<_>>();
    let surface_mini_tiles_2d_polys = union_box_mini_tiles_2d_boxes.map(|t| t.intersection(&surface_2d)).filter(|intersections| intersections.len() > 0).collect::<Vec<_>>();

    let base_mini_tiles_3d = base_mini_tiles_2d_polys.iter().map(|t| t[0].to_3d(&system, &original_distance_from_origin_base)).collect::<Vec<_>>();
    let surface_mini_tiles_3d = surface_mini_tiles_2d_polys.iter().map(|t| t[0].to_3d(&system, &original_distance_from_origin_surface)).collect::<Vec<_>>();

    let base_comps = base_mini_tiles_3d.into_iter().map(|b| b.destruct_to_components()).collect::<Vec<_>>();
    let surface_comps = surface_mini_tiles_3d.into_iter().map(|b| b.destruct_to_components()).collect::<Vec<_>>();

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

    Option::Some(res)
}

fn split_2d_surrounding_boxes(r: &Rectangle, unit_tile_width: f64, unit_tile_height: f64) -> Vec<Rectangle> {
    let mut res = vec![];

    let (start_x, start_y, end_x, end_y) = (r.low_left().x(), r.low_left().y(), r.up_right().x(), r.up_right().y());

    let mut tmp_x = start_x;
    while !tmp_x.simmilar_to(end_x, 0.001) && tmp_x < end_x {
        let mut tmp_y = start_y;

        while !tmp_y.simmilar_to(end_y, 0.001) && tmp_y < end_y {
            res.push(Rectangle::new(Point2D::new(tmp_x, tmp_y), 
            Point2D::new((tmp_x + unit_tile_width).min(end_x), (tmp_y + unit_tile_height).min(end_y))));
            tmp_y += unit_tile_height;
        }
        tmp_x += unit_tile_width;
    }

    res
}

pub fn are_tile_and_unit_tile_compatible(tile: &Tile, unit_tile: &UnitTile) -> Option<(f64, f64)> {
    let tile_width = tile.width();

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
