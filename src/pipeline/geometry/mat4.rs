pub struct Mat4 {
    data: [[f64; 4]; 4],
}

pub enum MatrixType {
    Identity,
    Translation(f64, f64, f64),
    Scaling(f64, f64, f64),
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
    Perspective(f64, f64, f64, f64),
    Orthographic(f64, f64, f64, f64),
    Viewport(f64, f64, f64, f64),
    View(f64, f64, f64, f64, f64, f64),
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

    /*
    pub fn from_type(matrix_type: MatrixType) -> Self {
        match matrix_type {
            MatrixType::Identity => Mat4::identity(),
            MatrixType::Translation(x, y, z) => Mat4::translation(x, y, z),
            MatrixType::Scaling(x, y, z) => Mat4::scaling(x, y, z),
            MatrixType::RotationX(angle) => Mat4::rotation_x(angle),
            MatrixType::RotationY(angle) => Mat4::rotation_y(angle),
            MatrixType::RotationZ(angle) => Mat4::rotation_z(angle),
            MatrixType::Perspective(fov, aspect, near, far) => Mat4::perspective(fov, aspect, near, far),
            MatrixType::Orthographic(left, right, bottom, top) => Mat4::orthographic(left, right, bottom, top),
            MatrixType::Viewport(x, y, width, height) => Mat4::viewport(x, y, width, height),
            MatrixType::View(eye_x, eye_y, eye_z, center_x, center_y, center_z) => {
                Mat4::view(eye_x, eye_y, eye_z, center_x, center_y, center_z)
            }
        }
    }*/
}