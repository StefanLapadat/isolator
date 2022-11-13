pub struct PositiveF64 {
    val: f64
}

impl PositiveF64 {
    pub fn new(val: f64) -> Option<PositiveF64> {
        if val >= 0. { Some(PositiveF64 {val}) } else {None}
    }

    pub fn val(&self) -> f64 {
        self.val
    }
}