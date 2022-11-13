use request_generation;
use generating_plan::general_geometry::Point;

fn main(){
    let req = request_generation::create_request(3);
    let plan = generating_plan::plan_generation::generate_plan(&req);

    // println!("{}", (Point::new(0., 0., 1.).angle_to(&Point::new(0., 0., -1.)).val()));
}
