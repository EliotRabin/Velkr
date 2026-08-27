use crate::pipeline::geometry::vec3::Vec3;

pub struct Model {
    vertices: Vec<Vec3>,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>) -> Self {
        Model { vertices }
    }

}