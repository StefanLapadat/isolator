use crate::general_geometry::{Polygon, Point, Plane, Simmilar};
use serde::{Serialize, Deserialize};
use crate::building_representations::polygon_walls::PolygonWalls;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    data: Vec<PolygonWithIsolationDetails>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonWithIsolationDetails {
    polygon: Polygon,
    polygon_normal: Point,
    isolation: Option<IsolationDetails>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsolationDetails {
    width: f64,
}

impl Request {
    pub fn from_polygon_walls_building(building: &PolygonWalls, width: f64) -> Request {
        let mut data: Vec<PolygonWithIsolationDetails> = vec![];

        for p in building.walls() {
            let normal = Plane::from_points_vector(p.rim()).unwrap().normal_vector();
            let plane_is_horizontal = normal.x.simmilar_to(0., 0.001) && normal.y.simmilar_to(0., 0.001);

            data.push(PolygonWithIsolationDetails {
                polygon: p.clone(),
                polygon_normal: normal,
                isolation: if plane_is_horizontal { Option::None } else { Option::Some(IsolationDetails {
                    width: width
                })}
            })
        }

        Request {
            data
        }
    }
}