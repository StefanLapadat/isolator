use serde::{Serialize, Deserialize};
use crate::Point;

#[derive(Serialize, Deserialize, Debug)]
pub struct Triangle {
    t1: Point,
    t2: Point,
    t3: Point
}

impl Triangle {
    pub fn new(t1: &Point, t2: &Point, t3: &Point) -> Triangle {
        Triangle {
            t1: t1.clone(),
            t2: t2.clone(),
            t3: t3.clone()
        }
    }
}