use crate::pipeline::geometry::vec3::Vec3
;
use crate::pipeline::geometry::mat4::Mat4
;

struct Camera {
    position: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    fov: f64,
    aspect_ratio: f64,
}