use crate::pipeline::geometry::fragment::{AttributKind, Fragment};
use crate::pipeline::geometry::hit::Hit;
use crate::pipeline::geometry::ray::Ray;
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    triangle: Triangle<'a>,
    hit: Hit,
    ray: Ray,
}

impl<'a> Intersection<'a> {
    pub fn new(triangle: Triangle<'a>, hit: Hit, ray: Ray) -> Self {
        Intersection { triangle, hit, ray }
    }

    pub fn triangle(&self) -> &Triangle<'a> {
        &self.triangle
    }

    pub fn hit(&self) -> &Hit {
        &self.hit
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn distance(&self) -> f64 {
        self.hit.distance()
    }

    pub fn point(&self) -> Vec3 {
        self.ray.at(self.hit.distance())
    }

    pub fn normal(&self) -> Vec3 {
        let normal = self.triangle.normal();

        if normal.dot(&self.ray.direction()) > 0.0 {
            -normal
        } else {
            normal
        }
    }

    pub fn fragment(&self) -> Fragment {
        let (alpha, beta, gamma) = self.hit.barycentric();
        self.triangle.interpolate(alpha, beta, gamma)
    }

    pub fn albedo(&self) -> Color {
        self.fragment()
            .color(AttributKind::Color)
            .unwrap_or(Color::new(255, 255, 255))
    }

    pub fn reflectivity(&self) -> f64 {
        self.fragment()
            .scalar(AttributKind::Reflectivity)
            .unwrap_or(0.0)
    }
}
