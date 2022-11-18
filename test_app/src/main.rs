use request_generation;
use generating_plan::general_geometry::{Line3D, Point, Plane, line3d, PolygonPointsOnSides, Polygon, Polygon2D, CoordinateSystem3D};
use generating_plan::tiling::{UnitTile, Tile};

fn main(){
    // let req = request_generation::create_request(1);
    // let plan = generating_plan::plan_generation::generate_plan(&req);

    test_distance_between_origin_and_plane();

    // let tile = Tile::new(
    //     PolygonPointsOnSides::new(vec![Point::new(0., 0., 0.), Point::new(1., 0., 0.), Point::new(0.5, 1., 0.), ], vec![]), 
    //     PolygonPointsOnSides::new(vec![Point::new(0., 0., 3.245), Point::new(1., 0., 3.245), Point::new(0.5, 1., 3.245), ], vec![])
    // );

    // let unit_tile = UnitTile::new(Point::new(3.246, 1., 1.)).unwrap();

    // println!("{}", generating_plan::tiling::tile::are_tile_and_unit_tile_compatible(&tile, &unit_tile));

    // let l1 = Line3D::new(Point::new(1., 1., 0.), Point::new(0., 0., 10.)).unwrap();
    // let l2 = Line3D::new(Point::new(20., 12., 3.), Point::new(0., 0., 10.)).unwrap();
    
    // println!("{:?}", line3d::intersection(&l1, &l2));
}

fn test_mapping_coordinates_from_2d_to_3d_and_back() {
    let poly = Polygon::from_triplets(vec![(0.,0.,0.), (0.,0.,25.), (0.,25.,25.), (0.,25.,0.)], vec![]);
    let original_distance_from_origin = Point::new(0., 0., 0.);
    let system = poly.coordinate_system_xy_parallel_to_self();

    let poly2d = poly.to_2d(&system);

    println!("{:?}", poly2d);
    println!("{:?}", poly2d.union_box());
    println!("{:?}", poly2d.union_box().to_3d(&system, &original_distance_from_origin));

    println!("{:?}", system);
    println!("{:?}", CoordinateSystem3D::inverse_system(&system));
}

fn test_angles() {
    println!("{}", (Point::new(0.0, 632.5, 0.0).angle_to(&Point::new(0.0, 625.0, 0.0)).val()));
}

fn test_distance_between_planes() {
    let p1 = Plane::from_points_vector(&vec![Point::new(0., 0., 0.), Point::new(0., 5., 0.), Point::new(0., 5., 5.)]).unwrap();
    let p2 = Plane::from_points_vector(&vec![Point::new(3., 0., 0.), Point::new(3., 5., 0.), Point::new(3., 5., 5.)]).unwrap();

    let p1 = Plane::new(0.0, 632.5 * (625./632.5), 0.0 ,-16002.25 * (625./632.5));
    let p2 = Plane::new(0.0 ,625.0 ,0.0 ,-15625.0);
    
    println!("{}",  (p1.d() - p2.d()).abs() / p1.normal_vector().modulo());
}

fn test_distance_between_origin_and_plane() {
    let p = Polygon::from_triplets(vec![(5.,-2.,15.), (5.,-2.,17.), (5.,0.,17.), (5.,0.,15.)], vec![]);
    println!("{:?}", p.plane());
    println!("{}", p.plane().above_origin());
}