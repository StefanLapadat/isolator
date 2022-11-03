use crate::general_geometry::{Point, Polygon};
use earcutr::earcut;

pub mod building_with_polygon_walls;
pub mod building_with_triangulized_walls;


#[derive(Debug)]
pub struct Plane { //ax + by + cz + d = 0
    a: f64,
    b: f64,
    c: f64,
    d: f64
}

impl Plane {
    fn new(a: f64, b: f64, c: f64, d: f64) -> Plane {
        Plane {
            a, b, c, d
        }
    }

    pub fn from_points_vector(points: &Vec<Point>) -> Option<Plane> {
        let noncolinear_points = Plane::get_three_noncolinear_points_from_vector_of_points(&points);
        match noncolinear_points {
            Option::Some(pts) => Option::Some(Plane::from_three_noncolinear_points(pts.0, pts.1, pts.2)),
            Option::None => Option::None
        }
    }

    fn from_three_noncolinear_points(t1: &Point, t2: &Point, t3: &Point) -> Plane {
        let normal = Point::vector_multiplication(&t2.subtract(t1), &t3.subtract(t2));
        Plane {
            a: normal.x,
            b: normal.y,
            c: normal.z,
            d: 0.
        }
    }

    fn get_three_noncolinear_points_from_vector_of_points<'a>(points: &'a Vec<Point>) -> Option<(&'a Point, &'a Point, &'a Point)> {
        if points.len() < 3 {
            return Option::None;
        } else {
            let res = Option::Some((&points[0], &points[1], &points[2]));
            let mut i = 2;

            while i < points.len() {
                if !Plane::are_points_colinear(&points[0], &points[1], &points[i]) {
                    return Option::Some((&points[0], &points[1], &points[i]));
                }

                i = i + 1;
            }
        
            return Option::None;
        }
    }

    fn are_points_colinear(t1: &Point, t2: &Point, t3: &Point) -> bool {
        Plane::are_vectors_colinear(&t2.subtract(t1), &t3.subtract(t1))
    }

    fn are_vectors_colinear(t1: &Point, t2: &Point) -> bool {
        let ratio;
        if t1.x != 0. {
            ratio = t2.x / t1.x;
        } else {
            if t1.y != 0. {
                ratio = t2.y / t1.y;
            } else {
                ratio = t2.z / t1.z;
            }
        }

        let epsilon = 0.01;

        (t2.x / t1.x).simmilar_to(ratio, epsilon) && (t2.y / t1.y).simmilar_to(ratio, epsilon) && (t2.z / t1.z).simmilar_to(ratio, epsilon)
    }

    fn normal_vector(&self) -> Point {
        Point {
            x: self.a, 
            y: self.b,
            z: self.c
        }
    }

    pub fn coordinate_system_normal_to_plane(&self) -> Vec<Point> {

        let z = self.normal_vector().normalize();
        let x = Point::vector_multiplication(&z, &z.add(&Point::new(z.x + 10.56782, z.y + 20.345454, z.z + -30.4563))).normalize();
        let y = Point::vector_multiplication(&z, &x).normalize();

        vec![z, x, y]
    }
}

trait Simmilar {
    fn simmilar_to(&self, other: Self, epsilon: f64) -> bool;
}

impl Simmilar for f64 {
    fn simmilar_to(&self, other: f64, epsilon: f64) -> bool {
        (self - other).abs() < epsilon
    }
}

pub fn triangulate_3d(polygon: &PolygonForTriangulation) -> Vec<usize> {
    return earcut(&PolygonForTriangulation::flatten_points(&polygon.points), &polygon.holes, 2);
}


pub enum Coordinate {
    X,
    Y,
    Z
}

fn find_constant_coordinate(points: &Vec<Point>) -> Coordinate {
    if points[0].x.simmilar_to(points[1].x, 0.01) && points[1].x.simmilar_to(points[2].x, 0.01) { 
        return Coordinate::X
    } else if points[0].y.simmilar_to(points[1].y, 0.01) && points[1].y.simmilar_to(points[2].y, 0.01) {
        return Coordinate::Y
    }

    Coordinate::Z
}


pub struct PolygonForTriangulation { 
    points: Vec<Point>,
    holes: Vec<usize>
}

impl PolygonForTriangulation {
    pub fn from_polygon(polygon: &Polygon) -> PolygonForTriangulation{
        PolygonForTriangulation {
            holes: PolygonForTriangulation::indices_of_holes_in_merged_points_and_holes(polygon),
            points: PolygonForTriangulation::merge_points_and_holes(polygon)
        }
    }

    pub fn points<'a>(&'a self) -> &'a Vec<Point> {
        &self.points
    }

    fn merge_points_and_holes(polygon: &Polygon) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];
    
        for point in polygon.points() {
            res.push(Point::copy_new(point));
        }
    
        for hole in polygon.holes() {
            for hole_point in hole {
                res.push(Point::copy_new(hole_point));
            }
        }
    
        res
    }

    fn indices_of_holes_in_merged_points_and_holes(polygon: &Polygon) -> Vec<usize> {
        let mut res = vec![];
    
        let mut acc: usize = 0;
    
        for hole in polygon.holes() {
            res.push(polygon.points().len() + acc);
            acc+=hole.len();
        }
    
        res
    }

    fn flatten_points(points: &Vec<Point>) -> Vec<f64> {
        if points.is_empty() {
            panic!("greska teska 1!");
        }
    
        let plane = crate::triangulation::Plane::from_points_vector(points);
    
        match plane {
            Option::None => panic!("greska teska 2"),
            Option::Some(plane) => {
                let new_coordinate_system = plane.coordinate_system_normal_to_plane();
                let mut new_coordinates: Vec<Point> = vec![];

                for p in points {
                    new_coordinates.push(p.coordinates_in_different_coordinate_system(&new_coordinate_system));
                }
    
                let res = PolygonForTriangulation::remove_constant_coordinate(&new_coordinates);
            
                res
            }
        }
    }

    fn remove_constant_coordinate(points: &Vec<Point>) -> Vec<f64> {
        let mut res = vec![];
    
        let constant_coordinate = find_constant_coordinate(points);
        match constant_coordinate {
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
}