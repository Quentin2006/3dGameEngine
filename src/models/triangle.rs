use crate::models::vec3::Vec3;

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
}
