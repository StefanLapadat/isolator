use crate::general_geometry::{Point, Angle, PositiveF64};
use std::f64::consts::PI;

pub struct CornerHandlingResult {
    base_line_near_line: Point,
    base_line_away_from_line: Point,
    other_near_line: Point,
    other_away_from_line: Point
}

pub fn handle_corner(angle_from_base_line_to_other_in_positive_direction: Angle, width_base_line: PositiveF64, width_other: PositiveF64) -> CornerHandlingResult {
    let angle = angle_from_base_line_to_other_in_positive_direction;
    let angle_other = Angle::new(angle.val() - PI/2.0);

    CornerHandlingResult {
        base_line_near_line: Point::new(0., 0., 0.),
        other_near_line: Point::new(0., 0., 0.),
        base_line_away_from_line: Point::new(0., width_base_line.val(), 0.),
        other_away_from_line: Point::new(width_other.val() * angle_other.val().cos(), width_other.val() * angle_other.val().sin(), 0.)
    }
}
