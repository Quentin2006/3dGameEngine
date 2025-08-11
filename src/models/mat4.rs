use std::ops::Mul;

use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    /// Creates an identity matrix
    fn identity() -> Mat4 {
        Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a translation matrix
    fn translate(tx: f32, ty: f32, tz: f32) -> Mat4 {
        let mut mat = Mat4::identity();
        mat.data[0][3] = tx;
        mat.data[1][3] = ty;
        mat.data[2][3] = tz;
        mat
    }

    /// Multiplies two Mat4 matrices (self * other)
    pub fn mul(&self, other: &Mat4) -> Mat4 {
        let mut result = Mat4 {
            data: [[0.0; 4]; 4],
        };

        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    result.data[row][col] += self.data[row][i] * other.data[i][col];
                }
            }
        }

        result
    }

    /// Builds a simple LookAt matrix
    /// `eye` = camera position
    /// `target` = point camera is looking at
    /// `up` = up direction vector
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        let f = (target - eye).normalize();
        let s = f.cross(&up).normalize();
        let u = s.cross(&f);

        let mut mat = Mat4::identity();

        mat.data[0][0] = s.x;
        mat.data[0][1] = s.y;
        mat.data[0][2] = s.z;
        mat.data[0][3] = -s.dot(&eye);

        mat.data[1][0] = u.x;
        mat.data[1][1] = u.y;
        mat.data[1][2] = u.z;
        mat.data[1][3] = -u.dot(&eye);

        mat.data[2][0] = -f.x;
        mat.data[2][1] = -f.y;
        mat.data[2][2] = -f.z;
        mat.data[2][3] = f.dot(&eye);

        mat.data[3][0] = 0.0;
        mat.data[3][1] = 0.0;
        mat.data[3][2] = 0.0;
        mat.data[3][3] = 1.0;

        mat
    }
}

impl std::ops::Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        let x = self.data[0][0] * v.x
            + self.data[0][1] * v.y
            + self.data[0][2] * v.z
            + self.data[0][3] * 1.0;
        let y = self.data[1][0] * v.x
            + self.data[1][1] * v.y
            + self.data[1][2] * v.z
            + self.data[1][3] * 1.0;
        let z = self.data[2][0] * v.x
            + self.data[2][1] * v.y
            + self.data[2][2] * v.z
            + self.data[2][3] * 1.0;
        let w = self.data[3][0] * v.x
            + self.data[3][1] * v.y
            + self.data[3][2] * v.z
            + self.data[3][3] * 1.0;

        if w != 0.0 {
            Vec3 {
                x: x / w,
                y: y / w,
                z: z / w,
            }
        } else {
            Vec3 { x, y, z }
        }
    }
}
