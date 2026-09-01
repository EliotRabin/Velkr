use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color::from_f64(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    pub fn from_f64(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn r(&self) -> u8 {
        Self::byte(self.r)
    }

    pub fn g(&self) -> u8 {
        Self::byte(self.g)
    }

    pub fn b(&self) -> u8 {
        Self::byte(self.b)
    }

    pub fn red(&self) -> f64 {
        self.r
    }

    pub fn green(&self) -> f64 {
        self.g
    }

    pub fn blue(&self) -> f64 {
        self.b
    }

    fn byte(value: f64) -> u8 {
        (value.clamp(0.0, 1.0) * 255.0).round() as u8
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::from_f64(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color::from_f64(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        color * self
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::from_f64(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
