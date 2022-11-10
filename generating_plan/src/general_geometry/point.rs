use serde::{Serialize, Deserialize};
use nalgebra::Matrix3;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn subtract(&self, point: &Point) -> Point {
        Point::new(self.x - point.x, self.y - point.y, self.z - point.z)
    }

    pub fn add(&self, point: &Point) -> Point {
        Point::new(self.x + point.x, self.y + point.y, self.z + point.z)
    }

    pub fn multiply(&self, scalar: f64) -> Point {
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

    pub fn inverse_mat(mat3x3: &Vec<Point>) -> Vec<Point> {
        let m = mat3x3;
        let m = Matrix3::new(m[0].x, m[0].y, m[0].z, m[1].x, m[1].y, m[1].z, m[2].x, m[2].y, m[2].z);
        
        let s = m.try_inverse().unwrap();

        vec![Point::new(s[0], s[3], s[6]), Point::new(s[1], s[4], s[7]), Point::new(s[2], s[5], s[8])]
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
