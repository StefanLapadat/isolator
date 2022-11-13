use request_generation;

fn main(){
    let req = request_generation::create_request(1);
    let plan = generating_plan::plan_generation::generate_plan(&req);
}