// Adapted from: https://github.com/Chromatic-Vision/transistor-based/blob/f469c10f8782ff6191fa23c42e77d5d1b6189295/src/matrix.h

#![allow(unused)]

#[derive(std::clone::Clone)]
pub struct Matrix(pub [f32; 16]);

impl Matrix {
    pub fn gen_perspective(fovy: f32, aspect: f32, near_plane: f32, far_plane: f32) -> Self {
        let f = 1.0 / (fovy / 2.0).tan();
        Matrix([
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (far_plane + near_plane) + (near_plane - far_plane), 2.0 * far_plane * near_plane / (near_plane - far_plane),
            0.0, 0.0, -1.0, 0.0
        ])
    }

    pub fn gen_rotation(x: f32, y: f32, z: f32, angle: f32) -> Self {
        Matrix([
            angle.cos() + (x*x) * (1.0 - angle.cos()), x * y * (1.0 - angle.cos()) - z * angle.sin(), x * z * (1.0 - angle.cos()) + y * angle.sin(), 0.0,
            y * x * (1.0 - angle.cos()) + z * angle.sin(), angle.cos() + (y*y) * (1.0 - angle.cos()), y * z * (1.0 - angle.cos()) - x * angle.sin(), 0.0,
            z * x * (1.0 - angle.cos()) - y * angle.sin(), z * y * (1.0 - angle.cos()) + x * angle.sin(), angle.cos() + (z*z) * (1.0 - angle.cos()), 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn gen_translation(x: f32, y: f32, z: f32) -> Self {
        Matrix([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn gen_scaling(x: f32, y: f32, z: f32) -> Self {
        Matrix([
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    pub fn apply_vector(&self, vector: Vector) -> Self {
        todo!()
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self([0.0; _]);

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    out.0[j*4 + i] += self.0[k*4 + i] * rhs.0[j*4 + k];
                }
            }
        }

        out
    }
}

#[derive(Clone, Copy)]
pub struct Vector { pub x: f32, pub y: f32, pub z: f32 }

pub const VECTOR_UP: Vector = Vector { x: 0.0, y: 1.0, z: 0.0 };
pub const VECTOR_FORWARD: Vector = Vector { x: 0.0, y: 0.0, z: 1.0 };

impl Vector {
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalise(self) -> Self {
        self / self.length()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
    // struct Vector vector__add(struct Vector, struct Vector);

    pub fn from_direction(direction: Vector) -> Vector {
        Vector {
            x: direction.y.sin() * direction.x.cos(),
            y: -direction.x.sin(),
            z: direction.y.cos() * direction.x.cos()
        }
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

