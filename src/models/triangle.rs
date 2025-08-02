use crate::models::vec3::Vec3;
use std::ops::{Add, Sub};

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

    pub fn add(&mut self, v0: Vec3, v1: Vec3, v2: Vec3) {
        self.v0 = self.v0.add(&v0);
        self.v1 = self.v1.add(&v1);
        self.v2 = self.v2.add(&v2);
    }
    pub fn subtract(&mut self, v0: Vec3, v1: Vec3, v2: Vec3) {
        self.v0 = self.v0.subtract(&v0);
        self.v1 = self.v1.subtract(&v1);
        self.v2 = self.v2.subtract(&v2);
    }

    pub fn random(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Self {
        Triangle::new(
            Vec3::random(min_x, min_y, min_z, max_x, max_y, max_z),
            Vec3::random(min_x, min_y, min_z, max_x, max_y, max_z),
            Vec3::random(min_x, min_y, min_z, max_x, max_y, max_z),
        )
    }
}

impl Sub for Triangle {
    type Output = Triangle;

    fn sub(self, other: Triangle) -> Triangle {
        Triangle {
            v0: self.v0.subtract(&other.v0),
            v1: self.v1.subtract(&other.v1),
            v2: self.v2.subtract(&other.v2),
        }
    }
}

impl Add for Triangle {
    type Output = Triangle;

    fn add(self, other: Triangle) -> Triangle {
        Triangle {
            v0: self.v0.add(&other.v0),
            v1: self.v1.add(&other.v1),
            v2: self.v2.add(&other.v2),
        }
    }
}
