use general_geometry::{LineSegment, Point};


fn main(){
    let req = request_generation::create_request(1, 2., 2., 0.1, 0.1);
    let plan = generating_plan::plan_generation::generate_plan(&req);
    
    println!("{:?}",     serde_json::to_string(&plan.planExecution.events()).unwrap());

    // println!("tiles: {} adhesive: {}", plan.tiles.triangulized_tiles().tiles().len(), plan.tiles.triangulized_adhesive().tiles().len());
}

