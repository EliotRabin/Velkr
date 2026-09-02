use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn distance(&self, other: &Vec2) -> f64 {
        (*self - *other).length()
    }

    pub fn midpoint(&self, other: &Vec2) -> Vec2 {
        self.lerp(other, 0.5)
    }

    pub fn lerp(&self, other: &Vec2, t: f64) -> Vec2 {
        *self + (*other - *self) * t
    }

    pub fn normalize(&self) -> Vec2 {
        let length = self.length();
        if length == 0.0 {
            Vec2::new(0.0, 0.0)
        } else {
            *self / length
        }
    }

    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn fract(&self) -> Vec2 {
        Vec2::new(
            self.x - self.x.floor(),
            self.y - self.y.floor(),
        )
    }

    pub fn min(&self, other: &Vec2) -> Vec2 {
        Vec2::new(
            self.x.min(other.x),
            self.y.min(other.y),
        )
    }

    pub fn max(&self, other: &Vec2) -> Vec2 {
        Vec2::new(
            self.x.max(other.x),
            self.y.max(other.y),
        )
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(
            self.x + other.x,
            self.y + other.y,
        )
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(
            self.x - other.x,
            self.y - other.y,
        )
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(
            self.x * scalar,
            self.y * scalar,
        )
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, vector: Vec2) -> Vec2 {
        Vec2::new(
            vector.x * self,
            vector.y * self,
        )
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f64) -> Vec2 {
        Vec2::new(
            self.x / scalar,
            self.y / scalar,
        )
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}
