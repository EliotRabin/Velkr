use crate::pipeline::math::vec3::Vec3;
use std::ops::Mul;
use crate::pipeline::geometry::camera::Camera;
use crate::pipeline::geometry::model::Model;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    data: [[f64; 4]; 4],
}

pub enum MatrixType {
    ModelMatrix(Model),
    ViewMatrix(Camera),
    ProjectionMatrix(Camera),
}

impl Mat4 {
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        Mat4 { data }
    }

    pub fn data(&self) -> &[[f64; 4]; 4] {
        &self.data
    }

    pub fn set_data(&mut self, data: [[f64; 4]; 4]) {
        self.data = data;
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Mat4::new(result)
    }
}

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        let x = self.data[0][0] * vec.x() + self.data[0][1] * vec.y() + self.data[0][2] * vec.z() + self.data[0][3];
        let y = self.data[1][0] * vec.x() + self.data[1][1] * vec.y() + self.data[1][2] * vec.z() + self.data[1][3];
        let z = self.data[2][0] * vec.x() + self.data[2][1] * vec.y() + self.data[2][2] * vec.z() + self.data[2][3];
        Vec3::new(x, y, z)
    }
}