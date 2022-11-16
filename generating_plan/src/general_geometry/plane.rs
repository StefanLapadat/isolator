use crate::general_geometry::Point;

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

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn c(&self) -> f64 {
        self.c
    }

    pub fn d(&self) -> f64 {
        self.d
    }

    pub fn from_points_vector_through_origin(points: &Vec<Point>) -> Option<Plane> {
        let noncolinear_points = Plane::get_three_noncolinear_points_from_vector_of_points(&points);
        match noncolinear_points {
            Option::Some(pts) => Option::Some(Plane::from_three_noncolinear_points_through_origin(pts.0, pts.1, pts.2)),
            Option::None => Option::None
        }
    }

    fn from_three_noncolinear_points_through_origin(t1: &Point, t2: &Point, t3: &Point) -> Plane {
        let normal = Point::vector_multiplication(&t2.subtract(t1), &t3.subtract(t2));
        Plane {
            a: normal.x,
            b: normal.y,
            c: normal.z,
            d: 0.
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
            d: -normal.dot_product(t1)
        }
    }

    fn get_three_noncolinear_points_from_vector_of_points<'a>(points: &'a Vec<Point>) -> Option<(&'a Point, &'a Point, &'a Point)> {
        if points.len() < 3 {
            Option::None
        } else {
            let mut i = 2;

            while i < points.len() {
                if !Point::are_points_colinear(&points[0], &points[1], &points[i]) {
                    return Option::Some((&points[0], &points[1], &points[i]));
                }

                i = i + 1;
            }
        
            Option::None
        }
    }

    pub fn normal_vector(&self) -> Point {
        Point::new(self.a, self.b, self.c)
    }

    pub fn coordinate_system_normal_to_plane(&self) -> Vec<Point> {

        let z = self.normal_vector().normalize();
        let x = Point::vector_multiplication(&z, &z.add(&Point::new(z.x + 10.56782, z.y + 20.345454, z.z + -30.4563))).normalize();
        let y = Point::vector_multiplication(&z, &x).normalize();

        vec![z, x, y]
    }
}