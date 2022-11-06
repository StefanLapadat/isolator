use serde::{Serialize, Deserialize};
use crate::general_geometry::{Polygon};
use crate::building_representations::{polygon_walls::PolygonWalls, triangulized_walls::TrianguizedWalls, levels::Levels, levels::Level, converters};

pub mod triangulation;
pub mod general_geometry;
pub mod building_representations;

pub fn create_plan() -> Plan {
    Plan {
        building: create_building()
    }
}

fn create_building1() -> TrianguizedWalls {
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

    TrianguizedWalls::from_building(PolygonWalls::new(walls))
}

fn create_building() -> TrianguizedWalls {

    let right = (5.0, 0.0);
    let up = (0.0, 5.0);
    let down = (0., -5.);
    let left = (-5., 0.);

    let levels: Levels = Levels::new(vec![
        Level::new(7., Polygon::in_xy_plane_no_holes_from_increments((-10., -10.), 
        vec![right, up, right, down, right, up, right, down, right, up, up, up, up, left, down, left, up, left, down, left, up, left ])),
        Level::new(5., Polygon::from_triplets(vec![(0.,0.,0.), (10., 0., 0.), (10., 10., 0.), (0., 10., 0.)], vec![])),
        Level::new(8., Polygon::from_triplets(vec![(5.,0.,0.), (10., 5., 0.), (5., 10., 0.), (0., 5., 0.)], vec![])),
        Level::new(3., Polygon::from_triplets(vec![(-5.,-5.,0.), (15., -5., 0.), (15., 15., 0.), (-5., 15., 0.)], vec![])),
    ]);

    TrianguizedWalls::from_building(converters::levels_to_polygon_walls(levels))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    building: TrianguizedWalls
}
