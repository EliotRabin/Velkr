use crate::pipeline::geometry::ray::Ray;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::viewport::Viewport;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectionType {
    Perspective(f64, f64, f64, f64),
    Orthographic(f64, f64, f64, f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    position: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
    fov: f64,
    aspect_ratio: f64,
    projection_type: ProjectionType,
}

impl Camera {
    pub fn new(position: Vec3, forward: Vec3, up: Vec3, fov: f64, aspect_ratio: f64, projection_type: ProjectionType) -> Self {
        let right: Vec3 = forward.cross(&up).normalize();
        Camera {
            position,
            forward,
            up,
            right,
            fov,
            aspect_ratio,
            projection_type,
        }
    }

    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    pub fn forward(&self) -> &Vec3 {
        &self.forward
    }

    pub fn up(&self) -> &Vec3 {
        &self.up
    }

    pub fn right(&self) -> &Vec3 {
        &self.right
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn projection_type(&self) -> &ProjectionType {
        &self.projection_type
    }

    pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let forward = self.forward.normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward);

        (forward, right, up)
    }

    pub fn ray(&self, x: usize, y: usize, viewport: &Viewport) -> Ray {
        let (forward, right, up) = self.basis();
        let tangent = (self.fov / 2.0).to_radians().tan();

        let sx = (2.0 * (x as f64 + 0.5) / viewport.width() as f64 - 1.0) * self.aspect_ratio * tangent;
        let sy = (1.0 - 2.0 * (y as f64 + 0.5) / viewport.height() as f64) * tangent;

        Ray::new(self.position, forward + right * sx + up * sy)
    }

    
}