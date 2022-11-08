use crate::general_geometry::{Polygon};
use crate::building_representations::{polygon_walls::PolygonWalls, triangulized_walls::TrianguizedWalls, levels::Levels, levels::Level, converters};
use crate::request_for_isolation::Request;
use crate::plan_generation::Plan;

pub mod triangulation;
pub mod general_geometry;
pub mod building_representations;
pub mod request_for_isolation;
pub mod tile;
pub mod plan_generation;

fn create_building_triangulized_1() -> TrianguizedWalls {
    converters::polygon_walls_to_triangulized_walls(create_building_polygon_walls_1())
}

fn create_building_polygon_walls_1() -> PolygonWalls {
    let house_whl = 25.0;

    let walls: Vec<Polygon> = vec![
        Polygon::from_triplets(vec![(0.,0.,0.), (10.,0.,0.), (10.,0.,10.), (15.,0.,10.), (15.,0.,0.), (house_whl,0.,0.), (house_whl,0.,house_whl), (0.,0.,house_whl)], 
            vec![vec![(5.,0.,15.), (10.,0.,15.), (10.,0.,19.), (5.,0.,19.)]]),
        Polygon::from_triplets(vec![(house_whl,0.,0.),  (house_whl,house_whl,0.), (house_whl,house_whl,house_whl),(house_whl,0.,house_whl)], vec![]),

        Polygon::from_triplets(vec![(0.,0.,0.), (0.,0.,25.), (0.,25.,25.), (0.,25.,0.)], vec![]),
        Polygon::from_triplets(vec![(0.,house_whl,0.),(0.,house_whl,house_whl),(house_whl,house_whl,house_whl), (house_whl,house_whl,0.)], vec![]),
        Polygon::from_triplets(vec![(0.,0.,0.), (0.,house_whl,0.), (house_whl,house_whl,0.), (house_whl,0.,0.)], vec![]),
        Polygon::from_triplets(vec![(0.,0.,house_whl), (house_whl,0.,house_whl), (house_whl,house_whl,house_whl), (0.,house_whl,house_whl)], vec![]),
       
        Polygon::from_triplets(vec![(5.,-2.,15.), (10.,-2.,15.), (10.,-2.,17.), (5.,-2.,17.)], vec![]),
        Polygon::from_triplets(vec![(5.,-2.,15.), (5.,-2.,17.), (5.,0.,17.), (5.,0.,15.)], vec![]),
        Polygon::from_triplets(vec![(10.,-2.,15.), (10.,0.,15.), (10.,0.,17.), (10.,-2.,17.)], vec![]),
        Polygon::from_triplets(vec![(5.,-2.,15.), (10.,-2.,15.), (10.,0.,15.), (5.,0.,15.)], vec![]),
    ];

    PolygonWalls::new(walls)
}

pub fn create_building_triangulized() -> TrianguizedWalls {
    converters::polygon_walls_to_triangulized_walls(create_building_polygon_walls())
}

pub fn create_building_polygon_walls() -> PolygonWalls {
    converters::levels_to_polygon_walls(create_building_levels())
}

pub fn create_building_levels() -> Levels {
    let right0 = (5.0, 0.0);
    let up0 = (0.0, 5.0);
    let down0 = (0., -5.);
    let left0 = (-5., 0.);

    Levels::new(vec![
        Level::new(7., Polygon::in_xy_plane_no_holes_from_increments((-10., -10.), 
        vec![right0, up0, right0, down0, right0, up0, right0, down0, right0, up0, up0, up0, up0, left0, down0, left0, up0, left0, down0, left0, up0, left0 ])),
        Level::new(5., Polygon::from_triplets(vec![(0.,0.,0.), (10., 0., 0.), (10., 10., 0.), (0., 10., 0.)], vec![])),
        Level::new(8., Polygon::from_triplets(vec![(5.,0.,0.), (10., 5., 0.), (5., 10., 0.), (0., 5., 0.)], vec![])),
        Level::new(3., Polygon::from_triplets(vec![(-5.,-5.,0.), (15., -5., 0.), (15., 15., 0.), (-5., 15., 0.)], vec![])),
    ])
}

pub fn create_request() -> Request {
    Request::from_polygon_walls_building(&create_building_polygon_walls_1(), 0.5)
}

pub fn create_plan(request: &Request) -> Plan {
    plan_generation::generate_plan(request)
}
