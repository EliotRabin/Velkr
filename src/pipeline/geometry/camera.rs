use crate::pipeline::math::vec3::Vec3;

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

    
}