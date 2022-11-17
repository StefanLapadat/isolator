use generating_plan::{request_for_isolation::Request, tiling::UnitTile};
use generating_plan::general_geometry::{Point};

pub mod building1;
pub mod building2;
pub mod building3;

pub fn create_request(request_id: i32) -> Request {
    let width = 0.3;
    let unit_tile = UnitTile::new(Point::new(5.0, 2.0, 0.3)).unwrap();
    
    match request_id {
        1 =>  Request::from_polygon_walls_building(&building1::create_building_polygon_walls(), width, unit_tile),
        2 =>  Request::from_polygon_walls_building(&building2::create_building_polygon_walls(), width, unit_tile),
        3 =>  Request::from_polygon_walls_building(&building3::create_building_polygon_walls(), width, unit_tile),

        _ => panic!("Not supported request id.")
    }
}
