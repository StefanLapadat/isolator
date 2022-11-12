use crate::general_geometry::Point;
use std::f64::consts::PI;

pub struct CornerHandlingResult {
    base_line_near_line: Point,
    base_line_away_from_line: Point,
    other_near_line: Point,
    other_away_from_line: Point
}

pub struct Angle {
    val: f64
}

impl Angle {
    fn new(val: f64) -> Angle {
        Angle {
            val: val % (2.0 * PI)
        }
    }

    fn val(&self) -> f64 {
        self.val
    } 
}

pub struct PositiveF64 {
    val: f64
}

impl PositiveF64 {
    fn new(val: f64) -> Option<PositiveF64> {
        if val >= 0. { Some(PositiveF64 {val}) } else {None}
    }

    fn val(&self) -> f64 {
        self.val
    }
}

fn handle_corner(angle_from_base_line_to_other_in_positive_direction: Angle, width_base_line: PositiveF64, width_other: PositiveF64) -> CornerHandlingResult {
    let angle = angle_from_base_line_to_other_in_positive_direction;
    let angle_other = Angle::new(angle.val() - PI/2.0);

    CornerHandlingResult {
        base_line_near_line: Point::new(0., 0., 0.),
        other_near_line: Point::new(0., 0., 0.),
        base_line_away_from_line: Point::new(0., width_base_line.val(), 0.),
        other_away_from_line: Point::new(width_other.val() * angle_other.val().cos(), width_other.val() * angle_other.val().sin(), 0.)
    }
}