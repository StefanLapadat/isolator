use general_geometry::{LineSegment, Point};


fn main(){
    // let req = request_generation::create_request(1, 0.9, 0.9, 0.6);
    // let plan = generating_plan::plan_generation::generate_plan(&req);
    
    // println!("tiles: {} adhesive: {}", plan.tiles.triangulized_tiles().tiles().len(), plan.tiles.triangulized_adhesive().tiles().len());

    let seg = LineSegment::new(Point::new(0., 0., 0.), Point::new(10., 0., 0.));
    let point = Point::new(8., 0., 0.);

    println!("{} ***** ", seg.distance_from_point(&point));
}

