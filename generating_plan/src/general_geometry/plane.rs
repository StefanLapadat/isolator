use crate::general_geometry::Point;
use crate::general_geometry::Simmilar;

#[derive(Debug)]
pub struct Plane {
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

    pub fn normal_vector(&self) -> Point {
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