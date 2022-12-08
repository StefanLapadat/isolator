use general_geometry::{LineSegment, Point};

fn main(){
    let req = request_generation::create_request(1, 0.5, 2., 0.1, 1.);

    use std::time::Instant;
    let now = Instant::now();

    let plan = generating_plan::plan_generation::generate_plan(&req);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.5?}", elapsed);

    
    println!("tiles: {} adhesive: {}", plan.tiles.triangulized_tiles().tiles().len(), plan.tiles.triangulized_adhesive().tiles().len());
}

