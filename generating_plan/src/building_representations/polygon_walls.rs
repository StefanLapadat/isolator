use crate::general_geometry::{Point, Polygon, Triangle};
use crate::building_representations::triangulized_walls::{TriangulizedWall};
use crate::triangulation::PolygonForTriangulation;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonWalls {
    walls: Vec<Polygon>
}

impl PolygonWalls {

    pub fn new(walls: Vec<Polygon>) -> PolygonWalls {
        PolygonWalls {
            walls
        }
    }

    pub fn walls(&self) -> &Vec<Polygon> {
        &self.walls
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
            for point in wall.rim() {
                seq.push(point.clone());
            }
            if !wall.rim().is_empty() {
                seq.push(wall.rim()[0].clone());
            }
            res.push(seq);
            for hole in wall.holes() {
                let mut seq_hole:Vec<Point> = vec![];
                for point in hole {
                    seq_hole.push(point.clone());
                }
                if !hole.is_empty() {
                    seq_hole.push(hole[0].clone());
                }
                res.push(seq_hole);
            }
        }

        res
    }
}
