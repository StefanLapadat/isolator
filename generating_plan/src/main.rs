use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use crate::general_geometry::{Point, Polygon};
use crate::triangulation::{PolygonForTriangulation};

mod triangulation;
mod general_geometry;

fn main() -> std::io::Result<()> {
    let plan: Plan = create_plan();

    let mut file = File::create("/home/stefan/Documents/cia/projects/isolator/drawing/public/abc.json")?;

    let serialized_points = serde_json::to_string(&plan).unwrap();

    file.write(serialized_points.as_bytes())?;

    Ok(())
}

fn create_plan() -> Plan {
    let building = create_building();

    Plan {
        building: BuildingForSerialization::from_building(building)
    }
}

fn create_building() -> Building {
    let house_whl = 25.0;

    let walls: Vec<Polygon> = vec![
        Polygon::new(vec![Point::new(0.,0.,0.), Point::new(10.,0.,0.), Point::new(10.,0.,10.), Point::new(15.,0.,10.), 
            Point::new(15.,0.,0.), Point::new(house_whl,0.,0.), Point::new(house_whl,0.,house_whl), Point::new(0.,0.,house_whl)], 
            vec![vec![Point::new(5.,0.,15.), Point::new(10.,0.,15.), Point::new(10.,0.,19.), Point::new(5.,0.,19.)]]),
        Polygon::new(vec![Point::new(house_whl,0.,0.), Point::new(house_whl,0.,house_whl), Point::new(house_whl,house_whl,house_whl), Point::new(house_whl,house_whl,0.)], 
            vec![]), 
        Polygon::new(vec![Point::new(0.,0.,0.), Point::new(0.,0.,25.), Point::new(0.,25.,25.), Point::new(0.,25.,0.)], 
            vec![]),
        Polygon::new(vec![Point::new(0.,house_whl,0.), Point::new(house_whl,house_whl,0.), Point::new(house_whl,house_whl,house_whl), Point::new(0.,house_whl,house_whl)], 
            vec![]),
        Polygon::new(vec![Point::new(0.,0.,0.), Point::new(0.,house_whl,0.), Point::new(house_whl,house_whl,0.), Point::new(house_whl,0.,0.)], 
            vec![]),
        Polygon::new(vec![Point::new(0.,0.,house_whl), Point::new(house_whl,0.,house_whl), Point::new(house_whl,house_whl,house_whl), Point::new(0.,house_whl,house_whl)], 
            vec![]),

        Polygon::new(vec![Point::new(5.,-2.,15.), Point::new(10.,-2.,15.), Point::new(10.,-2.,17.), Point::new(5.,-2.,17.)], 
            vec![]),
        Polygon::new(vec![Point::new(5.,-2.,15.), Point::new(5.,-2.,17.), Point::new(5.,0.,17.), Point::new(5.,0.,15.)], 
            vec![]),
        Polygon::new(vec![Point::new(10.,-2.,15.), Point::new(10.,-2.,17.), Point::new(10.,0.,17.), Point::new(10.,0.,15.)], 
            vec![]),
        Polygon::new(vec![Point::new(5.,-2.,15.), Point::new(10.,-2.,15.), Point::new(10.,0.,15.), Point::new(5.,0.,15.)], 
            vec![]),
    ];

    Building {
        walls: walls
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Plan {
    building: BuildingForSerialization
}

struct Building {
    walls: Vec<Polygon>
}

impl Building {
    fn triangulation(&self) -> Vec<TriangulizedWall> {
        let mut triangulized_walls: Vec<TriangulizedWall> = vec![];

        for wall in &self.walls {
            let poly_tri = PolygonForTriangulation::from_polygon(wall);

            let triangulation = crate::triangulation::triangulate_3d(&poly_tri);
            
            let mut triangles: Vec<Triangle> = vec![];
            
            let mut i = 0;
            while i<triangulation.len() {
                triangles.push(Triangle {
                    t1: Point::copy_new(&poly_tri.points()[triangulation[i]]),
                    t2: Point::copy_new(&poly_tri.points()[triangulation[i+1]]),
                    t3: Point::copy_new(&poly_tri.points()[triangulation[i+2]]),
                });
                i = i + 3;
            }

            triangulized_walls.push(TriangulizedWall {
                triangles: triangles
            });
        }
    
        triangulized_walls
    }

    fn wireframe(&self) -> Vec<Vec<Point>> {
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

#[derive(Serialize, Deserialize, Debug)]
struct BuildingForSerialization {
    walls: Vec<TriangulizedWall>,
    wireframe: Vec<Vec<Point>>
}

impl BuildingForSerialization {
    fn from_building(building: Building) -> BuildingForSerialization {
        BuildingForSerialization {
             walls: building.triangulation(),
             wireframe: building.wireframe()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TriangulizedWall {
    triangles: Vec<Triangle>
}

#[derive(Serialize, Deserialize, Debug)]
struct Triangle {
    t1: Point,
    t2: Point,
    t3: Point
}
