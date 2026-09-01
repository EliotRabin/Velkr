use crate::pipeline::geometry::fragment::{Fragment, Interpolable};
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::math::mat4::Mat4;
use crate::pipeline::math::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenTriangle<'a> {
    triangle: Triangle<'a>,
    positions: [Vec3; 3],
}

impl<'a> ScreenTriangle<'a> {
    pub fn new(triangle: Triangle<'a>, positions: [Vec3; 3]) -> Self {
        ScreenTriangle { triangle, positions }
    }

    pub fn project(triangle: Triangle<'a>, matrix: &Mat4) -> Self {
        let (p0, p1, p2) = triangle.positions();
        let positions = [*matrix * p0, *matrix * p1, *matrix * p2];
        ScreenTriangle { triangle, positions }
    }

    pub fn triangle(&self) -> &Triangle<'a> {
        &self.triangle
    }

    pub fn positions(&self) -> &[Vec3; 3] {
        &self.positions
    }

    pub fn signed_area(&self) -> f64 {
        let [p0, p1, p2] = self.positions;
        Self::edge(&p0, &p1, &p2)
    }

    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        let [p0, p1, p2] = self.positions;
        (p0.min(&p1).min(&p2), p0.max(&p1).max(&p2))
    }

    pub fn edge_weights(&self, p: &Vec3) -> (f64, f64, f64) {
        let [p0, p1, p2] = self.positions;
        (
            Self::edge(&p1, &p2, p),
            Self::edge(&p2, &p0, p),
            Self::edge(&p0, &p1, p),
        )
    }

    pub fn barycentric(&self, p: &Vec3) -> Option<(f64, f64, f64)> {
        let area = self.signed_area();
        if area == 0.0 {
            return None;
        }

        let (w0, w1, w2) = self.edge_weights(p);
        let alpha = w0 / area;
        let beta = w1 / area;
        let gamma = w2 / area;

        if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
            return None;
        }

        Some((alpha, beta, gamma))
    }

    pub fn interpolate_z(&self, alpha: f64, beta: f64, gamma: f64) -> f64 {
        let [p0, p1, p2] = self.positions;
        f64::interpolate(p0.z(), p1.z(), p2.z(), alpha, beta, gamma)
    }

    pub fn fragment_at(&self, p: &Vec3) -> Option<Fragment> {
        let (alpha, beta, gamma) = self.barycentric(p)?;
        Some(self.triangle.interpolate(alpha, beta, gamma))
    }

    fn edge(p0: &Vec3, p1: &Vec3, p: &Vec3) -> f64 {
        (*p - *p0).cross_z(&(*p1 - *p0))
    }
}
