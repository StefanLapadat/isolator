use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle};
use crate::triangulation::building_with_polygon_walls::BuildingWithPolygonWalls;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingWithTrianguizedWalls {
    walls: Vec<TriangulizedWall>,
    wireframe: Vec<Vec<Point>>
}

impl BuildingWithTrianguizedWalls {
    pub fn from_building(building: BuildingWithPolygonWalls) -> BuildingWithTrianguizedWalls {
        BuildingWithTrianguizedWalls {
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