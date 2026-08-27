use crate::pipeline::math::vec3::Vec3;

pub enum ProjectionType {
    Perspective(f64, f64, f64, f64),
    Orthographic(f64, f64, f64, f64),
}
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
}