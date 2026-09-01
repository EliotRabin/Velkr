use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Surface {
    point: Vec3,
    normal: Vec3,
    albedo: Color,
}

impl Surface {
    pub fn new(point: Vec3, normal: Vec3, albedo: Color) -> Self {
        Surface { point, normal, albedo }
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }

    pub fn set_point(&mut self, point: Vec3) {
        self.point = point;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn set_albedo(&mut self, albedo: Color) {
        self.albedo = albedo;
    }
}
