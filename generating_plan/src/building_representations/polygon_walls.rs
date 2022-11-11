use crate::general_geometry::{Point, Polygon};
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
            walls: Polygon::merge_multiple_polygons(&walls)
            // walls
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
}
