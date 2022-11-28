use general_geometry::{Polygon, Point};
use crate::tiling::UnitTile;
use serde::{Serialize, Deserialize};
use crate::building_representations::polygon_walls::PolygonWalls;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    data: Vec<PolygonWithIsolationDetails>,
    unit_tile: UnitTile,
    hooks: Vec<HookPair>
}

impl Request {
    pub fn data(&self) -> &Vec<PolygonWithIsolationDetails> {
        &self.data
    }

    pub fn unit_tile(&self) -> &UnitTile {
        &self.unit_tile
    }

    pub fn hooks(&self) -> &Vec<HookPair> {
        &self.hooks
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HookPair {
    robot_hook: Point,
    carrier_hook: Point
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
    pub fn from_polygon_walls_building(building: &PolygonWalls, width: f64, unit_tile: UnitTile, hooks: Vec<HookPair>) -> Request {
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
            hooks
        }
    }
}
