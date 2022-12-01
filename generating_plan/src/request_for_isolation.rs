use general_geometry::{Polygon, Point};
use crate::tiling::UnitTile;
use serde::{Serialize, Deserialize};
use crate::building_representations::polygon_walls::PolygonWalls;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    data: Vec<PolygonWithIsolationDetails>,
    unit_tile: UnitTile,
    hooks: Vec<HookSystem>,
    velocity: f64
}

impl Request {
    pub fn data(&self) -> &Vec<PolygonWithIsolationDetails> {
        &self.data
    }

    pub fn unit_tile(&self) -> &UnitTile {
        &self.unit_tile
    }

    pub fn hooks(&self) -> &Vec<HookSystem> {
        &self.hooks
    }

    pub fn velocity(&self) -> f64 {
        self.velocity
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HookSystem {
    robot_hook: Point,
    carrier_hook: Point,
    robot_hook_ground: Point,
    carrier_hook_ground: Point
}

impl HookSystem {

    pub fn new(robot_hook: Point, carrier_hook: Point, robot_hook_ground: Point, carrier_hook_ground: Point) -> Self {
        Self {
            robot_hook, carrier_hook, robot_hook_ground, carrier_hook_ground
        }
    }

    pub fn robot_hook(&self) -> &Point {
        &self.robot_hook
    }

    pub fn carrier_hook(&self) -> &Point {
        &self.carrier_hook
    }

    pub fn robot_hook_ground(&self) -> &Point {
        &self.robot_hook_ground
    }

    pub fn carrier_hook_ground(&self) -> &Point {
        &self.carrier_hook_ground
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonWithIsolationDetails {
    polygon: Polygon,
    isolation: Option<IsolationDetails>
}

impl PolygonWithIsolationDetails {
    pub fn polygon(&self) -> &Polygon {
        &self.polygon
    }

    pub fn isolation(&self) -> &Option<IsolationDetails> {
        &self.isolation
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsolationDetails {
    width: f64,
}

impl IsolationDetails {
    pub fn width(&self) -> f64{
        self.width
    }
}

impl Request {
    pub fn from_polygon_walls_building(building: &PolygonWalls, width: f64, unit_tile: UnitTile, hooks: Vec<HookSystem>, velocity: f64) -> Request {
        let mut data: Vec<PolygonWithIsolationDetails> = vec![];

        for p in building.walls() {
            let plane_normal = &p.normal().normalize();
            let plane_is_horizontal = Point::are_points_simmilar(plane_normal, &Point::new(0., 0., 1.)) || Point::are_points_simmilar(plane_normal, &Point::new(0., 0., -1.));

            data.push(PolygonWithIsolationDetails {
                polygon: p.clone(),
                isolation: if plane_is_horizontal { 
                    Option::None 
                } else { 
                    Option::Some(IsolationDetails {
                        width
                    })
                }
            })
        }

        Request {
            data,
            unit_tile,
            hooks,
            velocity
        }
    }
}
