
fn main(){
    let req = request_generation::create_request(1, 0.5, 0.1, 0.6);
    let plan = generating_plan::plan_generation::generate_plan(&req);
    
    // println!("{}", serde_json::to_string(&plan).unwrap());
    println!("tiles: {} adhesive: {}", plan.tiles.triangulized_tiles().tiles().len(), plan.tiles.triangulized_adhesive().tiles().len());
}

