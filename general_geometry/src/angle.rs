use std::f64::consts::PI;

pub struct Angle {
    val: f64
}

impl Angle {
    pub fn new(val: f64) -> Angle {
        Angle {
            val: val % (2.0 * PI)
        }
    }

    pub fn val(&self) -> f64 {
        self.val
    } 
}