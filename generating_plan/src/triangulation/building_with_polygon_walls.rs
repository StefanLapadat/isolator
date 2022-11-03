
use crate::general_geometry::{Point, Polygon, Triangle};
use crate::triangulation::building_with_triangulized_walls::{TriangulizedWall, BuildingWithTrianguizedWalls};
use crate::triangulation::PolygonForTriangulation;

pub struct BuildingWithPolygonWalls {
    walls: Vec<Polygon>
}

impl BuildingWithPolygonWalls {

    pub fn new(walls: Vec<Polygon>) -> BuildingWithPolygonWalls {
        BuildingWithPolygonWalls {
            walls
        }
    }

    pub fn triangulation(&self) -> Vec<TriangulizedWall> {
        let mut res: Vec<TriangulizedWall> = vec![];

        for wall in &self.walls {
            let poly_tri = PolygonForTriangulation::from_polygon(wall);

            let tri = crate::triangulation::triangulate_3d(&poly_tri);
            
            let mut triangles: Vec<Triangle> = vec![];
            
            let mut i = 0;
            while i<tri.len() {
                let pts = poly_tri.points();
                triangles.push(Triangle::new(&pts[tri[i]], &pts[tri[i+1]], &pts[tri[i+2]]));
                i += 3;
            }

            res.push(TriangulizedWall::new(triangles));
        }
    
        res
    }

    pub fn wireframe(&self) -> Vec<Vec<Point>> {
        let mut res = vec![];

        for wall in &self.walls {
            let mut seq: Vec<Point> = vec![];
            for point in wall.points() {
                seq.push(Point::copy_new(point));
            }
            if !wall.points().is_empty() {
                seq.push(Point::copy_new(&wall.points()[0]));
            }
            res.push(seq);
            for hole in wall.holes() {
                let mut seq_hole:Vec<Point> = vec![];
                for point in hole {
                    seq_hole.push(Point::copy_new(point));
                }
                if !hole.is_empty() {
                    seq_hole.push(Point::copy_new(&hole[0]));
                }
                res.push(seq_hole);
            }
        }

        res
    }
}
