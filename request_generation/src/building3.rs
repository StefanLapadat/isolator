use generating_plan::general_geometry::{Polygon};
use generating_plan::building_representations::{polygon_walls::PolygonWalls};

pub(crate) fn create_building_polygon_walls() -> PolygonWalls {

    let walls: Vec<Polygon> = vec![
        Polygon::from_triplets(vec![(0.0, 0.0, 0.0), (10.0, 0.0, 0.0), (10.0, 10.0, 0.0), (0.0, 10.0, 0.0)], 
        vec![vec![(1.0, 1.0, 0.0), (2.0, 1.0, 0.0), (2.0, 2.0, 0.0), (1.0, 2.0, 0.0)]]),
        Polygon::from_triplets(vec![(10.0, 0.0, 0.0), (20.0, 0.0, 0.0), (20.0, 10.0, 0.0), (10.0, 10.0, 0.0)], vec![]),
    ];

    PolygonWalls::new(walls)
}