use crate::{Point, Simmilar};

#[derive(Clone, Debug)]
pub struct Line3D {
    direction: Point,
    n: Point
}

impl Line3D {
    pub fn new(direction: Point, n: Point) -> Option<Line3D> {
        if direction.modulo().simmilar_to(0., 0.0001) { None } else {
            Some(Line3D {
                direction, n
            })
        }
    }

    pub fn direction(&self) -> &Point {
        &self.direction
    }

    pub fn from_2_points(p1: &Point, p2: &Point) -> Option<Line3D> {
        Self::new(p1.subtract(&p2), p1.clone())
    }
}

fn are_parallel(l1: &Line3D, l2: &Line3D) -> bool {
    Point::are_points_simmilar(&l1.direction, &l2.direction)
}

fn are_same(l1: &Line3D, l2: &Line3D) -> bool {
    Point::are_points_simmilar(&l1.direction, &l2.direction) && Point::are_points_simmilar(&l1.n, &l2.n)
}

// Thank this guy some day :) https://math.stackexchange.com/questions/2213165/find-shortest-distance-between-lines-in-3d
fn distance(l1: &Line3D, l2: &Line3D) -> f64 {
    let r1 = &l1.n;
    let r2 = &l2.n;
    let e1 = &l1.direction;
    let e2 = &l2.direction;

    let n = Point::cross_prod(e1, e2);

    let distance = n.dot_product(&r1.subtract(r2)).abs() / n.modulo();

    distance
}

pub fn intersection(l1: &Line3D, l2: &Line3D) -> Intersection {
    if are_same(l1, l2) {
        Intersection::Line(l1.clone())
    } else {
        if are_parallel(l1, l2) {
            Intersection::None
        } else {
            let distance = distance(l1, l2);
            if !distance.simmilar_to(0., 0.0001) {
                Intersection::None
            } else {
                intersection_of_two_lines_with_exactly_one_common_point(l1, l2)
            }
        }
    }
}

fn intersection_of_two_lines_with_exactly_one_common_point(l1: &Line3D, l2: &Line3D) -> Intersection {
    let r1 = &l1.n;
    let r2 = &l2.n;
    let e1 = &l1.direction;
    let e2 = &l2.direction;

    let n = Point::cross_prod(e1, e2);

    Intersection::Point(l1.direction.multiply(Point::cross_prod(e2, &n).dot_product(&r2.subtract(r1))/n.dot_product(&n)).add(&l1.n))
}

#[derive(Debug)]
pub enum Intersection {
    None,
    Point(crate::Point),
    Line(crate::Line3D)
}
