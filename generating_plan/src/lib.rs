use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use crate::general_geometry::{Polygon};
use crate::triangulation::{building_with_polygon_walls::BuildingWithPolygonWalls, building_with_triangulized_walls::BuildingWithTrianguizedWalls};

pub mod triangulation;
pub mod general_geometry;

fn main() -> std::io::Result<()> {
    let plan: Plan = create_plan();

    let mut file = File::create("/home/stefan/Documents/cia/projects/isolator/drawing/public/abc.json")?;

    let serialized_points = serde_json::to_string(&plan).unwrap();

    file.write(serialized_points.as_bytes())?;

    Ok(())
}

pub fn create_plan() -> Plan {
    let building = create_building();

    Plan {
        building: BuildingWithTrianguizedWalls::from_building(building)
    }
}

fn create_building() -> BuildingWithPolygonWalls {
    let house_whl = 25.0;

    let walls: Vec<Polygon> = vec![
        Polygon::from_triplets(vec![(0.,0.,0.), (10.,0.,0.), (10.,0.,10.), (15.,0.,10.), (15.,0.,0.), (house_whl,0.,0.), (house_whl,0.,house_whl), (0.,0.,house_whl)], 
            vec![vec![(5.,0.,15.), (10.,0.,15.), (10.,0.,19.), (5.,0.,19.)]]),
        Polygon::from_triplets(vec![(house_whl,0.,0.), (house_whl,0.,house_whl), (house_whl,house_whl,house_whl), (house_whl,house_whl,0.)], vec![]),
        Polygon::from_triplets(vec![(0.,0.,0.), (0.,0.,25.), (0.,25.,25.), (0.,25.,0.)], vec![]),
        Polygon::from_triplets(vec![(0.,house_whl,0.), (house_whl,house_whl,0.), (house_whl,house_whl,house_whl), (0.,house_whl,house_whl)], vec![]),
        Polygon::from_triplets(vec![(0.,0.,0.), (0.,house_whl,0.), (house_whl,house_whl,0.), (house_whl,0.,0.)], vec![]),
        Polygon::from_triplets(vec![(0.,0.,house_whl), (house_whl,0.,house_whl), (house_whl,house_whl,house_whl), (0.,house_whl,house_whl)], vec![]),
        Polygon::from_triplets(vec![(5.,-2.,15.), (10.,-2.,15.), (10.,-2.,17.), (5.,-2.,17.)], vec![]),
        Polygon::from_triplets(vec![(5.,-2.,15.), (5.,-2.,17.), (5.,0.,17.), (5.,0.,15.)], vec![]),
        Polygon::from_triplets(vec![(10.,-2.,15.), (10.,-2.,17.), (10.,0.,17.), (10.,0.,15.)], vec![]),
        Polygon::from_triplets(vec![(5.,-2.,15.), (10.,-2.,15.), (10.,0.,15.), (5.,0.,15.)], vec![]),
    ];

    BuildingWithPolygonWalls::new(walls)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    building: BuildingWithTrianguizedWalls
}
