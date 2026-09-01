use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::viewport::Viewport;
use std::ops::Mul;
use crate::pipeline::geometry::camera::Camera;
use crate::pipeline::geometry::model::Model;
use crate::pipeline::geometry::camera::ProjectionType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    data: [[f64; 4]; 4],
}

pub enum MatrixType<'a> {
    ModelMatrix(&'a Model<'a>),
    ViewMatrix(&'a Camera),
    ProjectionMatrix(&'a Camera),
    ViewportMatrix(&'a Viewport),
    MVPMatrix(&'a Model<'a>, &'a Camera, &'a Viewport),
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

    pub fn from_matrix_type(matrix_type: MatrixType<'_>) -> Self {
        match matrix_type {
            MatrixType::ModelMatrix(model) => {
                let translation = model.location();
                let rotation = model.rotation();
                let scale = model.scale();

                let translation_matrix = Mat4::new([
                    [1.0, 0.0, 0.0, translation.x()],
                    [0.0, 1.0, 0.0, translation.y()],
                    [0.0, 0.0, 1.0, translation.z()],
                    [0.0, 0.0, 0.0, 1.0],
                ]);

                let rotation_x_matrix = Mat4::new([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, rotation.x().cos(), -rotation.x().sin(), 0.0],
                    [0.0, rotation.x().sin(), rotation.x().cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]);

                let rotation_y_matrix = Mat4::new([
                    [rotation.y().cos(), 0.0, rotation.y().sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-rotation.y().sin(), 0.0, rotation.y().cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]);

                let rotation_z_matrix = Mat4::new([
                    [rotation.z().cos(), -rotation.z().sin(), 0.0, 0.0],
                    [rotation.z().sin(), rotation.z().cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]);

                let scale_matrix = Mat4::new([
                    [scale.x(), 0.0, 0.0, 0.0],
                    [0.0, scale.y(), 0.0, 0.0],
                    [0.0, 0.0, scale.z(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]);

                translation_matrix * rotation_x_matrix * rotation_y_matrix * rotation_z_matrix * scale_matrix
            }
            MatrixType::ViewMatrix(camera) => {
                let position = camera.position();
                let forward = camera.forward();
                let up = camera.up();
                let right = camera.right();

                let view_matrix = Mat4::new([
                    [right.x(), right.y(), right.z(), -right.dot(position)],
                    [up.x(), up.y(), up.z(), -up.dot(position)],
                    [-forward.x(), -forward.y(), -forward.z(), forward.dot(position)],
                    [0.0, 0.0, 0.0, 1.0],
                ]);

                view_matrix
            }
            MatrixType::ProjectionMatrix(camera) => {
                match camera.projection_type() {
                    ProjectionType::Perspective(fov, aspect_ratio, near, far) => {
                        let f = 1.0 / (fov.to_radians() / 2.0).tan();
                        let nf = 1.0 / (near - far);
                        Mat4::new([
                            [f / aspect_ratio, 0.0, 0.0, 0.0],
                            [0.0, f, 0.0, 0.0],
                            [0.0, 0.0, (far + near) * nf, 2.0 * far * near * nf],
                            [0.0, 0.0, -1.0, 0.0],
                        ])
                    }
                    ProjectionType::Orthographic(left, right, bottom, top) => {
                        let rl = right - left;
                        let tb = top - bottom;
                        Mat4::new([
                            [2.0 / rl, 0.0, 0.0, -(right + left) / rl],
                            [0.0, 2.0 / tb, 0.0, -(top + bottom) / tb],
                            [0.0, 0.0, -1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0],
                        ])
                    }
                }
            }
            MatrixType::ViewportMatrix(viewport) => {
                let width = viewport.width() as f64;
                let height = viewport.height() as f64;
                Mat4::new([
                    [width / 2.0, 0.0, 0.0, (width - 1.0) / 2.0],
                    [0.0, -height / 2.0, 0.0, (height - 1.0) / 2.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ])
            }
            MatrixType::MVPMatrix(model, camera, viewport) => {
                let model_matrix = Mat4::from_matrix_type(MatrixType::ModelMatrix(model));
                let view_matrix = Mat4::from_matrix_type(MatrixType::ViewMatrix(camera));
                let projection_matrix = Mat4::from_matrix_type(MatrixType::ProjectionMatrix(camera));
                let viewport_matrix = Mat4::from_matrix_type(MatrixType::ViewportMatrix(viewport));

                viewport_matrix * projection_matrix * view_matrix * model_matrix
            }
        }
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
        let w = self.data[3][0] * vec.x() + self.data[3][1] * vec.y() + self.data[3][2] * vec.z() + self.data[3][3];

        if w != 0.0 && w != 1.0 {
            Vec3::new(x / w, y / w, z / w)
        } else {
            Vec3::new(x, y, z)
        }
    }
}