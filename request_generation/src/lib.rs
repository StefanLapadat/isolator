use generating_plan::request_for_isolation::Request;

pub mod building1;
pub mod building2;
pub mod building3;


pub fn create_request(request_id: i32) -> Request {
    match request_id {
        1 =>  Request::from_polygon_walls_building(&building1::create_building_polygon_walls(), 0.2),
        2 =>  Request::from_polygon_walls_building(&building2::create_building_polygon_walls(), 0.2),
        3 =>  Request::from_polygon_walls_building(&building3::create_building_polygon_walls(), 0.2),


        _ => panic!("Not supported request id.")
    }
}
