use generating_plan::general_geometry::{Polygon, Point};
// use old_geo_types::{Polygon, polygon};
// use geo_booleanop::boolean::BooleanOp;

fn main() {
    let p1 = Polygon::from_triplets(vec![(0.,0.,0.), (10.,0.,0.), (10.,0.,10.), (0.,0.,10.),], vec![]);
    let p2 = Polygon::from_triplets(vec![(10.,0.,2.), (20.,0.,2.), (20.,0.,8.), (10.,0.,8.),], vec![]);

    let p3 = Polygon::merge_polygons(&p1, &p2);
    println!("{:?}", p3);
}

