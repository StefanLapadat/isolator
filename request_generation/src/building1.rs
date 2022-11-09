use generating_plan::general_geometry::{Polygon};
use generating_plan::building_representations::{polygon_walls::PolygonWalls};

pub(crate) fn create_building_polygon_walls() -> PolygonWalls {
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