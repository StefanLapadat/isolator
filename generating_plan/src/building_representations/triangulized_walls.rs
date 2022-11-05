use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle};
use crate::building_representations::polygon_walls::PolygonWalls;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrianguizedWalls {
    walls: Vec<TriangulizedWall>,
    wireframe: Vec<Vec<Point>>
}

impl TrianguizedWalls {
    pub fn from_building(building: PolygonWalls) -> TrianguizedWalls {
        TrianguizedWalls {
             walls: building.triangulation(),
             wireframe: building.wireframe()
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