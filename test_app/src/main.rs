use general_geometry::{LineSegment, Point};


fn main(){
    // let req = request_generation::create_request(1, 2., 2., 0.1);
    // let plan = generating_plan::plan_generation::generate_plan(&req);
    
    // println!("{:?}", plan.planExecution.events().len());

    // println!("tiles: {} adhesive: {}", plan.tiles.triangulized_tiles().tiles().len(), plan.tiles.triangulized_adhesive().tiles().len());

    let seg = LineSegment::new(Point::new(0., -1., 0.), Point::new(0., -1., 10.));
    println!("{}", seg.distance_from_point(&Point::new(0., 0., 4.)));
}

