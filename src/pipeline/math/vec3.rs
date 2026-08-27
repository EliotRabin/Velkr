use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    pub fn distance(&self, other: &Vec3) -> f64 {
        let dx: f64 = self.x - other.x;
        let dy = self.y - other.y;
        let dz: f64 = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    pub fn midpoint(&self, other: &Vec3) -> Vec3 {
        let mx: f64 = (self.x + other.x) / 2.0;
        let my: f64 = (self.y + other.y) / 2.0;
        let mz: f64 = (self.z + other.z) / 2.0;
        Vec3::new(mx, my, mz)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let x: f64 = self.y * other.z - self.z * other.y;
        let y: f64 = self.z * other.x - self.x * other.z;
        let z: f64 = self.x * other.y - self.y * other.x;
        Vec3::new(x, y, z)
    }

    pub fn normalize(&self) -> Vec3 {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length == 0.0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            Vec3::new(self.x / length, self.y / length, self.z / length)
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn min(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn max(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3::new(
            vector.x * self,
            vector.y * self,
            vector.z * self,
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}