use crate::pipeline::math::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Triangle { v0, v1, v2 }
    }

    pub fn v0(&self) -> &Vec3 {
        &self.v0
    }

    pub fn v1(&self) -> &Vec3 {
        &self.v1
    }

    pub fn v2(&self) -> &Vec3 {
        &self.v2
    }

    pub fn signed_area(&self) -> f64 {
        Self::edge(&self.v0, &self.v1, &self.v2)
    }

    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        (
            self.v0.min(&self.v1).min(&self.v2),
            self.v0.max(&self.v1).max(&self.v2),
        )
    }

    pub fn edge_weights(&self, p: &Vec3) -> (f64, f64, f64) {
        (
            Self::edge(&self.v1, &self.v2, p),
            Self::edge(&self.v2, &self.v0, p),
            Self::edge(&self.v0, &self.v1, p),
        )
    }

    pub fn interpolate_z(&self, alpha: f64, beta: f64, gamma: f64) -> f64 {
        alpha * self.v0.z() + beta * self.v1.z() + gamma * self.v2.z()
    }

    fn edge(p0: &Vec3, p1: &Vec3, p: &Vec3) -> f64 {
        (p.x() - p0.x()) * (p1.y() - p0.y()) - (p.y() - p0.y()) * (p1.x() - p0.x())
    }
}
