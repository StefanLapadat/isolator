use generating_plan::general_geometry::{Polygon};
use generating_plan::building_representations::{polygon_walls::PolygonWalls};

pub(crate) fn create_building_polygon_walls() -> PolygonWalls {
    let house_whl = 25.0;

    let walls: Vec<Polygon> = vec![
        
        // Polygon::from_triplets(vec![(15.0, 10.0, 0.0), (10.0, 10.0, 0.0), (10.0, 10.0, 7.0), (15.0, 10.0, 7.0)], vec![]),

        // Polygon::from_triplets(vec![(10.0, 10.0, 7.0),(0.0, 10.0, 7.0),(0.0, 10.0, 12.0), (10.0, 10.0, 12.0)], vec![]),

        // Polygon::from_triplets(vec![(5.0, 10.0, 0.0), (0.0, 10.0, 0.0), (0.0, 10.0, 7.0), (5.0, 10.0, 7.0)], vec![]),

        Polygon::from_triplets(vec![(0.0, 10.0, 0.0), (0.0, 5.0, 0.0), (0.0, 5.0, 7.0), (0.0, 10.0, 7.0)], vec![]),

        Polygon::from_triplets(vec![(0.0, 10.0, 7.0), (0.0, 0.0, 7.0), (0.0, 0.0, 12.0), (0.0, 10.0, 12.0)], vec![]),

        // Polygon::from_triplets(vec![(0.0, 0.0, 0.0), (10.0, 0.0, 0.0), (10.0, 0.0, 10.0), (0.0, 0.0, 10.0)], vec![]),

        // Polygon::from_triplets(vec![(10.0, 0.0, 2.0), (20.0, 0.0, 2.0), (20.0, 0.0, 8.0), (10.0, 0.0, 8.0)], vec![]),
        
    ];

    PolygonWalls::new(walls)
}