// use generating_plan::general_geometry::Polygon;
use old_geo_types::{Polygon, polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    //vec![vec![(1.,0.,0.), (2.,0.,0.), (2.,0.,2.), (1.,0.,2.)]]

    // let p1 = Polygon::from_triplets(vec![(0.,0.,0.), (10.,0.,0.), (10.,0.,10.), (0.,0.,10.),], vec![]);
    // let p2 = Polygon::from_triplets(vec![(10.,0.,2.), (20.,0.,2.), (20.,0.,8.), (10.,0.,8.),], vec![]);

    // let p3 = Polygon::merge_polygons(&p1, &p2);

    // println!("{:?}", p3);

    let big: Polygon<f32> = polygon![
        (x: 0.0, y: 0.0),
        (x: 9.447436, y: -3.2781005),
        (x: 12.725537, y: 6.1693363),
        (x: 3.2781005, y: 9.447436),
    ];

    let small: Polygon<f32>  = polygon![
        (x: 10.103057, y: -1.3886131),
        (x: 19.550493, y: -4.6667137),
        (x: 21.517353, y: 1.0017484),
        (x: 12.069917, y: 4.279849),
        (x: 10.103057, y: -1.3886131)
    ];

    let union = small.union(&big);
    for p in union.into_iter() {
        dbg!(p);
    }

    // let s: f64 = 1.000000000000001;
    // let t: f64 = 1.0;

    // println!("************** {}", s == t);
}

