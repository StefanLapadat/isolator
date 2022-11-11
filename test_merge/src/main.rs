use old_geo_types::{Polygon, polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let big: Polygon<f64> = polygon![
        (x: 416., y: 256.),
        (x: 432., y: 240.),
        (x: 432., y: 224.),
        (x: 448., y: 280.),
        (x: 480., y: 208.),
        (x: 480., y: 256.),
    ];

    let small: Polygon<f64>  = polygon![
        (x: 400., y: 272.),
        (x: 416., y: 256.),
        (x: 480., y: 256.),
        (x: 480., y: 272.),
    ];

    let union = small.union(&big);
    for p in union.into_iter() {
        dbg!(p);
    }
}