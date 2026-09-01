#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    distance: f64,
    u: f64,
    v: f64,
}

impl Hit {
    pub fn new(distance: f64, u: f64, v: f64) -> Self {
        Hit { distance, u, v }
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }

    pub fn barycentric(&self) -> (f64, f64, f64) {
        (1.0 - self.u - self.v, self.u, self.v)
    }
}
