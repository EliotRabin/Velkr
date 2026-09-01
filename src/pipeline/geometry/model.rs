use crate::pipeline::geometry::fragment::Fragment;
use crate::pipeline::geometry::mesh::Mesh;
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::math::mat4::{Mat4, MatrixType};
use crate::pipeline::math::vec3::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub struct Model<'a> {
    mesh: &'a Mesh,
    triangles: Vec<Triangle<'a>>,
    location: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl<'a> Model<'a> {
    pub fn new(mesh: &'a Mesh, location: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Model { mesh, triangles: mesh.triangles(), location, rotation, scale }
    }

    pub fn from_mesh(mesh: &'a Mesh) -> Self {
        Model::new(
            mesh,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        )
    }

    pub fn mesh(&self) -> &'a Mesh {
        self.mesh
    }

    pub fn triangles(&self) -> &Vec<Triangle<'a>> {
        &self.triangles
    }

    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_matrix_type(MatrixType::ModelMatrix(self))
    }

    pub fn world_vertices(&self) -> Vec<Fragment> {
        self.mesh.transformed_vertices(&self.model_matrix())
    }

    pub fn world_triangles<'b>(&self, vertices: &'b [Fragment]) -> Vec<Triangle<'b>> {
        self.mesh.triangles_from(vertices)
    }

    pub fn location(&self) -> &Vec3 {
        &self.location
    }

    pub fn rotation(&self) -> &Vec3 {
        &self.rotation
    }

    pub fn scale(&self) -> &Vec3 {
        &self.scale
    }

    pub fn set_location(&mut self, location: Vec3) {
        self.location = location;
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }
}
