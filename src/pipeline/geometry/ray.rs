use crate::pipeline::math::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction: direction.normalize() }
    }

    pub fn between(origin: Vec3, target: Vec3) -> Self {
        Ray::new(origin, target - origin)
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + self.direction * distance
    }

    pub fn shifted(&self, offset: Vec3) -> Ray {
        Ray { origin: self.origin + offset, direction: self.direction }
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction.normalize();
    }
}