use crate::general_geometry::{Point, Polygon, Plane, Triangle};
use earcutr::earcut;

pub struct PolygonForTriangulation { 
    points: Vec<Point>,
    holes: Vec<usize>
}

impl PolygonForTriangulation {
    pub fn from_polygon(polygon: &Polygon) -> PolygonForTriangulation{
        PolygonForTriangulation {
            holes: PolygonForTriangulation::indices_of_holes_in_merged_points_and_holes(polygon),
            points: PolygonForTriangulation::merge_points_and_holes(polygon)
        }
    }

    pub fn points<'a>(&'a self) -> &'a Vec<Point> {
        &self.points
    }

    fn merge_points_and_holes(polygon: &Polygon) -> Vec<Point> {
        let mut res: Vec<Point> = vec![];
    
        for point in polygon.rim() {
            res.push(point.clone());
        }
    
        for hole in polygon.holes() {
            for hole_point in hole {
                res.push(hole_point.clone());
            }
        }
    
        res
    }

    fn indices_of_holes_in_merged_points_and_holes(polygon: &Polygon) -> Vec<usize> {
        let mut res = vec![];
    
        let mut acc: usize = 0;
    
        for hole in polygon.holes() {
            res.push(polygon.rim().len() + acc);
            acc+=hole.len();
        }
    
        res
    }

    fn flatten_points(points: &Vec<Point>) -> Vec<f64> {
        if points.is_empty() {
            panic!("greska teska 1!");
        }
    
        let plane = crate::triangulation::Plane::from_points_vector(points);
    
        match plane {
            Option::None => panic!("greska teska 2"),
            Option::Some(plane) => {
                let new_coordinate_system = plane.coordinate_system_normal_to_plane();
                let mut new_coordinates: Vec<Point> = vec![];

                for p in points {
                    new_coordinates.push(p.coordinates_in_different_coordinate_system(&new_coordinate_system));
                }
    
                let res = PolygonForTriangulation::remove_constant_coordinate(&new_coordinates);
            
                res
            }
        }
    }

    fn remove_constant_coordinate(points: &Vec<Point>) -> Vec<f64> {
        let mut res = vec![];
    
        let constant_coordinate = find_constant_coordinate(points);
        match constant_coordinate {
            Coordinate::X => {
                for p in points {
                    res.push(p.y);
                    res.push(p.z);
                }
            },
            Coordinate::Y => {
                for p in points {
                    res.push(p.x);
                    res.push(p.z);
                }
            },
            Coordinate::Z => {
                for p in points {
                    res.push(p.x);
                    res.push(p.y);
                }
            }
        }
    
        res
    }

    fn triangulate_3d_indices_result(&self) -> Vec<usize> {
        return earcut(&PolygonForTriangulation::flatten_points(&self.points), &self.holes, 2);
    }

    pub fn triangulate_3d(&self) -> Vec<Triangle>{
        let tri = self.triangulate_3d_indices_result();
            
        let mut triangles: Vec<Triangle> = vec![];
        
        let mut i = 0;
        while i<tri.len() {
            let pts = self.points();
            triangles.push(Triangle::new(&pts[tri[i]], &pts[tri[i+1]], &pts[tri[i+2]]));
            i += 3;
        }

        triangles
    }
}




pub enum Coordinate {
    X,
    Y,
    Z
}

fn find_constant_coordinate(points: &Vec<Point>) -> Coordinate {
    if points[0].x.simmilar_to(points[1].x, 0.01) && points[1].x.simmilar_to(points[2].x, 0.01) { 
        return Coordinate::X
    } else if points[0].y.simmilar_to(points[1].y, 0.01) && points[1].y.simmilar_to(points[2].y, 0.01) {
        return Coordinate::Y
    }

    Coordinate::Z
}

trait Simmilar {
    fn simmilar_to(&self, other: Self, epsilon: f64) -> bool;
}

impl Simmilar for f64 {
    fn simmilar_to(&self, other: f64, epsilon: f64) -> bool {
        (self - other).abs() < epsilon
    }
}


