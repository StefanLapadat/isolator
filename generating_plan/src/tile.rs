use serde::{Serialize, Deserialize};
use crate::general_geometry::{Point, Triangle};

pub struct Tile {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulizedTile {
    triangles: Vec<Triangle>
}
