use general_geometry::{Point, Polygon, Plane};
use crate::building_representations::triangulized_walls::{TriangulizedWall};
use crate::triangulation::PolygonForTriangulation;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonWalls {
    pub walls: Vec<Polygon>
}

impl PolygonWalls {

    pub fn new(walls: Vec<Polygon>) -> PolygonWalls {
        let walls_merged = Polygon::merge_multiple_polygons(&walls);

        PolygonWalls {
            walls: walls_merged
        }
    }

    pub fn walls(&self) -> &Vec<Polygon> {
        &self.walls
    }

    pub fn triangulation(&self) -> Vec<TriangulizedWall> {
        let mut res: Vec<TriangulizedWall> = vec![];

        for wall in &self.walls {
            res.push(TriangulizedWall::new(PolygonForTriangulation::from_polygon(wall).triangulate_3d()));
        }
    
        res
    }

    pub fn wireframe(&self) -> Vec<Vec<Point>> {
        let mut res: Vec<Vec<Point>>  = vec![];

        for wall in &self.walls {
            let mut wall_wireframe = wall.wireframe();
            res.append(&mut wall_wireframe);
        }

        res
    }

    pub fn horizontal_walls(&self) -> Vec<Polygon> {
        self.walls.iter().filter(|wall| wall.plane().parallel_to(&Plane::XY)).map(|e| e.clone()).collect::<Vec<_>>()
    }
}
