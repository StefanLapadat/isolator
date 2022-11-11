use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Plane, Simmilar};
use geo_booleanop::boolean::BooleanOp;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Polygon {
    rim: Vec<Point>,
    holes: Vec<Vec<Point>>
}

impl Polygon {
    pub fn new(rim: Vec<Point>, holes: Vec<Vec<Point>>) -> Polygon {
        let temp = Polygon {
            rim: rim,
            holes: holes
        };

        temp.remove_points_not_on_corners()
    }

    pub fn from_triplets(rim: Vec<(f64, f64, f64)>, holes: Vec<Vec<(f64, f64, f64)>>) -> Polygon {
        let mut points_vec: Vec<Point> = vec![];
        let mut holes_vec: Vec<Vec<Point>> = vec![];
        let mut i: usize = 0;
        while i < rim.len() {
            points_vec.push(Point::new(rim[i].0, rim[i].1, rim[i].2));
            i+=1;
        }
        i = 0;
        let mut j: usize = 0;
        while i < holes.len() {
            holes_vec.push(vec![]);
            while j < holes[i].len(){
                holes_vec[i].push(Point::new(holes[i][j].0,holes[i][j].1,holes[i][j].2));
                j+=1;
            }

            i+=1;
        }

        Polygon::new(points_vec, holes_vec)
    }

    pub fn in_xy_plane_no_holes_from_increments(start_rim: (f64, f64), increments_rim: Vec<(f64, f64)>) -> Polygon {
        let holes: Vec<Vec<(f64, f64, f64)>> = vec![];
        let mut rim: Vec<(f64, f64, f64)> = vec![];

        let mut temp: (f64, f64, f64) = (start_rim.0, start_rim.1, 0.);
        rim.push(temp);

        for inc in increments_rim {
            temp = (temp.0 + inc.0, temp.1 + inc.1, 0.);
            rim.push(temp);
        }

        Self::from_triplets(rim, holes)
    }

    pub fn rim<'a>(&'a self) -> & 'a Vec<Point> {
        &self.rim
    }

    pub fn holes<'a>(&'a self) -> & 'a Vec<Vec<Point>> {
        &self.holes
    }

    pub fn wireframe(&self) -> Vec<Vec<Point>> {
        let mut res: Vec<Vec<Point>> = vec![];

        let mut seq: Vec<Point> = vec![];
        for point in self.rim() {
            seq.push(point.clone());
        }
        if !self.rim().is_empty() {
            seq.push(self.rim()[0].clone());
        }
        res.push(seq);
        for hole in self.holes() {
            let mut seq_hole:Vec<Point> = vec![];
            for point in hole {
                seq_hole.push(point.clone());
            }
            if !hole.is_empty() {
                seq_hole.push(hole[0].clone());
            }
            res.push(seq_hole);
        }

        res
    }

    pub fn translate(&self, inc: &Point) -> Polygon {
        let mut rim: Vec<Point> = vec![];
        let mut holes: Vec<Vec<Point>> = vec![];

        for point in self.rim() {
            rim.push(point.add(&inc));
        }

        for hole in self.holes() {
            let mut new_hole: Vec<Point> = vec![];
            for point in hole {
                new_hole.push(point.add(&inc));
            }
            holes.push(new_hole);
        }

        Polygon {
            rim,
            holes
        }
    }

    pub fn rim_extrusion(&self, inc: &Point) -> Vec<Polygon> {
        let mut res: Vec<Polygon> = vec![];

        let mut i: usize = 0;
        let rl = self.rim().len();

        while i < rl {
            res.push(Polygon::from_triplets(vec![
                (self.rim()[i].x, self.rim()[i].y, self.rim()[i].z), 
                (self.rim()[(i+1) % rl].x, self.rim()[(i+1) % rl].y, self.rim()[(i+1) % rl].z), 
                (self.rim()[(i+1) % rl].x + inc.x, self.rim()[(i+1) % rl].y + inc.y, self.rim()[(i+1) % rl].z + inc.z), 
                (self.rim()[i].x + inc.x, self.rim()[i].y + inc.y, self.rim()[i].z + inc.z)] ,vec![]));
            i+=1;
        }

        res
    }

    pub fn holes_extrusion(&self, inc: &Point) -> Vec<Polygon> {
        let mut res: Vec<Polygon> = vec![];

        for hole in self.holes() {
            res.append(&mut Polygon::hole_extrusion(hole, inc));
        }

        res
    }

    pub fn hole_extrusion(hole: &Vec<Point>, inc: &Point) -> Vec<Polygon> {
        let mut res: Vec<Polygon> = vec![];

        let mut i: usize = 0;
        let rl = hole.len();

        while i < rl {
            res.push(Polygon::from_triplets(vec![
                (hole[i].x, hole[i].y, hole[i].z), 
                (hole[(i+1) % rl].x, hole[(i+1) % rl].y, hole[(i+1) % rl].z), 
                (hole[(i+1) % rl].x + inc.x, hole[(i+1) % rl].y + inc.y, hole[(i+1) % rl].z + inc.z), 
                (hole[i].x + inc.x, hole[i].y + inc.y, hole[i].z + inc.z)] ,vec![]));
            i+=1;
        }

        res
    }

    pub fn merge_geo_polygons(geo_poly1: &old_geo_types::Polygon<f64>, geo_poly2: &old_geo_types::Polygon<f64>) -> old_geo_types::Polygon<f64> {
        println!("Geopolygons: {:?} {:?}", geo_poly1, geo_poly2);

        let mut union = geo_poly1.union(geo_poly2).into_iter();
        let res = union.next().unwrap();
        res
    }

    pub fn merge_polygons(poly1: &Polygon, poly2: &Polygon) -> Polygon {
        let geo_poly1 = poly1.polygon_to_geo_polygon();
        let geo_poly2 = poly2.polygon_to_geo_polygon();

        let (geo_poly1, geo_poly2) = Self::prepare_geo_polygons_for_merging(&geo_poly1, &geo_poly2);
        
        let flat_points_first_stage = Self::flatten_points_no_removal(poly1.rim());
        let constant_coord = find_constant_coordinate(&flat_points_first_stage);

        let plane = Plane::from_points_vector(poly1.rim()).unwrap();
        let new_coordinate_system = plane.coordinate_system_normal_to_plane();

        let merged_geo_poly = Self::merge_geo_polygons(&geo_poly1, &geo_poly2);

        println!("Merged {:?} ", merged_geo_poly);


        let mut holes: Vec<Vec<Point>> = vec![];

        for hole in poly1.holes() {
            holes.push(hole.clone());
        }

        for hole in poly2.holes() {
            holes.push(hole.clone());
        }

        Polygon::new(Self::geo_polygon_to_polygon(&merged_geo_poly, new_coordinate_system, constant_coord.0, constant_coord.1), holes)
    }

    fn prepare_geo_polygons_for_merging(geo_poly1: &old_geo_types::Polygon<f64>, geo_poly2: &old_geo_types::Polygon<f64>) ->  (old_geo_types::Polygon<f64>, old_geo_types::Polygon<f64>){
        let mut rim_coordinates11: Vec<(f64, f64)> = vec![];
        let mut rim_coordinates21: Vec<(f64, f64)> = vec![];

        for point in &geo_poly1.exterior().0 {
            match Self::simmilar_point_in_geo_poly(point.x, point.y, geo_poly2) {
                Option::Some(pt) => {
                    rim_coordinates11.push((pt.0, pt.1));
                },
                Option::None => {
                    rim_coordinates11.push((point.x, point.y));
                }
            }
        }

        for point in &geo_poly2.exterior().0 {
            rim_coordinates21.push((point.x, point.y));
        }

        let mut rim_coordinates12: Vec<old_geo_types::Coordinate<f64>> = vec![];
        let mut rim_coordinates22: Vec<old_geo_types::Coordinate<f64>> = vec![];

        for rc in &rim_coordinates11 {
            rim_coordinates12.push(old_geo_types::Coordinate::from(Self::modify_coord_to_fall_into_another_poly(*rc, &rim_coordinates21)));
        }

        for rc in &rim_coordinates21 {
            rim_coordinates22.push(old_geo_types::Coordinate::from(Self::modify_coord_to_fall_into_another_poly(*rc, &rim_coordinates11)));
        }

        let geo_poly1_new = old_geo_types::Polygon::new(old_geo_types::LineString(rim_coordinates12), vec![]);
        let geo_poly2_new = old_geo_types::Polygon::new(old_geo_types::LineString(rim_coordinates22), vec![]);

        (geo_poly1_new, geo_poly2_new)
    }

    fn modify_coord_to_fall_into_another_poly(point: (f64, f64),  rim: &Vec<(f64, f64)>) -> (f64, f64) {
        let mut i = 0;
        while i < rim.len() {
            let pt: Point = Point::new(point.0, point.1, 0.);
            let seg0: Point = Point::new(rim[i].0, rim[i].1, 0.);
            let seg1: Point = Point::new(rim[(i+1)%rim.len()].0, rim[(i+1)%rim.len()].1, 0.);

            if Self::point_near_line_segment(&pt, &seg0, &seg1) {
                
                if Self::point_right_of_line(&pt, &seg0, &seg1) {
                    let sym_p = Self::point_symetric_to_line(&pt, &seg0, &seg1);
                    return (sym_p.x, sym_p.y);
                }
            }
            i+=1;
        }

        point
    }

    fn point_right_of_line(pt: &Point, seg0: &Point, seg1: &Point) -> bool {
        let v1 = seg1.subtract(&seg0);
        let v2 = pt.subtract(&seg0);

        v1.x * v2.y * v1.y.signum() * v2.y.signum() > v2.x * v1.y * v1.y.signum() * v2.y.signum() 
    }

    fn point_symetric_to_line(pt: &Point, seg0: &Point, seg1: &Point) -> Point {
        // not really symetric, just moved along the normal on the segment, towards the segment
        pt.add(&(Self::normal_to_2d_line(seg0, seg1).multiply(0.00001)))
    }

    fn normal_to_2d_line(seg0: &Point, seg1: &Point) -> Point {
        Point::new(seg0.y-seg1.y, seg1.x-seg0.x, 0.)
    }

    pub fn point_near_line_segment(pt: &Point, seg0: &Point, seg1: &Point) -> bool  {
        if (pt.x == seg0.x && pt.y == seg0.y && pt.z == seg0.z) || (pt.x == seg1.x && pt.y == seg1.y && pt.z == seg1.z) {
            false
        } else {
            let v1 = seg1.subtract(&seg0);
            let v2 = pt.subtract(&seg0);

            if v1.are_vectors_colinear(&v2) && v1.same_oktant(&v2) && v1.modulo() > v2.modulo() {
                return true;
            }

            false
        }
    }

    fn simmilar_point_in_geo_poly(x: f64, y: f64, geo_poly: &old_geo_types::Polygon<f64>) -> Option<(f64, f64)> {
        for point in &geo_poly.exterior().0 {
            if point.x.simmilar_to(x, 0.001) && point.y.simmilar_to(y, 0.001) {
                return Option::Some((point.x, point.y));
            }
        }

        Option::None
    }

    fn polygon_to_geo_polygon(&self) -> old_geo_types::Polygon<f64> {
        let mut rim_coordinates: Vec<old_geo_types::Coordinate<f64>> = vec![];

        let flat_points_first_stage = Self::flatten_points_no_removal(self.rim());
        let flat_points: Vec<f64> = Self::remove_constant_coordinate(&flat_points_first_stage);

        let mut i = 0;
        while i<flat_points.len() {
            rim_coordinates.push(old_geo_types::Coordinate::from((flat_points[i] as f64, flat_points[i+1] as f64)));
            i+=2;
        }
        
        old_geo_types::Polygon::new(old_geo_types::LineString(rim_coordinates), vec![])
    }

    fn geo_polygon_to_polygon(poly: &old_geo_types::Polygon<f64>, coordinate_system: Vec<Point>, coord_to_insert: Coordinate, value_to_insert: f64) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];
        let invers_coordinate_system = Point::inverse_mat(&coordinate_system);
        
        let mut i = 0;
        while i < poly.exterior().0.len() - 1 {
            let coord = poly.exterior().0[i];
            
            res.push(Self::widen_coordinate(coord.x as f64, coord.y as f64, coord_to_insert.clone(), value_to_insert).coordinates_in_different_coordinate_system(&invers_coordinate_system));
            i+=1;
        }

        // for coord in &poly.exterior().0 {
        //     res.push(Self::widen_coordinate(coord.x as f64, coord.y as f64, coord_to_insert.clone(), value_to_insert).coordinates_in_different_coordinate_system(&invers_coordinate_system));
        // }

        res
    }

    fn widen_coordinate(flat_x: f64, flat_y: f64, coord_to_insert: Coordinate, value_to_insert: f64) -> Point {
        match coord_to_insert {
            Coordinate::X => {
                Point::new(value_to_insert, flat_x, flat_y)
            },
            Coordinate::Y => {
                Point::new( flat_x, value_to_insert, flat_y)
            },
            Coordinate::Z => {
                Point::new(flat_x, flat_y, value_to_insert, )
            }
        }
    }

    pub fn flatten_points_no_removal(points: &Vec<Point>) -> Vec<Point> {
        if points.is_empty() {
            panic!("greska teska 1!");
        }
    
        let plane = Plane::from_points_vector(points);
    
        match plane {
            Option::None => panic!("greska teska 2 {:?}", points),
            Option::Some(plane) => {
                let new_coordinate_system = plane.coordinate_system_normal_to_plane();
                let mut new_coordinates: Vec<Point> = vec![];

                for p in points {
                    new_coordinates.push(p.coordinates_in_different_coordinate_system(&new_coordinate_system));
                }
    
                new_coordinates
            }
        }
    }

    pub fn flatten_points(points: &Vec<Point>) -> Vec<f64> {
        Self::remove_constant_coordinate(&Self::flatten_points_no_removal(points))
    }

    fn remove_constant_coordinate(points: &Vec<Point>) -> Vec<f64> {
        let mut res = vec![];
    
        let constant_coordinate = find_constant_coordinate(points);
        match constant_coordinate.0 {
            Coordinate::X => {
                for p in points {
                    res.push(p.y);
                    res.push(p.z);
                }
            },
            Coordinate::Y => {
                for p in points {
                    res.push(p.x);
                    res.push(p.z);
                }
            },
            Coordinate::Z => {
                for p in points {
                    res.push(p.x);
                    res.push(p.y);
                }
            }
        }
    
        res
    }

    fn remove_points_not_on_corners(&self) -> Polygon {
        let rim: Vec<Point> = Self::remove_points_not_on_corners_one_ring(self.rim());
        let holes: Vec<Vec<Point>> = self.holes().into_iter().map(|hole| Self::remove_points_not_on_corners_one_ring(hole)).collect::<Vec<_>>();

        Polygon{rim, holes}
    }

    fn remove_points_not_on_corners_one_ring(rim: &Vec<Point>) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];
        let mut i: usize = 0;
        let rl = rim.len();

        while i < rl {
            let next = &rim[(i+1)%rl];
            let prev = &rim[(i + rl -1)%rl];
            if !Point::are_points_colinear(prev, &rim[i], next) {
                res.push(rim[i].clone());
            }
            i+=1;
        }

        res        
    }

    pub fn merge_multiple_polygons(polygons: &Vec<Polygon>) -> Vec<Polygon> {
        println!("There is {} polygons", polygons.len());

        let mut res = vec![];
        let groups: Vec<Vec<usize>> = Self::get_connected_groups(polygons);
        for group in &groups {
            res.push(Self::merge_group_of_neighbouring_polygons(group, polygons));
        }

        println!("There is {} result polygons", res.len());

        res
    }

    fn get_connected_groups(polygons: &Vec<Polygon>) -> Vec<Vec<usize>> {
        let connections = Self::get_connections(polygons);
        let g = UnGraph::<u32, ()>::from_edges(&connections);
        let connected_components = algo::kosaraju_scc(&g);
        let res = connected_components.into_iter().map(|x| x.into_iter().map(|y| y.index()).collect::<Vec<_>>()).collect::<Vec<_>>();
        println!("{:?}", res);
        res
    }

    fn get_connections(polygons: &Vec<Polygon>) -> Vec<(u32, u32)> {
        let mut res: Vec<(u32, u32)> = vec![((polygons.len() - 1) as u32, (polygons.len() - 1) as u32)];
        
        let mut i: u32 = 0;
        while i < polygons.len() as u32 - 1 {
            let mut j: u32 = i + 1;

            while j < polygons.len() as u32 {
                if Self::are_neighbours(&polygons[i as usize], &polygons[j as usize]) {
                    println!("********* {:?} {:?}", polygons[i as usize], polygons[j as usize]);
                    res.push((i, j));
                }

                j+=1;
            }
            
            i+=1;
        }

        res
    }

    fn are_neighbours(poly1: &Polygon, poly2: &Polygon) -> bool {
        let normal1 = poly1.normal();
        let normal2 = poly2.normal();
        if !normal1.are_vectors_colinear(&normal2) || !normal1.same_oktant(&normal2) {
            false 
        } else {
            Self::is_poly1_close_to_poly2(poly1, poly2) || Self::is_poly1_close_to_poly2(poly2, poly1)
        }
    }

    fn is_poly1_close_to_poly2(poly1: &Polygon, poly2: &Polygon) -> bool {
        for point in poly1.rim() {
            let mut i = 0;
            let rl = poly2.rim().len();
            while i < rl {
                if point.x.simmilar_to(poly2.rim()[i].x, 0.0001) && point.y.simmilar_to(poly2.rim()[i].y, 0.0001) && 
                point.z.simmilar_to(poly2.rim()[i].z, 0.0001) {
                    println!("Simmilar corners {:?} {:?}", point, poly2.rim()[i]);
                    return true;
                }

                let prev = &poly2.rim()[(i + rl -1) % rl];
                if Self::point_near_line_segment(point, prev, &poly2.rim()[i]) {
                    println!("Near the line {:?} {:?} {:?}", point, poly2.rim()[i], prev);

                    return true;
                }

                i+=1;
            }
        }
        false
    }

    fn normal(&self) -> Point {
        Plane::from_points_vector(self.rim()).unwrap().normal_vector()
    }

    fn merge_group_of_neighbouring_polygons(group: &Vec<usize>, polygons: &Vec<Polygon>) -> Polygon {
        let mut res = polygons[group[0]].clone();
        let mut i = 1;

        while i < group.len() {
            res = Self::merge_polygons(&res, &polygons[group[i]]);
            i+=1;
        }

        res
    }
}

#[derive(Clone)]
enum Coordinate {
    X,
    Y,
    Z
}

fn find_constant_coordinate(points: &Vec<Point>) -> (Coordinate, f64) { //TODO: This is not right
    if points[0].x.simmilar_to(points[1].x, 0.01) && points[1].x.simmilar_to(points[2].x, 0.01) { 
        return (Coordinate::X, points[0].x)
    } else if points[0].y.simmilar_to(points[1].y, 0.01) && points[1].y.simmilar_to(points[2].y, 0.01) {
        return (Coordinate::Y, points[0].y)
    }

    (Coordinate::Z, points[0].z)
}
