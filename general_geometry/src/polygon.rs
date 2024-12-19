use serde::{Serialize, Deserialize};
use crate::{Point, Plane, Simmilar, PolygonPointsOnSides, LineSegment, CoordinateSystem3D, Point2D};
use petgraph::graph::UnGraph;
use petgraph::algo;

use super::Polygon2D;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Polygon {
    rim: Vec<Point>,
    holes: Vec<Vec<Point>>
}

impl Polygon {
    pub fn new(rim: Vec<Point>, holes: Vec<Vec<Point>>) -> Polygon {
        Self::from_polygon_points_on_sides(PolygonPointsOnSides::new(rim, holes))
    }

    pub fn from_polygon_points_on_sides(poly: PolygonPointsOnSides) -> Polygon {
        let rim_and_holes = poly.to_polygon();

        Polygon { rim: rim_and_holes.0, holes: rim_and_holes.1 }
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

    pub fn in_xy_plane_no_holes_from_increments_points(start_rim: (f64, f64), increments_rim: Vec<Point>) -> Polygon {
        Self::in_xy_plane_no_holes_from_increments(start_rim, increments_rim.iter().map(|x| { (x.x, x.y) }).collect())
    }

    pub fn rim(&self) -> &Vec<Point> {
        &self.rim
    }

    pub fn holes(&self) -> &Vec<Vec<Point>> {
        &self.holes
    }

    pub fn translate(&self, inc: &Point) -> Polygon {
        let translate_vec_points = |x: &Vec<Point>| x.iter().map(|pt| pt.add(inc)).collect::<Vec<_>>();

        let rim = translate_vec_points(self.rim());
        let holes = self.holes().iter().map(|hole| translate_vec_points(hole)).collect::<Vec<_>>();

        Polygon::new(rim, holes)
    }

    pub fn wireframe(&self) -> Vec<Vec<Point>> {
        let mut res: Vec<Vec<Point>> = vec![];

        res.push(Self::rim_wireframe(self.rim()));

        for hole in self.holes() {
            res.push(Self::rim_wireframe(hole));
        }

        res
    }

    fn rim_wireframe(points: &Vec<Point>)-> Vec<Point> {
        let mut res = points.clone();
        res.push(points[0].clone());
        res
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

    pub fn flatten_points(points: &Vec<&Point>, system: &CoordinateSystem3D) -> Vec<f64> {
        Self::remove_constant_coordinate(&Self::flatten_points_no_removal_of_constant_coordinate(points, system))
    }

    pub fn destruct_to_components(self) -> (Vec<Point>, Vec<Vec<Point>>) {
        (self.rim, self.holes)
    }

    pub fn distance_from_origin(&self) -> Point {
        self.plane().distance_from_origin()
    }

    pub fn flatten_points_no_removal_of_constant_coordinate(points: &Vec<&Point>, system: &CoordinateSystem3D) -> Vec<Point> {
        points.iter().map(
            |p| p.coordinates_in_different_coordinate_system_original_base(system))
            .collect::<Vec<_>>()
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

        let res = if !normal1.are_vectors_colinear(&normal2) || !normal1.same_oktant(&normal2) {
            false 
        } else {
            Self::is_poly1_close_to_poly2(poly1, poly2) || Self::is_poly1_close_to_poly2(poly2, poly1)
        };

        res
    }

    fn is_poly1(poly: &Polygon) -> bool {
        if Point::are_points_simmilar(&poly.rim()[0], &Point::new(0.0, 25.0, 0.0)) &&
            Point::are_points_simmilar(&poly.rim()[1], &Point::new(0.0, -0.3, 0.0)) &&
            Point::are_points_simmilar(&poly.rim()[2], &Point::new(-0.3, -0.3, 0.0)) &&
            Point::are_points_simmilar(&poly.rim()[3], &Point::new(-0.3, 25.3, 0.0)) {
                return true;
        }

        false
    }

    fn is_poly2(poly: &Polygon) -> bool {
        if  Point::are_points_simmilar(&poly.rim()[0], &Point::new( 0.0, 25.3, 0.0 )) &&
            Point::are_points_simmilar(&poly.rim()[0], &Point::new( 0.0, 25.0, 0.0 )) &&
            Point::are_points_simmilar(&poly.rim()[0], &Point::new(-0.3, 25.3, 0.0))  {
                return true;
        }

        false
    }

    fn is_poly1_close_to_poly2(poly1: &Polygon, poly2: &Polygon) -> bool {
        let mut j = 0;
        while j < poly1.rim().len() {
            let point = &poly1.rim()[j];
            let rl1 = poly1.rim().len();
            let prev_poly1 = &poly1.rim()[(j + rl1 - 1)%rl1];
            let next_poly1 = &poly1.rim()[(j +  1)%rl1];
            
            let mut i = 0;
            let rl = poly2.rim().len();
            while i < rl {
                let tmp = &poly2.rim()[i];
                let prev = &poly2.rim()[(i + rl -1) % rl];
                let next = &poly2.rim()[(i+1)%rl];

                if Point::are_points_simmilar(point, tmp) && 
                    ((Point::are_points_simmilar(&next_poly1, prev) || 
                    LineSegment::new(prev.clone(), tmp.clone()).point_on_a_line_segment_excluding_end_points(&next_poly1)
                    ) || ((Point::are_points_simmilar(&prev_poly1, next) || 
                    LineSegment::new(next.clone(), tmp.clone()).point_on_a_line_segment_excluding_end_points(&prev_poly1)
                )))
                {
                    return true;
                }

                if LineSegment::new(prev.clone(), poly2.rim()[i].clone()).point_on_a_line_segment_excluding_end_points(point) {
                    return true;
                }

                i+=1;
            }

            j+=1;
        }   
        false
    }

    pub fn normal(&self) -> Point {
        self.plane().normal_vector()
    }

    pub fn plane(&self) -> Plane {
        Plane::from_points_vector(self.rim()).unwrap()
    }

    pub fn coordinate_system_xy_parallel_to_self(&self) -> CoordinateSystem3D {
        self.plane().coordinate_system_normal_to_plane_origin_at_base()
    }

    pub fn to_2d(&self, system: &CoordinateSystem3D) -> Polygon2D {
        let rim = Self::flatten_points_to_points_2d(&self.rim(), system);

        let holes = 
            self.holes().into_iter().map(|hole| Self::flatten_points_to_points_2d(hole, system)).collect::<Vec<_>>();

        Polygon2D::new(rim, holes)
    }

    fn flatten_points_to_points_2d(points: &Vec<Point>, system: &CoordinateSystem3D) -> Vec<Point2D> {
        Self::from_raw_vals_to_points(Self::flatten_points(&points.iter().collect::<Vec<_>>(), system))
    }


    fn from_raw_vals_to_points(vals: Vec<f64>) -> Vec<Point2D> {
        let mut i = 0;

        let mut res = vec![];

        while i < vals.len() {
            res.push(Point2D::new(vals[i], vals[i+1]));
            i+=2;
        }

        res
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

    fn merge_polygons(poly1: &Polygon, poly2: &Polygon) -> Polygon {
        let mut result_rim: Vec<Point> = vec![];
        let start = Self::pick_starting_point_for_merging(&poly1, &poly2);

        let mut tmp = start;
        let start_poly = if start.0 { poly1 } else { poly2 };
        let mut tmp_poly = start_poly;

        loop {
            result_rim.push(tmp_poly.rim()[tmp.1].clone());

            tmp = Self::pick_next_point_for_merging(tmp, poly1, poly2, &result_rim);
            tmp_poly = if tmp.0 { poly1 } else { poly2 };
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
            if Point::are_points_simmilar(point, tmp) || LineSegment::new(tmp.clone(), next.clone()).point_on_a_line_segment_excluding_end_points(&point) {
                return false;
            }
            i+=1;
        }

        true
    }

    fn pick_next_point_for_merging<'a>(current_tmp: (bool, usize), poly1: &'a Polygon, poly2: &'a Polygon, current_result_rim: &Vec<Point>) -> (bool, usize) {
        let (current_poly, not_current_poly) = if current_tmp.0 { (poly1, poly2) } else { (poly2, poly1)};
        let current_point = &current_poly.rim()[current_tmp.1];
        let next_point = &current_poly.rim()[(current_tmp.1 + 1) % current_poly.rim().len()];

        let segment_on_which_point_lies = Self::segment_on_which_point_lies(current_point, not_current_poly);
        match segment_on_which_point_lies {
            Some(segment) => {
                match current_result_rim.into_iter().find(|pt| Point::are_points_simmilar(&not_current_poly.rim()[segment.0], &pt)) {
                    Some(_) =>  (current_tmp.0, (current_tmp.1 + 1) % current_poly.rim().len()),
                    None => (!current_tmp.0, segment.1)
                }
            },
            None => {
                let point_from_not_current_poly_on_segment: Option<usize> = Self::point_from_poly_on_segment(current_point, next_point, not_current_poly);

                match point_from_not_current_poly_on_segment {
                    Some(next_point_ind) => { 
                        match current_result_rim.into_iter().find(|pt| Point::are_points_simmilar(&not_current_poly.rim()[next_point_ind], &pt)) {
                            Some(_) => (!current_tmp.0, (next_point_ind + 1) % not_current_poly.rim().len()),
                            None => (!current_tmp.0, next_point_ind)
                        }
                    },

                    None => {
                        (current_tmp.0, (current_tmp.1 + 1) % current_poly.rim().len())
                    } 
                }
            }
        }
    }

    fn point_from_poly_on_segment(seg0: &Point, seg1: &Point, poly: &Polygon) -> Option<usize> {
        let mut i = 0; 
        let mut points_on_segment: Vec<usize> = vec![];
        while i < poly.rim().len() {
            let tmp = &poly.rim()[i];
            if Point::are_points_simmilar(tmp, seg1) || 
            LineSegment::new(seg0.clone(), seg1.clone()).point_on_a_line_segment_excluding_end_points(tmp) {
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

    fn segment_on_which_point_lies(point: &Point, poly: &Polygon) -> Option<(usize, usize)> {
        let mut i = 0; 
        while i < poly.rim().len() {
            let tmp = &poly.rim()[i];
            let next = &poly.rim()[(i+1) % poly.rim().len()];

            if Point::are_points_simmilar(point, next) || LineSegment::new(tmp.clone(), next.clone()).point_on_a_line_segment_excluding_end_points(point) {
                return Option::Some((i, (i+1) % poly.rim().len()))
            }

            i+=1;
        }

        Option::None
    }

    pub fn to_polygon_points_on_sides(&self) -> PolygonPointsOnSides {
        PolygonPointsOnSides::new(self.rim().clone(), self.holes().clone())
    }

    pub fn get_all_corners_on_polygon(ind: usize, all_polys: &Vec<Polygon>) -> Vec<Corner> {
        let mut res = vec![];
        let mut i = 0;

        while i < all_polys.len() {
            if i == ind {
                i+=1;
                continue;
            }

            res.append(&mut Self::get_all_corners_for_polygons(&all_polys[ind], &all_polys[i], i));
            i+=1;
        }

        res
    }

    fn get_all_corners_for_polygons(poly_on_which_to_find_corners: &Polygon, poly2: &Polygon, ind_poly2: usize) -> Vec<Corner> {
        let mut res = vec![];
        let poly = poly_on_which_to_find_corners;

        let mut i = 0;
        let prl = poly.rim().len();
        let rl2 = poly2.rim().len();

        while i < prl {
            let mut j = 0;
            let tmp_p = &poly.rim()[i];
            let next_p = &poly.rim()[(i+1)%prl];

            while j < rl2 {
                let tmp_p2 = &poly2.rim()[j];
                let next_p2 = &poly2.rim()[(j+1)%rl2];

                let seg = LineSegment::shared_segment_no_len0(
                    &LineSegment::new(tmp_p.clone(), next_p.clone()), 
                    &LineSegment::new(tmp_p2.clone(), next_p2.clone())
                );
                
                match seg {
                    Some(seg) => {
                        if !next_p.subtract(tmp_p).same_oktant(&next_p2.subtract(tmp_p2)) {
                            res.push(Corner{pt: seg.p1().clone(), ind_of_bordering_polygon: ind_poly2, ind_of_side_in_this_polygon: i});
                            res.push(Corner{pt: seg.p2().clone(), ind_of_bordering_polygon: ind_poly2, ind_of_side_in_this_polygon: i});
                        }                        
                    }, 
                    None => {}
                }
                
                j+=1;
            }
            i+=1;
        }

        res
    }

    pub fn is_convex_polygon_with_no_holes(&self) -> bool {
        self.holes().is_empty() && self.is_convex() 
    }

    fn is_convex(&self) -> bool {
        let mut i = 0;
        let len = self.rim().len();
        let normal = &self.normal();

        while i < len {
            let t1 = &self.rim()[i];
            let t2 = &self.rim()[(i+1)%len];
            let t3 = &self.rim()[(i+2)%len];

            if !LineSegment::point_left_from_line_segment_or_colinear(t3, t1, t2, normal) {
                return false;
            }

            i+=1;
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct Corner {
    pub pt: Point,
    pub ind_of_bordering_polygon: usize,
    pub ind_of_side_in_this_polygon: usize
}

#[derive(Clone, Debug)]
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
