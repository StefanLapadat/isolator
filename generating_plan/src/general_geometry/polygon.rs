use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Plane, Simmilar};
use petgraph::graph::{UnGraph};
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
        let mut res = vec![];
        let groups: Vec<Vec<usize>> = Self::get_connected_groups(polygons);

        for group in &groups {
            res.push(Self::merge_group_of_neighbouring_polygons(group, polygons));
        }

        res
    }

    fn get_connected_groups(polygons: &Vec<Polygon>) -> Vec<Vec<usize>> {
        let connections = Self::get_connections(polygons);
        let g = UnGraph::<u32, ()>::from_edges(&connections);
        let connected_components = algo::kosaraju_scc(&g);
        let res = connected_components.into_iter().map(|x| x.into_iter().map(|y| y.index()).collect::<Vec<_>>()).collect::<Vec<_>>();
        res
    }

    fn get_connections(polygons: &Vec<Polygon>) -> Vec<(u32, u32)> {
        let mut res: Vec<(u32, u32)> = vec![((polygons.len() - 1) as u32, (polygons.len() - 1) as u32)];
        
        let mut i: u32 = 0;
        while i < polygons.len() as u32 - 1 {
            let mut j: u32 = i + 1;

            while j < polygons.len() as u32 {
                if Self::are_neighbours(&polygons[i as usize], &polygons[j as usize]) {
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
                if Point::are_points_simmilar(&point, &poly2.rim()[i]) {
                    return true;
                }

                let prev = &poly2.rim()[(i + rl -1) % rl];
                if Self::point_near_line_segment(point, prev, &poly2.rim()[i]) {
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
            println!("{:?} {:?}", group, res);
            i+=1;
        }

        res
    }

    fn merge_polygons(poly1: &Polygon, poly2: &Polygon) -> Polygon {
        let mut result_rim: Vec<Point> = vec![];
        let start = Self::pick_starting_point_for_merging(&poly1, &poly2);
        let mut tmp = start;
        let start_poly = if start.0 { poly1 } else { poly2 };
        let tmp_poly = start_poly;

        loop {
            result_rim.push(tmp_poly.rim()[tmp.1].clone());
            tmp = Self::pick_next_point_for_merging(tmp, poly1, poly2);
            
            if Point::are_points_simmilar(&tmp_poly.rim()[tmp.1], &start_poly.rim()[start.1]) {
                break;
            }
        }

        let mut result_holes: Vec<Vec<Point>> = vec![];

        result_holes.append(&mut poly1.holes().clone());
        result_holes.append(&mut poly2.holes().clone());
        
        Polygon::new(result_rim, result_holes)
    }

    fn pick_starting_point_for_merging<'a>(poly1: &'a Polygon, poly2: &'a Polygon) -> (bool, usize) {
        let mut ind: usize = 0;
        
        loop {
            if Self::point_not_near_poly_rim(&poly1.rim()[ind], poly2) {
                break;
            }

            ind+=1;
        }

        (true, ind)
    }

    fn point_not_near_poly_rim(point: &Point, poly: &Polygon) -> bool {
        let mut i = 0;

        while i < poly.rim().len() {
            let tmp = &poly.rim()[i];
            let next = &poly.rim()[(i + 1) % poly.rim().len()];
            if Point::are_points_simmilar(point, tmp) || Self::point_near_line_segment(point, tmp, next) {
                return false;
            }
            i+=1;
        }

        true
    }

    fn pick_next_point_for_merging<'a>(current_tmp: (bool, usize), poly1: &'a Polygon, poly2: &'a Polygon) -> (bool, usize) {
        let (current_poly, not_current_poly) = if current_tmp.0 { (poly1, poly2) } else { (poly2, poly1)};
        let current_point = &current_poly.rim()[current_tmp.1];
        let next_point = &current_poly.rim()[(current_tmp.1 + 1) % current_poly.rim().len()];

        let point_from_not_current_poly_on_segment: Option<(usize)> = Self::point_from_poly_on_segment(current_point, next_point, not_current_poly);
        match point_from_not_current_poly_on_segment {
            Some(next_point_ind) => (!current_tmp.0, next_point_ind),
            None => (current_tmp.0, current_tmp.1 + 1)
        }
    }

    fn point_from_poly_on_segment(seg0: &Point, seg1: &Point, poly: &Polygon) -> Option<usize> {
        let mut i = 0; 
        let mut points_on_segment: Vec<usize> = vec![];
        while i < poly.rim().len() {
            let tmp = &poly.rim()[i];
            if Point::are_points_simmilar(tmp, seg0) || Point::are_points_simmilar(tmp, seg1) || 
            Self::point_near_line_segment(tmp, seg0, seg1) {
                points_on_segment.push(i);
            }
            i+=1;
        }
        if points_on_segment.is_empty() {
            Option::None
        } else {
            let moduls: Vec<f64> = points_on_segment.clone().into_iter().map(|pt| poly.rim()[pt].subtract(seg0).modulo()).collect::<Vec<_>>();
            let mut min_modulo_ind: usize = 0;
            let mut j = 1;
            while j < moduls.len() {
                if moduls[j] < moduls[min_modulo_ind] {
                    min_modulo_ind = j;
                }
                j+=1;
            }

            Option::Some(points_on_segment[min_modulo_ind])
        }
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
