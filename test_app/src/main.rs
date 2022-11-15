use request_generation;
use generating_plan::general_geometry::{Line3D, Point, line3d};

fn main(){
    // let req = request_generation::create_request(3);
    // let plan = generating_plan::plan_generation::generate_plan(&req);

    // println!("{}", (Point::new(0., 0., 1.).angle_to(&Point::new(0., 0., -1.)).val()));

    let l1 = Line3D::new(Point::new(1., 1., 0.), Point::new(0., 0., 10.)).unwrap();
    let l2 = Line3D::new(Point::new(20., 12., 3.), Point::new(0., 0., 10.)).unwrap();
    
    println!("{:?}", line3d::intersection(&l1, &l2));
}
