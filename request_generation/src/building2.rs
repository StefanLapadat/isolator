use general_geometry::{Polygon, Point};
use generating_plan::building_representations::{polygon_walls::PolygonWalls, triangulized_walls::TrianguizedWalls, levels::Levels, levels::Level, converters};
use generating_plan::{request_for_isolation::HookSystem};

pub fn create_building_triangulized() -> TrianguizedWalls {
    converters::polygon_walls_to_triangulized_walls(create_building_polygon_walls())
}

pub(crate) fn create_building_polygon_walls() -> PolygonWalls {
    converters::levels_to_polygon_walls(create_building_levels())
}

fn create_building_levels() -> Levels {
    let right0 = (5.0, 0.0);
    let up0 = (0.0, 5.0);
    let down0 = (0., -5.);
    let left0 = (-5., 0.);
    let ld = (-5.0, -5.0);
    let rd = (5.0, -5.0);

    Levels::new(vec![
        Level::new(7., Polygon::in_xy_plane_no_holes_from_increments((-10., -10.), 
        vec![right0, up0, right0, down0, right0, up0, right0, down0, right0, up0, up0, up0, up0, left0, down0, left0, up0, left0, down0, left0, up0, left0, ld, down0 ])),
        Level::new(5., Polygon::from_triplets(vec![(0.,0.,0.), (10., 0., 0.), (10., 10., 0.), (0., 10., 0.)], vec![])),
        Level::new(8., Polygon::from_triplets(vec![(5.,0.,0.), (10., 5., 0.), (5., 10., 0.), (0., 5., 0.)], vec![])),
        Level::new(3., Polygon::from_triplets(vec![(-5.,-5.,0.), (15., -5., 0.), (15., 15., 0.), (-5., 15., 0.)], vec![])),
    ])
}

pub(crate) fn hooks() -> Vec<HookSystem> {
    vec![
        HookSystem::new(Point::new(-9., -11., 10.), Point::new(-10., -11., 10.), Point::new(-9., -11., 0.), Point::new(-10., -11., 0.))
    ]
}