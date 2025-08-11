use super::{mat4::Mat4, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec3,
    pub pitch: f32, // rotation around X axis
    pub yaw: f32,   // rotation around Y axis,
}

impl Camera {
    pub fn new(position: Vec3, pitch: f32, yaw: f32) -> Self {
        Self {
            position,
            pitch,
            yaw,
        }
    }

    pub fn direction(&self) -> Vec3 {
        Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize()
    }

    pub fn view_matrix(&self) -> Mat4 {
        let dir = self.direction();
        Mat4::look_at(self.position, self.position + dir, Vec3::new(0.0, 1.0, 0.0))
    }
}
