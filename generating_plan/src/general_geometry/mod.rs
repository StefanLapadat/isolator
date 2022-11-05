use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn copy_new(point: &Point) -> Point {
        Point::new(point.x, point.y, point.z)
    }

    pub fn subtract(&self, point: &Point) -> Point {
        Point::new(self.x - point.x, self.y - point.y, self.z - point.z)
    }

    pub fn add(&self, point: &Point) -> Point {
        Point::new(self.x + point.x, self.y + point.y, self.z + point.z)
    }

    fn multiply(&self, scalar: f64) -> Point {
        Point::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    fn divide(&self, scalar: f64) -> Point {
        Point::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }

    pub fn vector_multiplication(a: &Point, b: &Point) -> Point {
        Point::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
    }

    pub fn coordinates_in_different_coordinate_system(&self, new_system: &Vec<Point>) -> Point {
        Point {
            x: new_system[0].dot_product(self),
            y: new_system[1].dot_product(self),
            z: new_system[2].dot_product(self),
        }
    }

    pub fn dot_product(&self, b: &Point) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn modulo(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Point {
        let modulo = self.modulo();

        Point {
            x: self.x / modulo,
            y: self.y / modulo,
            z: self.z / modulo
        }
    }
}

pub struct Polygon {
    rim: Vec<Point>,
    holes: Vec<Vec<Point>>
}

impl Polygon {
    pub fn new(rim: Vec<Point>, holes: Vec<Vec<Point>>) -> Polygon {
        Polygon {
            rim: rim,
            holes: holes
        }
    }

    pub fn from_triplets(points: Vec<(f64, f64, f64)>, holes: Vec<Vec<(f64, f64, f64)>>) -> Polygon {
        let mut points_vec: Vec<Point> = vec![];
        let mut holes_vec: Vec<Vec<Point>> = vec![];
        let mut i: usize = 0;
        while i < points.len() {
            points_vec.push(Point::new(points[i].0, points[i].1, points[i].2));
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

    pub fn rim<'a>(&'a self) -> & 'a Vec<Point> {
        &self.rim
    }

    pub fn holes<'a>(&'a self) -> & 'a Vec<Vec<Point>> {
        &self.holes
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Triangle {
    t1: Point,
    t2: Point,
    t3: Point
}

impl Triangle {
    pub fn new(t1: &Point, t2: &Point, t3: &Point) -> Triangle {
        Triangle {
            t1: Point::copy_new(t1),
            t2: Point::copy_new(t2),
            t3: Point::copy_new(t3)
        }
    }
}

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
