use crate::models::vec3::Vec3;
use std::ops::{Add, Sub};

use super::{mat4::Mat4, vec2::Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self { v0, v1, v2 }
    }

    pub fn random(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Self {
        Triangle::new(
            Vec3::random(min_x, min_y, min_z, max_x, max_y, max_z),
            Vec3::random(min_x, min_y, min_z, max_x, max_y, max_z),
            Vec3::random(min_x, min_y, min_z, max_x, max_y, max_z),
        )
    }

    pub fn transform(self, mat: Mat4) -> Triangle {
        Triangle::new(mat * self.v0, mat * self.v1, mat * self.v2)
    }

    // assumes that the triangle is [-1, 1]
    pub fn viewport_transform(&self, width: f32, height: f32) -> Triangle {
        let convert = |v: &Vec3| {
            Vec3::new(
                ((v.x + 1.0) * 0.5) * width,
                ((1.0 - v.y) * 0.5) * height,
                v.z,
            )
        };
        Triangle::new(convert(&self.v0), convert(&self.v1), convert(&self.v2))
    }
}

impl Sub for Triangle {
    type Output = Triangle;

    fn sub(self, other: Triangle) -> Triangle {
        Triangle {
            v1: self.v1 - other.v1,
            v0: self.v0 - other.v0,
            v2: self.v2 - other.v2,
        }
    }
}

impl Add for Triangle {
    type Output = Triangle;

    fn add(self, other: Triangle) -> Triangle {
        Triangle {
            v0: self.v0 + other.v0,
            v1: self.v1 + other.v1,
            v2: self.v2 + other.v2,
        }
    }
}
