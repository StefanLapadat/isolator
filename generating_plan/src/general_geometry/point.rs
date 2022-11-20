use serde::{Serialize, Deserialize};
use nalgebra::Matrix3;
use crate::general_geometry::{Angle, Simmilar, CoordinateSystem3D};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub const ZERO: Point = Point::new(0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64) -> Point {
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

    pub fn cross_prod(a: &Point, b: &Point) -> Point {
        Point::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
    }

    pub fn coordinates_in_different_coordinate_system_original_base(&self, new_system: &CoordinateSystem3D) -> Point {
        if !Point::are_points_simmilar(new_system.o(), &Point::ZERO) {
            panic!("New system has to have origin at (0, 0, 0). Provided was {:?}", new_system.o());
        }

        Point {
            x: new_system.x().dot_product(self),
            y: new_system.y().dot_product(self),
            z: new_system.z().dot_product(self),
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
        self.divide(self.modulo())
    }

    pub fn are_points_colinear(t1: &Point, t2: &Point, t3: &Point) -> bool {
        t2.subtract(t1).are_vectors_colinear(&t3.subtract(t1))
    }

    pub fn divide_by_parallel_vec(&self, v2: &Point) -> f64 {
        
        if Self::are_points_simmilar(self, &Point { x: -43.39411254969542, y: -43.39411254969542, z: -1.05387500062791e-15 }) &&
        Self::are_points_simmilar(v2, &Point { x: -43.39411254969542, y: -43.39411254969542, z: -2.4088571442923657e-15 }){
            println!("ker");
        }

        if !self.are_vectors_colinear(v2) {
            panic!("This function can only be called for parallel vectors. Actual were: {:?} {:?}", self, v2);
        }
        
        let ratio;
        if v2.x != 0. {
            ratio = self.x / v2.x;
        } else {
            if self.y != 0. {
                ratio = self.y / v2.y;
            } else {
                ratio = self.z / v2.z;
            }
        }

        ratio
    }

    pub fn are_points_simmilar(t1: &Point, t2: &Point) -> bool {
        let epsilon = 0.0001;
        t1.x.simmilar_to(t2.x, epsilon) && t1.y.simmilar_to(t2.y, epsilon) && t1.z.simmilar_to(t2.z, epsilon) 
    }

    pub fn same_oktant(&self, t2: &Point) -> bool {
        let sx = Self::to_zero_if_minus_zero(self.x);
        let sy = Self::to_zero_if_minus_zero(self.y);
        let sz = Self::to_zero_if_minus_zero(self.z);

        let tx = Self::to_zero_if_minus_zero(t2.x);
        let ty = Self::to_zero_if_minus_zero(t2.y);
        let tz = Self::to_zero_if_minus_zero(t2.z);
        
        if self.modulo().simmilar_to(0., 0.0001) || t2.modulo().simmilar_to(0., 0.0001) {
            true
        } else {
            sx.signum() == tx.signum() && sy.signum() == ty.signum() && sz.signum() == tz.signum()
        }
    }

    fn to_zero_if_minus_zero(num: f64) -> f64 {
        if num == 0. {
            0.
        } else {
            num
        }
    }

    pub fn are_vectors_colinear(&self, t2: &Point) -> bool {
        if self.modulo().simmilar_to(0.0, 0.0001) || t2.modulo().simmilar_to(0.0, 0.0001) {
             return true;
        }

        let ratio;
        if !self.x.simmilar_to(0., 0.0001) {
            ratio = t2.x / self.x;
        } else {
            if !self.y.simmilar_to(0., 0.0001) {
                ratio = t2.y / self.y;
            } else {
                ratio = t2.z / self.z;
            }
        }

        let epsilon = 0.0000001;

        if(self.x.simmilar_to(0., 0.0001) && !t2.x.simmilar_to(0., 0.0001)) || 
        (self.y.simmilar_to(0., 0.0001) && !t2.y.simmilar_to(0., 0.0001)) || 
        (self.z.simmilar_to(0., 0.0001) && !t2.z.simmilar_to(0., 0.0001)) {
            return false;
        }

        (self.x.simmilar_to(0.,epsilon) | (t2.x / self.x).simmilar_to(ratio, epsilon)) &&
        (self.y.simmilar_to(0., epsilon) || (t2.y / self.y).simmilar_to(ratio, epsilon)) &&
        (self.z.simmilar_to(0., epsilon) || (t2.z / self.z).simmilar_to(ratio, epsilon))
    }

    pub fn angle_to(&self, other: &Point) -> Angle {
        Angle::new((self.dot_product(other) / (self.modulo() * other.modulo())).acos())
    }

    pub fn exactly_same(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    pub fn close_to_zero(&self) -> bool {
        return Point::are_points_simmilar(&self, &Self::ZERO);
    }
}
