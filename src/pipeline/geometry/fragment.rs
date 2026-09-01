use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::color::Color;

pub trait Interpolable: Copy {
    fn interpolate(v0: Self, v1: Self, v2: Self, alpha: f64, beta: f64, gamma: f64) -> Self;
}

impl Interpolable for f64 {
    fn interpolate(v0: f64, v1: f64, v2: f64, alpha: f64, beta: f64, gamma: f64) -> f64 {
        alpha * v0 + beta * v1 + gamma * v2
    }
}

impl Interpolable for Vec3 {
    fn interpolate(v0: Vec3, v1: Vec3, v2: Vec3, alpha: f64, beta: f64, gamma: f64) -> Vec3 {
        Vec3::new(
            f64::interpolate(v0.x(), v1.x(), v2.x(), alpha, beta, gamma),
            f64::interpolate(v0.y(), v1.y(), v2.y(), alpha, beta, gamma),
            f64::interpolate(v0.z(), v1.z(), v2.z(), alpha, beta, gamma),
        )
    }
}

impl Interpolable for Color {
    fn interpolate(v0: Color, v1: Color, v2: Color, alpha: f64, beta: f64, gamma: f64) -> Color {
        let channel = |c0: u8, c1: u8, c2: u8| {
            f64::interpolate(c0 as f64, c1 as f64, c2 as f64, alpha, beta, gamma)
                .round()
                .clamp(0.0, 255.0) as u8
        };

        Color::new(
            channel(v0.r(), v1.r(), v2.r()),
            channel(v0.g(), v1.g(), v2.g()),
            channel(v0.b(), v1.b(), v2.b()),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributKind {
    Position,
    Normal,
    Color,
    Uv,
    Custom(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributValue {
    Scalar(f64),
    Vector(Vec3),
    Color(Color),
}

impl AttributValue {
    pub fn as_scalar(&self) -> Option<f64> {
        match self {
            AttributValue::Scalar(scalar) => Some(*scalar),
            _ => None,
        }
    }

    pub fn as_vector(&self) -> Option<Vec3> {
        match self {
            AttributValue::Vector(vector) => Some(*vector),
            _ => None,
        }
    }

    pub fn as_color(&self) -> Option<Color> {
        match self {
            AttributValue::Color(color) => Some(*color),
            _ => None,
        }
    }
}

impl Interpolable for AttributValue {
    fn interpolate(
        v0: AttributValue,
        v1: AttributValue,
        v2: AttributValue,
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) -> AttributValue {
        match (v0, v1, v2) {
            (AttributValue::Scalar(s0), AttributValue::Scalar(s1), AttributValue::Scalar(s2)) => {
                AttributValue::Scalar(f64::interpolate(s0, s1, s2, alpha, beta, gamma))
            }
            (AttributValue::Vector(w0), AttributValue::Vector(w1), AttributValue::Vector(w2)) => {
                AttributValue::Vector(Vec3::interpolate(w0, w1, w2, alpha, beta, gamma))
            }
            (AttributValue::Color(c0), AttributValue::Color(c1), AttributValue::Color(c2)) => {
                AttributValue::Color(Color::interpolate(c0, c1, c2, alpha, beta, gamma))
            }
            _ => v0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attribut {
    kind: AttributKind,
    value: AttributValue,
}

impl Attribut {
    pub fn new(kind: AttributKind, value: AttributValue) -> Self {
        Attribut { kind, value }
    }

    pub fn scalar(kind: AttributKind, scalar: f64) -> Self {
        Attribut::new(kind, AttributValue::Scalar(scalar))
    }

    pub fn vector(kind: AttributKind, vector: Vec3) -> Self {
        Attribut::new(kind, AttributValue::Vector(vector))
    }

    pub fn color(kind: AttributKind, color: Color) -> Self {
        Attribut::new(kind, AttributValue::Color(color))
    }

    pub fn kind(&self) -> &AttributKind {
        &self.kind
    }

    pub fn value(&self) -> &AttributValue {
        &self.value
    }

    pub fn set_value(&mut self, value: AttributValue) {
        self.value = value;
    }
}

impl Interpolable for Attribut {
    fn interpolate(
        v0: Attribut,
        v1: Attribut,
        v2: Attribut,
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) -> Attribut {
        Attribut::new(
            v0.kind,
            AttributValue::interpolate(v0.value, v1.value, v2.value, alpha, beta, gamma),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fragment {
    attributs: Vec<Attribut>,
}

impl Fragment {
    pub fn new() -> Self {
        Fragment { attributs: Vec::new() }
    }

    pub fn with_attributs(attributs: Vec<Attribut>) -> Self {
        Fragment { attributs }
    }

    pub fn from_position(position: Vec3) -> Self {
        Fragment::with_attributs(vec![Attribut::vector(AttributKind::Position, position)])
    }

    pub fn interpolate(
        v0: &Fragment,
        v1: &Fragment,
        v2: &Fragment,
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) -> Self {
        let mut fragment = Fragment::new();
        fragment.interpolate_from(v0, v1, v2, alpha, beta, gamma);
        fragment
    }

    pub fn position(&self) -> Vec3 {
        self.vector(AttributKind::Position).unwrap_or(Vec3::new(0.0, 0.0, 0.0))
    }

    pub fn depth(&self) -> f64 {
        self.position().z()
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.set_attribut(Attribut::vector(AttributKind::Position, position));
    }

    pub fn attributs(&self) -> &Vec<Attribut> {
        &self.attributs
    }

    pub fn attribut(&self, kind: AttributKind) -> Option<&Attribut> {
        self.attributs.iter().find(|attribut| attribut.kind == kind)
    }

    pub fn scalar(&self, kind: AttributKind) -> Option<f64> {
        self.attribut(kind)?.value().as_scalar()
    }

    pub fn vector(&self, kind: AttributKind) -> Option<Vec3> {
        self.attribut(kind)?.value().as_vector()
    }

    pub fn color(&self, kind: AttributKind) -> Option<Color> {
        self.attribut(kind)?.value().as_color()
    }

    pub fn push_attribut(&mut self, attribut: Attribut) {
        self.attributs.push(attribut);
    }

    pub fn set_attribut(&mut self, attribut: Attribut) {
        match self.attributs.iter_mut().find(|held| held.kind == attribut.kind) {
            Some(held) => held.set_value(attribut.value),
            None => self.attributs.push(attribut),
        }
    }

    pub fn remove_attribut(&mut self, kind: AttributKind) {
        self.attributs.retain(|attribut| attribut.kind != kind);
    }

    pub fn clear(&mut self) {
        self.attributs.clear();
    }

    pub fn interpolate_from(
        &mut self,
        v0: &Fragment,
        v1: &Fragment,
        v2: &Fragment,
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) {
        self.attributs.clear();
        for ((a0, a1), a2) in v0.attributs.iter().zip(v1.attributs.iter()).zip(v2.attributs.iter()) {
            self.attributs.push(Attribut::interpolate(*a0, *a1, *a2, alpha, beta, gamma));
        }
    }
}
