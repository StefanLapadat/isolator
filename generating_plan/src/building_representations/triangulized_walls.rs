use serde::{Serialize, Deserialize};
use general_geometry::{Point, Triangle};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrianguizedWalls {
    walls: Vec<TriangulizedWall>,
    wireframe: Vec<Vec<Point>>
}

impl TrianguizedWalls {
    pub fn new(walls: Vec<TriangulizedWall>, wireframe: Vec<Vec<Point>>) -> TrianguizedWalls {
        TrianguizedWalls {
            walls,
            wireframe
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulizedWall {
    triangles: Vec<Triangle>
}

impl TriangulizedWall {
    pub fn new(triangles: Vec<Triangle>) -> TriangulizedWall {
        TriangulizedWall {
            triangles
        }
    }
}