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
    let all_points = vec![
        vec![Point::new(0.,0.,0.), Point::new(10.,0.,0.), Point::new(10.,0.,10.), Point::new(15.,0.,10.), 
            Point::new(15.,0.,0.), Point::new(25.,0.,0.), Point::new(25.,0.,25.), Point::new(0.,0.,25.)],
        vec![Point::new(0.,0.,0.), Point::new(0.,0.,25.), Point::new(0.,25.,25.), Point::new(0.,25.,0.)],
        vec![Point::new(25.,0.,0.), Point::new(25.,0.,25.), Point::new(25.,25.,25.), Point::new(25.,25.,0.)], 
        vec![Point::new(0.,25.,0.), Point::new(25.,25.,0.), Point::new(25.,25.,25.), Point::new(0.,25.,25.)],
        vec![Point::new(0.,0.,0.), Point::new(0.,25.,0.), Point::new(25.,25.,0.), Point::new(25.,0.,0.)],
        vec![Point::new(0.,0.,25.), Point::new(25.,0.,25.), Point::new(25.,25.,25.), Point::new(0.,25.,25.)],
        vec![Point::new(5.,-3.,15.), Point::new(10.,-3.,15.), Point::new(10.,-3.,20.), Point::new(5.,-3.,20.)],
    ];

    let mut walls: Vec<TriangulizedWall> = vec![];

    for points in all_points {

    let triangles = crate::triangulation::triangulate_3d(&points);
    
    match triangles {
        Option::None => panic!("greska teska"),
        Option::Some(triangulation) => {

            let mut triangles: Vec<Triangle> = vec![];
            
            let mut i = 0;
            while i<triangulation.len() {
                triangles.push(Triangle {
                    t1: Point::copy_new(&points[triangulation[i]]),
                    t2: Point::copy_new(&points[triangulation[i+1]]),
                    t3: Point::copy_new(&points[triangulation[i+2]]),
                });
                i = i + 3;
            }

            walls.push(TriangulizedWall {
                triangles: triangles
            });
        }
    }
}
    Plan {
        building: Building {
            walls: walls
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Plan {
    building: Building
}

#[derive(Serialize, Deserialize, Debug)]
struct Building {
    walls: Vec<TriangulizedWall>
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

