use old_geo_types::{Polygon, polygon, LineString};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let big: Polygon<f64> = polygon!(exterior: [(
        x: 0.,
        y: 0.
    ), (
        x: 300.,
        y: 0.
    ), (
        x: 300.,
        y: 500.
    ), (
        x: 250.,
        y: 500.
    ), (
        x: 250.,
        y: 400.
    ), (
        x: 200.,
        y: 400.
    ), (
        x: 200.,
        y: 500.
    ), (
        x: 0.,
        y: 500.
    ), (
        x: 0.,
        y: 0.
    )], interiors: []);
    
    // let mut polygon = Polygon::new(LineString::from(vec![(0., 0.), (1., 0.), (1., 1.)]), vec![]);

    let small: Polygon<f64>  = polygon!(exterior: [(
        x: -100.,
        y: 700.
    ), (
        x: 150.,
        y: 700.
    ), (
        x: 150.,
        y: 450.
    ), (
        x: 270.,
        y: 450.
    ), (
        x: 270.,
        y: 380.
    ), (
        x: -100.,
        y: 380.
    ), (
        x: -100.,
        y: 700.
    )
    ], interiors: [[
        (
            x: -50.,
            y: 450.
        ), (
            x: 50.,
            y: 450.
        ), (
            x: 50.,
            y: 550.
        ), (
            x: -50.,
            y: 550.
        ), (
            x: -50.,
            y: 450.
        )]
    ]);

    let union = small.intersection(&big);
    for p in union.into_iter() {
        dbg!(p);
    }
}