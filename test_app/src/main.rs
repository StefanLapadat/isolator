use request_generation;
use generating_plan::general_geometry::{Line3D, Point, line3d, PolygonPointsOnSides, Polygon, Polygon2D, CoordinateSystem3D};
use generating_plan::tiling::{UnitTile, Tile};

fn main(){
    // let req = request_generation::create_request(2);
    // let plan = generating_plan::plan_generation::generate_plan(&req);

    // let tile = Tile::new(
    //     PolygonPointsOnSides::new(vec![Point::new(0., 0., 0.), Point::new(1., 0., 0.), Point::new(0.5, 1., 0.), ], vec![]), 
    //     PolygonPointsOnSides::new(vec![Point::new(0., 0., 3.245), Point::new(1., 0., 3.245), Point::new(0.5, 1., 3.245), ], vec![])
    // );

    // let unit_tile = UnitTile::new(Point::new(3.246, 1., 1.)).unwrap();

    // println!("{}", generating_plan::tiling::tile::are_tile_and_unit_tile_compatible(&tile, &unit_tile));

    // println!("{}", (Point::new(0., 0., 1.).angle_to(&Point::new(0., 0., -1.)).val()));

    // let l1 = Line3D::new(Point::new(1., 1., 0.), Point::new(0., 0., 10.)).unwrap();
    // let l2 = Line3D::new(Point::new(20., 12., 3.), Point::new(0., 0., 10.)).unwrap();
    
    // println!("{:?}", line3d::intersection(&l1, &l2));

    let poly = Polygon::from_triplets(vec![(0.,0.,0.), (0.,0.,25.), (0.,25.,25.), (0.,25.,0.)], vec![]);
    let system = poly.coordinate_system_xy_parallel_to_self();

    let poly2d = poly.to_2d(&system);

    println!("{:?}", poly2d);
    println!("{:?}", poly2d.union_box());
    println!("{:?}", poly2d.union_box().to_3d(&system));

    println!("{:?}", system);
    println!("{:?}", CoordinateSystem3D::inverse_system(&system));
}
