use crate::pipeline::geometry::fragment::Fragment;
use crate::pipeline::math::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle<'a> {
    v0: &'a Fragment,
    v1: &'a Fragment,
    v2: &'a Fragment,
}

impl<'a> Triangle<'a> {
    pub fn new(v0: &'a Fragment, v1: &'a Fragment, v2: &'a Fragment) -> Self {
        Triangle { v0, v1, v2 }
    }

    pub fn v0(&self) -> &'a Fragment {
        self.v0
    }

    pub fn v1(&self) -> &'a Fragment {
        self.v1
    }

    pub fn v2(&self) -> &'a Fragment {
        self.v2
    }

    pub fn positions(&self) -> (Vec3, Vec3, Vec3) {
        (self.v0.position(), self.v1.position(), self.v2.position())
    }

    pub fn edges(&self) -> (Vec3, Vec3) {
        let (p0, p1, p2) = self.positions();
        (p1 - p0, p2 - p0)
    }

    pub fn normal(&self) -> Vec3 {
        let (edge0, edge1) = self.edges();
        edge0.cross(&edge1).normalize()
    }

    pub fn area(&self) -> f64 {
        let (edge0, edge1) = self.edges();
        edge0.cross(&edge1).length() / 2.0
    }

    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        let (p0, p1, p2) = self.positions();
        (p0.min(&p1).min(&p2), p0.max(&p1).max(&p2))
    }

    pub fn interpolate(&self, alpha: f64, beta: f64, gamma: f64) -> Fragment {
        Fragment::interpolate(self.v0, self.v1, self.v2, alpha, beta, gamma)
    }
}
