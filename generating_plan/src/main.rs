use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use crate::general_geometry::Point;

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
        building: BuildingForSerialization::from_building_points(building)
    }
}

fn create_building() -> Building {
    let house_whl = 25.0;

    let walls: Vec<Polygon> = vec![
        Polygon::from_points(vec![Point::new(0.,0.,0.), Point::new(10.,0.,0.), Point::new(10.,0.,10.), Point::new(15.,0.,10.), 
            Point::new(15.,0.,0.), Point::new(house_whl,0.,0.), Point::new(house_whl,0.,house_whl), Point::new(0.,0.,house_whl)]),
        Polygon::from_points(vec![Point::new(house_whl,0.,0.), Point::new(house_whl,0.,house_whl), Point::new(house_whl,house_whl,house_whl), Point::new(house_whl,house_whl,0.)]), 
        Polygon::from_points(vec![Point::new(0.,0.,0.), Point::new(0.,0.,25.), Point::new(0.,25.,25.), Point::new(0.,25.,0.)]),
        Polygon::from_points(vec![Point::new(0.,house_whl,0.), Point::new(house_whl,house_whl,0.), Point::new(house_whl,house_whl,house_whl), Point::new(0.,house_whl,house_whl)]),
        Polygon::from_points(vec![Point::new(0.,0.,0.), Point::new(0.,house_whl,0.), Point::new(house_whl,house_whl,0.), Point::new(house_whl,0.,0.)],),
        Polygon::from_points(vec![Point::new(0.,0.,house_whl), Point::new(house_whl,0.,house_whl), Point::new(house_whl,house_whl,house_whl), Point::new(0.,house_whl,house_whl)],),
        Polygon::from_points(vec![Point::new(5.,-3.,15.), Point::new(10.,-3.,15.), Point::new(10.,-3.,20.), Point::new(5.,-3.,20.)]),
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
    
            let triangles = crate::triangulation::triangulate_3d(&wall.points);
            
            match triangles {
                Option::None => panic!("greska teska"),
                Option::Some(triangulation) => {
    
                    let mut triangles: Vec<Triangle> = vec![];
                    
                    let mut i = 0;
                    while i<triangulation.len() {
                        triangles.push(Triangle {
                            t1: Point::copy_new(&wall.points[triangulation[i]]),
                            t2: Point::copy_new(&wall.points[triangulation[i+1]]),
                            t3: Point::copy_new(&wall.points[triangulation[i+2]]),
                        });
                        i = i + 3;
                    }
    
                    triangulized_walls.push(TriangulizedWall {
                        triangles: triangles
                    });
                }
            }
        }
    
        triangulized_walls
    }

    fn wireframe(&self) -> Vec<Vec<Point>> {
        let mut res = vec![];

        for wall in &self.walls {
            let mut seq: Vec<Point> = vec![];
            for point in &wall.points {
                seq.push(Point::copy_new(point));
            }
            if !wall.points.is_empty() {
                seq.push(Point::copy_new(&wall.points[0]));
            }
            res.push(seq);
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
    fn from_building_points(building: Building) -> BuildingForSerialization {
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

struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn from_points(points: Vec<Point>) -> Polygon {
        Polygon {
            points: points
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Triangle {
    t1: Point,
    t2: Point,
    t3: Point
}
