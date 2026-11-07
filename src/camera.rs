use crate::matrix;

pub struct Camera {
    pub position: matrix::Vector,
    // The direction the camera is facing
    pub direction: matrix::Vector,

    pub aspect_ratio: f32,
}

impl Camera {
    pub fn move_in_view_space(&mut self, x: f32, y: f32, z: f32) {
        let unit_dir = matrix::Vector::from_direction(
            matrix::Vector { x: 0.0, y: self.direction.y, z: self.direction.z }
        );

        let x_axis = unit_dir.cross(matrix::VECTOR_UP);
        self.position += x_axis * x;

        self.position += matrix::VECTOR_UP * y;

        self.position += unit_dir * z;
    }

    pub fn gen_world_to_clip(&self) -> matrix::Matrix {
        matrix::Matrix::gen_translation(-self.position.x, -self.position.y, -self.position.z)
            * matrix::Matrix::gen_rotation(0.0, 1.0, 0.0, -self.direction.y)
            * matrix::Matrix::gen_rotation(1.0, 0.0, 0.0, -self.direction.x)
            * matrix::Matrix::gen_rotation(0.0, 0.0, 1.0, -self.direction.z)

            * matrix::Matrix::gen_perspective(3.141 / 4.0, self.aspect_ratio, 0.1, 100.0)
    }
}

