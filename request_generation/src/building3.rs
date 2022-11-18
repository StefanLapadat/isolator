use generating_plan::general_geometry::{Polygon};
use generating_plan::building_representations::{polygon_walls::PolygonWalls};

pub(crate) fn create_building_polygon_walls() -> PolygonWalls {

    let house_whl = 25.0;

    let walls: Vec<Polygon> = vec![
        Polygon::from_triplets(vec![(house_whl,0.,0.), (house_whl,house_whl,0.), (house_whl,house_whl,house_whl),(house_whl,0.,house_whl)], 
        vec![]),
    ];

    // let walls: Vec<Polygon> = vec![
    //     Polygon::from_triplets(vec![(0.0, 0.0, 0.0), (10.0, 0.0, 0.0), (10.0, 0.0, 10.0), (0.0, 0.0, 10.0)], vec![]),
    //     Polygon::from_triplets(vec![(10.0, 0.0, 0.0), (3.0, 3.0, 0.0), (3.0, 3.0, 10.0), (10.0, 0.0, 10.0)], vec![]),
    // ];

    // let walls: Vec<Polygon> = vec![
    //     Polygon::from_triplets(vec![(0.0, 0.0, 2.0), (10.0, 0.0, 0.0), (10.0, 10.0, 0.0), (0.0, 10.0, 2.0)], vec![]),
    //     Polygon::from_triplets(vec![(10.0, 0.0, 0.0), (10.0, 0.0, 10.0), (10.0, 10.0, 10.0), (10.0, 10.0, 0.0)], vec![]),
    //     Polygon::from_triplets(vec![(0.0, 0.0, 2.0), (0.0, -10.0, 6.), (10.0, -10.0, 4.0), (10.0, 0.0, 0.0)], vec![]),
    // ];

    // let walls: Vec<Polygon> = vec![
    //     Polygon::from_triplets(vec![(0.0, 0.0, 0.0), (10.0, 0.0, 0.0), (10.0, 0.0, 10.0), (0.0, 0.0, 10.0)], vec![]),
    //     Polygon::from_triplets(vec![(10.0, 0.0, 0.0), (10.0, 10.0, 0.0), (10.0, 10.0, 10.0), (10.0, 0.0, 10.0)], vec![]),
    // ];

    // let walls: Vec<Polygon> = vec![
    //     Polygon::from_triplets(vec![(0.0, 0.0, 0.0), (10.0, 0.0, 0.0), (10.0, 10.0, 0.0), (0.0, 10.0, 0.0)], vec![]),
    //     Polygon::from_triplets(vec![(10.0, 0.0, 0.0), (20.0, 0.0, 0.0), (20.0, 10.0, 0.0), (10.0, 10.0, 0.0)], vec![]),
    // ];

    // let walls: Vec<Polygon> = vec![
    //     Polygon::from_triplets(vec![(0.0, 0.0, 2.0), (10.0, 0.0, 0.0), (10.0, 10.0, 0.0), (0.0, 10.0, 2.0)], vec![]),
    //     Polygon::from_triplets(vec![(10.0, 0.0, 0.0), (10.0, 0.0, 10.0), (10.0, 10.0, 10.0), (10.0, 10.0, 0.0)], vec![]),
    //     Polygon::from_triplets(vec![(0.0, 0.0, 2.0), (0.0, -10.0, 6.), (10.0, -10.0, 4.0), (10.0, 0.0, 0.0)], vec![]),
    // ];


    // let right0 = (5.0, 0.0);
    // let up0 = (0.0, 5.0);
    // let down0 = (0., -5.);
    // let left0 = (-5., 0.);

    // let walls: Vec<Polygon> = vec![
    //     Polygon::in_xy_plane_no_holes_from_increments((-10., -10.), 
    //     vec![right0, up0, right0, down0, right0, up0, right0, down0, right0, up0, up0, up0, up0, left0, down0, left0, up0, left0, down0, left0, up0, left0 ]),
    //     Polygon::in_xy_plane_no_holes_from_increments((-12., 10.), 
    //     vec![right0, up0, right0, down0, right0, up0, right0, down0, right0, up0, up0, up0, up0, left0, down0, left0, up0, left0, down0, left0, up0, left0 ]),
    // ];

    // let walls: Vec<Polygon> = vec![
    //     Polygon::new(vec![Point { x: 0.0, y: 10.0, z: 0.0 }, Point { x: 0.0, y: 5.0, z: 0.0 }, Point { x: 0.0, y: 5.0, z: 7.0 }, Point { x: 0.0, y: 10.0, z: 7.0 }], vec![]),
    //     Polygon::new(vec![Point { x: 0.0, y: 16.0, z: 7.0 }, Point { x: 0.0, y: 5.0, z: 7.0 }, Point { x: 0.0, y: 5.0, z: 12.0 }, Point { x: 0.0, y: 16.0, z: 12.0 }], vec![]),
    // ];

    // let walls: Vec<Polygon> = vec![
    //     Polygon::new(vec![Point { x: 0.0, y: 10.0, z: 0.0 }, Point { x: 0.0, y: 5.0, z: 0.0 }, Point { x: 0.0, y: 5.0, z: 7.0 }, Point { x: 0.0, y: 10.0, z: 7.0 }], vec![]),
    //     Polygon::new(vec![Point { x: 0.0, y: 10.0, z: 7.0 }, Point { x: 0.0, y: 0.0, z: 7.0 }, Point { x: 0.0, y: 0.0, z: 12.0 }, Point { x: 0.0, y: 10.0, z: 12.0 }], vec![]),
    // ];

    PolygonWalls::new(walls)
}
