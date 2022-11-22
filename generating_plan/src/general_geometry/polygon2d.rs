use crate::general_geometry::{Point2D, Polygon, Point};
use old_geo_types::{Polygon as GeoPolygon, LineString};
use geo_booleanop::boolean::BooleanOp;

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

    pub fn intersection(&self, other: &Self) -> Vec<Self> {
        let gp1 = self.to_geo_polygon();
        let gp2 = other.to_geo_polygon();

        gp1.intersection(&gp2).into_iter().map(|gp| Self::geo_polygon_to_polygon(&gp)).collect::<Vec<_>>()
    }

    fn to_geo_polygon(&self) -> GeoPolygon<f64> {
        let rim: LineString<f64> = self.rim().iter().map(|pt| (pt.x(), pt.y())).collect();
        let mut holes: Vec<LineString<f64>> = vec![];
        for h in self.holes() {
            holes.push(h.iter().map(|pt| (pt.x(), pt.y())).collect());
        }

        GeoPolygon::new(rim, holes)
    }

    fn geo_polygon_to_polygon(gp: &GeoPolygon<f64>) -> Self {
        let rim: Vec<Point2D> = gp.exterior().points_iter().map(|pt| Point2D::new(pt.x(), pt.y())).collect::<Vec<_>>();

        let mut holes: Vec<Vec<Point2D>> = vec![];
        for line in gp.interiors() {
            holes.push(line.points_iter().map(|pt| Point2D::new(pt.x(), pt.y())).collect::<Vec<_>>());
        }

        Self::new(rim, holes)
    }

    pub fn to_3d(&self, self_system: &CoordinateSystem3D, original_distance_from_origin: &Point) -> Polygon {
        let inv_system = self_system.inverse_system();
        let rim_pts = self.to_points_3d_in_self_plane().iter().map(|pt| pt.coordinates_in_different_coordinate_system_original_base(&inv_system).add(original_distance_from_origin)).collect::<Vec<_>>();
        
        Polygon::new(rim_pts, vec![])
    }

    fn to_points_3d_in_self_plane(&self) -> Vec<Point> {
        self.rim().iter().map(|p| Self::point_2d_to_point_3d_in_self_plane(p)).collect::<Vec<_>>()
    }

    fn point_2d_to_point_3d_in_self_plane(point: &Point2D) -> Point {
        Point::new(point.x(), point.y(), 0.)
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

    pub fn to_polygon_2d(&self) -> Polygon2D {
        Polygon2D::new(vec![self.low_left.clone(), self.low_right(), self.up_right.clone(), self.up_left()], vec![])
    }

    pub fn low_right(&self) -> Point2D {
        Point2D::new(self.up_right.x(), self.low_left.y())
    }

    pub fn up_left(&self) -> Point2D {
        Point2D::new(self.low_left.x(), self.up_right.y())
    }

    pub fn low_left(&self) -> Point2D {
        self.low_left.to_owned()
    }

    pub fn up_right(&self) -> Point2D {
        self.up_right.to_owned()
    }

    pub fn width(&self) -> f64 {
        (self.low_left.x() - self.low_right().x()).abs()
    }

    pub fn height(&self) -> f64 {
        (self.low_left.y() - self.up_right().y()).abs()
    }
}
