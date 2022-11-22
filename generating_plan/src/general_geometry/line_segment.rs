use crate::general_geometry::{Point, Simmilar};

#[derive(Clone, Debug)]
pub struct LineSegment {
    p1: Point,
    p2: Point
}

impl LineSegment {
    pub fn new(p1: Point, p2: Point) -> LineSegment {
        LineSegment {p1, p2}
    }

    pub fn point_on_a_line_segment_excluding_end_points(&self, pt: &Point) -> bool {
        if Point::are_points_simmilar(pt, &self.p1) || Point::are_points_simmilar(pt, &self.p2) {
            false
        } else {
            self.point_on_a_line_segment(pt)
        }
    }

    pub fn point_on_a_line_segment(&self, pt: &Point) -> bool {
        let seg0 = &self.p1;
        let seg1 = &self.p2;

        let v1 = seg1.subtract(seg0);
        let v2 = pt.subtract(seg0);

        if v1.are_vectors_colinear(&v2) && v1.same_oktant(&v2) && (v1.modulo() > v2.modulo() || v1.modulo().simmilar_to(v2.modulo(), 0.0001)) {
            return true;
        }

        false   
    }

    pub fn shared_segment_no_len0(ls1: &LineSegment, ls2: &LineSegment) -> Option<LineSegment> {
        if ls2.point_on_a_line_segment(&ls1.p1) && ls2.point_on_a_line_segment(&ls1.p2) {
            return Some(ls1.clone());
        }

        if ls1.point_on_a_line_segment(&ls2.p1) && ls1.point_on_a_line_segment(&ls2.p2) {
            return Some(ls2.clone());
        }

        if ls2.point_on_a_line_segment(&ls1.p1) && ls1.point_on_a_line_segment(&ls2.p1) && !Point::are_points_simmilar(&ls1.p1, &ls2.p1){
            return Some(LineSegment::new(ls1.p1.clone(), ls2.p1.clone()));
        }

        if ls2.point_on_a_line_segment(&ls1.p1) && ls1.point_on_a_line_segment(&ls2.p2) && !Point::are_points_simmilar(&ls1.p1, &ls2.p2) {
            return Some(LineSegment::new(ls1.p1.clone(), ls2.p2.clone()));
        }

        if ls2.point_on_a_line_segment(&ls1.p2) && ls1.point_on_a_line_segment(&ls2.p1) && !Point::are_points_simmilar(&ls1.p2, &ls2.p1) {
            return Some(LineSegment::new(ls1.p2.clone(), ls2.p1.clone()));
        }

        if ls2.point_on_a_line_segment(&ls1.p2) && ls1.point_on_a_line_segment(&ls2.p2) && !Point::are_points_simmilar(&ls1.p2, &ls2.p2) {
            return Some(LineSegment::new(ls1.p2.clone(), ls2.p2.clone()));
        }

        None
    }

    pub fn to_point(&self) -> Point {
        self.p2.subtract(&self.p1)
    }

    pub fn p1(&self) -> &Point {
        &self.p1
    }

    pub fn p2(&self) -> &Point {
        &self.p2
    }

    pub fn len(&self) -> f64 {
        self.to_point().modulo()
    }
}