use old_geo_types::{Polygon, polygon};
use geo_booleanop::boolean::BooleanOp;

fn main() {
    // test_problematic_union();
    test_intersection_empty_result();
}

fn test_intersection_1() {
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

fn test_intersection_empty_result() {
    let big: Polygon<f64> = polygon!(exterior: [(
        x: 0.,
        y: 0.
    ), (
        x: 10.,
        y: 0.
    ), (
        x: 10.,
        y: 10.
    ), (
        x: 0.,
        y: 0.
    )], interiors: []);

    let small: Polygon<f64> = polygon!(exterior: [(
        x: 20.,
        y: 0.
    ), (
        x: 30.,
        y: 0.
    ), (
        x: 30.,
        y: 10.
    ), (
        x: 20.,
        y: 0.
    )], interiors: []);


    let res = small.intersection(&big);
    for p in res.into_iter() {
        dbg!(p);
    }
}

fn test_problematic_union() {

    let p1: Polygon<f32> = polygon!(
        exterior: [
            ( x: -8.315300828559245, y: -5.554797217771496 ),
            ( x: -4.1576504142796225, y: -2.777398608885748 ),
            ( x: -8.04600846671967, y: 3.043311971105723 ),
            ( x: -12.203658880999292, y: 0.2659133622199752 ),
            ( x: -8.315300828559245, y: -5.554797217771496 )
        ], 
        interiors: []
    );

    let p2: Polygon<f32> = polygon!(
        exterior: [
            ( x: -12.203658880999292, y: 0.1659133622199752 ),
            ( x: -3.8883580524400467, y: 5.820710579991471 ),
            ( x: -6.665756661325794, y: 9.978360994271092 ),
            ( x: -14.981057489885039, y: 4.423563776499597 ),
            ( x: -12.203658880999292, y: 0.1659133622199752 )
        ], 
        interiors: []
    );

    let union = p1.union(&p2);

    println!("{:?}", union);
}