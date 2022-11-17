use crate::general_geometry::{Point2D, Polygon, Point};

use super::CoordinateSystem3D;

#[derive(Clone, Debug)]
pub struct Polygon2D {
    rim: Vec<Point2D>,
    holes: Vec<Vec<Point2D>>
}

impl Polygon2D {
    pub fn new(rim: Vec<Point2D>, holes: Vec<Vec<Point2D>>) -> Polygon2D {
        Polygon2D { rim, holes }
    }

    pub fn union_box(&self) -> Rectangle {
        let (min_x, max_x, min_y, max_y): (f64, f64, f64, f64);

        min_x = self.rim.iter().map(|r| r.x()).min_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        max_x = self.rim.iter().map(|r| r.x()).max_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        min_y = self.rim.iter().map(|r| r.y()).min_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        max_y = self.rim.iter().map(|r| r.y()).max_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        
        Rectangle::new(Point2D::new(min_x, min_y), Point2D::new(max_x, max_y))
    }

    pub fn union_box_many(polygons: Vec<Polygon2D>) -> Rectangle {
        Rectangle::union_box(polygons.iter().map(|p| p.union_box()).collect::<Vec<_>>())
    }

    pub fn rim(&self) -> &Vec<Point2D> {
        &self.rim
    }

    pub fn holes(&self) -> &Vec<Vec<Point2D>> {
        &self.holes
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    low_left: Point2D,
    up_right: Point2D
}

impl Rectangle {
    pub fn new(low_left: Point2D, up_right: Point2D) -> Rectangle {
        Rectangle { low_left, up_right }
    }

    pub fn union_box(rects: Vec<Rectangle>) -> Rectangle {
        let (min_x, max_x, min_y, max_y): (f64, f64, f64, f64);

        min_x = rects.iter().map(|r| r.low_left.x()).min_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        max_x = rects.iter().map(|r| r.up_right.x()).max_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        min_y = rects.iter().map(|r| r.low_left.y()).min_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        max_y = rects.iter().map(|r| r.up_right.y()).max_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap();
        
        Rectangle::new(Point2D::new(min_x, min_y), Point2D::new(max_x, max_y))
    }

    pub fn to_3d(&self, self_system: &CoordinateSystem3D) -> Polygon {
        let inv_system = self_system.inverse_system();
        let rim_pts = self.to_points_3d_in_self_plane().iter().map(|pt| pt.coordinates_in_different_coordinate_system_original_base(&inv_system)).collect::<Vec<_>>();
        Polygon::new(rim_pts, vec![])
    }

    fn to_points_3d_in_self_plane(&self) -> Vec<Point> {
        vec![
            self.low_left.clone(), 
            self.low_right(), 
            self.up_right.clone(), 
            self.up_left()].iter().map(|p| Self::point_2d_to_point_3d_in_self_plane(p)).collect::<Vec<_>>()
    }

    fn low_right(&self) -> Point2D {
        Point2D::new(self.up_right.x(), self.low_left.y())
    }

    fn up_left(&self) -> Point2D {
        Point2D::new(self.low_left.x(), self.up_right.y())
    }

    fn point_2d_to_point_3d_in_self_plane(point: &Point2D) -> Point {
        Point::new(point.x(), point.y(), 0.)
    }
}
