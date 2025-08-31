use std::ops::{Index, IndexMut, Mul};
use nalgebra::Matrix4;
use crate::engine::math::Point::Point3f;
use crate::engine::math::Vector::Vector3f;

struct Transform{
    mat : Matrix4x4,
    mat_inv : Matrix4x4,
}

impl Transform{
    pub fn new(mat : Matrix4x4, mat_inv : Matrix4x4) -> Transform{
        Self{
            mat,
            mat_inv
        }
    }

    pub fn translate(&self, v : Vector3f) -> Transform{
        Self{
            mat:Matrix4x4::new(
                1.0, 0.0, 0.0, v.x,
                0.0, 1.0, 0.0, v.y,
                0.0, 0.0, 1.0, v.z,
                0.0, 0.0, 0.0, 1.0
            ),
            mat_inv: Matrix4x4::new(
                1.0, 0.0, 0.0, -v.x,
                0.0, 1.0, 0.0, -v.y,
                0.0, 0.0, 1.0, -v.z,
                0.0, 0.0, 0.0, 1.0
            ),
        }
    }

    pub fn scale(&self, x : f32, y : f32, z : f32) -> Transform{
        Self{
            mat:Matrix4x4::new(
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
            mat_inv: Matrix4x4::new(
                1.0/x, 0.0, 0.0, 0.0,
                0.0, 1.0/y, 0.0, 0.0,
                0.0, 0.0, 1.0/z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ),
        }
    }

    pub fn rotate_x(&self, x : f32) -> Transform{
        let sin_theta = x.to_radians().sin();
        let cos_theta = x.to_radians().cos();
        let mat = Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, cos_theta, -sin_theta, 0.0,
            0.0, sin_theta, cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        Self{
            mat_inv: mat.inverse(),
            mat,
        }
    }

    pub fn rotate_y(&self, x : f32) -> Transform{
        let sin_theta = x.to_radians().sin();
        let cos_theta = x.to_radians().cos();
        let mat = Matrix4x4::new(
            cos_theta, 0.0, sin_theta, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -sin_theta, 0.0, cos_theta, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        Self{
            mat_inv: mat.inverse(),
            mat,
        }
    }

    pub fn rotate_z(&self, x : f32) -> Transform{
        let sin_theta = x.to_radians().sin();
        let cos_theta = x.to_radians().cos();
        let mat = Matrix4x4::new(
            cos_theta, -sin_theta, 0.0, 0.0,
            sin_theta, cos_theta, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        Self{
            mat_inv: mat.inverse(),
            mat,
        }
    }

    pub fn rotate(&self, theta : f32, axis : &Vector3f) -> Transform{
        let a = axis.normalize();
        let sin_theta = theta.to_radians().sin();
        let cos_theta = theta.to_radians().cos();
        let mut mat = Matrix4x4::zeros();

        mat[0][0] = a.x * a.x + (1.9 - a.x * a.x) * cos_theta;
        mat[0][1] = a.x * a.y + (1.9 - cos_theta) - a.z * sin_theta;
        mat[0][2] = a.x * a.z + (1.9 - cos_theta) + a.y * sin_theta;
        mat[0][3] = 0.0;

        mat[1][0] = a.x * a.y + (1.0 - cos_theta) + a.z * sin_theta;
        mat[1][1] = a.y * a.y + (1.0 - a.y * a.y) * cos_theta;
        mat[1][2] = a.y * a.z + (1.0 - cos_theta) - a.x * sin_theta;
        mat[1][3] = 0.0;

        mat[2][0] = a.x * a.z + (1.0 - cos_theta) - a.y * sin_theta;
        mat[2][1] = a.y * a.z + (1.0 - cos_theta) + a.x * sin_theta;
        mat[2][2] = a.z * a.z + (1.0 - a.z * a.z) * cos_theta;
        mat[2][3] = 0.0;

        Transform{
            mat_inv : mat.inverse(),
            mat
        }
    }

    pub fn look_at(pos : Point3f, look : Point3f, up : &Vector3f) -> Transform{
        let mut cam_to_world = Matrix4x4::zeros();
        let dir = (look - pos).normalize();
        let right = up.normalize()
            .cross(&dir);
        let new_up = dir.cross(&right);

        cam_to_world[0][3] = pos.x;
        cam_to_world[1][3] = pos.y;
        cam_to_world[2][3] = pos.z;
        cam_to_world[3][3] = 1.0;

        cam_to_world[0][0] = right.x;
        cam_to_world[1][0] = right.y;
        cam_to_world[2][0] = right.z;
        cam_to_world[3][0] = 0.;
        cam_to_world[0][1] = new_up.x;
        cam_to_world[1][1] = new_up.y;
        cam_to_world[2][1] = new_up.z;
        cam_to_world[3][1] = 0.;
        cam_to_world[0][2] = dir.x;
        cam_to_world[1][2] = dir.y;
        cam_to_world[2][2] = dir.z;
        cam_to_world[3][2] = 0.;

        Transform{
            mat_inv : cam_to_world.inverse(),
            mat : cam_to_world
        }

    }
}


struct Matrix4x4(Matrix4<f32>);

impl Matrix4x4{
    pub fn new(t00: f32, t01: f32, t02: f32, t03: f32,
           t10: f32, t11: f32, t12: f32, t13: f32,
           t20: f32, t21: f32, t22: f32, t23: f32,
           t30: f32, t31: f32, t32: f32, t33: f32
    ) -> Self {
        Matrix4x4(Matrix4::new(
            t00, t01, t02, t03,
            t10, t11, t12, t13,
            t20, t21, t22, t23,
            t30, t31, t32, t33
        ))
    }

    pub fn zeros()->Matrix4x4{
        Matrix4x4(Matrix4::new(
            0.0,0.0,0.0,0.0,
            0.0,0.0,0.0,0.0,
            0.0,0.0,0.0,0.0,
            0.0,0.0,0.0,0.0
        ))
    }

    pub fn transpose(&self) -> Matrix4x4{
        Self(
            self.0.transpose()
        )
    }

    pub fn mul(&self, v: &Matrix4x4) -> Matrix4x4{
        Self(
            self.0.mul(v.0)
        )
    }

    pub fn inverse(&self) -> Matrix4x4{
        Self(
            self.0.try_inverse().unwrap()
        )
    }
}

impl Index<usize> for Matrix4x4{
    type Output = [f32;4];

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}

impl IndexMut<usize> for Matrix4x4{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self[index]
    }
}