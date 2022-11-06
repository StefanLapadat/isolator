use crate::general_geometry::Polygon;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Levels {
    levels: Vec<Level>
}

impl Levels {
    pub fn new(levels: Vec<Level>) -> Levels {
        Levels {
            levels
        }
    }

    pub fn levels<'a>(&'a self) -> &'a Vec<Level> {
        &self.levels
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    height: f64,
    rim: Polygon
}

impl Level {
    pub fn new(height: f64, rim: Polygon) -> Level {
        Level {
            height,
            rim
        }
    }

    pub fn rim<'a>(&'a self) -> &Polygon {
        &self.rim
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}