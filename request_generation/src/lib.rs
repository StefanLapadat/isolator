use generating_plan::{request_for_isolation::Request, tiling::UnitTile};
use general_geometry::{Point};

pub mod building1;
pub mod building2;
pub mod building3;

pub fn create_request(request_id: i32, length: f64, height: f64, width: f64, velocity: f64) -> Request {
    let unit_tile = UnitTile::new(Point::new(length, height, width)).unwrap();
    
    match request_id {
        1 =>  Request::from_polygon_walls_building(&building1::create_building_polygon_walls(), width, unit_tile, building1::hooks(), velocity),
        2 =>  Request::from_polygon_walls_building(&building2::create_building_polygon_walls(), width, unit_tile, building2::hooks(), velocity),
        3 =>  Request::from_polygon_walls_building(&building3::create_building_polygon_walls(), width, unit_tile, building3::hooks(), velocity),

        _ => panic!("Not supported request id.")
    }
}
