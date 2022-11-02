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
    points: Vec<Point>,
    holes: Vec<Vec<Point>>
}

impl Polygon {
    pub fn new(points: Vec<Point>, holes: Vec<Vec<Point>>) -> Polygon {
        Polygon {
            points: points,
            holes: holes
        }
    }

    pub fn points<'a>(&'a self) -> & 'a Vec<Point> {
        &self.points
    }

    pub fn holes<'a>(&'a self) -> & 'a Vec<Vec<Point>> {
        &self.holes
    }
}
