use crate::{Point, Simmilar, Angle};

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

    pub fn distance_from_point(&self, pt: &Point) -> f64 {
        if self.point_on_a_line_segment(pt) {
            return 0.;
        }

        if self.point_in_segment_cylingder(pt) {
            let a = self.len();
            let b_vec = pt.subtract(&self.p1);
            let c_vec = pt.subtract(&self.p2);

            return Self::triangle_height_on_first_side(a, b_vec.modulo(), c_vec.modulo(), &b_vec.angle_to(&c_vec));
        } else {
            return pt.subtract(&self.p1).modulo().min(pt.subtract(&self.p2).modulo());
        }
    }

    fn triangle_height_on_first_side(a: f64, b: f64, c: f64, bc_angle: &Angle) -> f64 {
        let twice_area = b * c * bc_angle.val().sin();
        twice_area / a
    }

    pub fn point_in_segment_cylingder(&self, pt: &Point) -> bool {
        let ang1 = self.to_point().angle_to(&pt.subtract(&self.p1)).val();
        let ang2 = self.invert().to_point().angle_to(&pt.subtract(&self.p1)).val();

        let pi_half = std::f64::consts::PI/2.;
        let eps = 0.0001;

        (ang1.simmilar_to(pi_half, eps) || ang1 < pi_half) && 
        (ang1.simmilar_to(0., eps) || ang1 > 0.) && 
        (ang2.simmilar_to(pi_half, eps) || ang2 < pi_half) && 
        (ang2.simmilar_to(0., eps) || ang2 > 0.)
    }

    pub fn invert(&self) -> Self {
        Self::new(self.p2.to_owned(), self.p1.to_owned())
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