use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Surface {
    point: Vec3,
    normal: Vec3,
    view: Vec3,
    albedo: Color,
    shininess: f64,
}

impl Surface {
    pub fn new(point: Vec3, normal: Vec3, view: Vec3, albedo: Color, shininess: f64) -> Self {
        Surface { point, normal, view, albedo, shininess }
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn view(&self) -> Vec3 {
        self.view
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_point(&mut self, point: Vec3) {
        self.point = point;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn set_view(&mut self, view: Vec3) {
        self.view = view;
    }

    pub fn set_albedo(&mut self, albedo: Color) {
        self.albedo = albedo;
    }

    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = shininess;
    }
}
