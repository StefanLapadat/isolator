#[derive(Clone, Debug)]
pub struct Point2D {
    x: f64,
    y: f64
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D{x, y}
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64{
        self.y
    }
}