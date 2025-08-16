use minifb::Key;

use super::{mat4::Mat4, vec2::Vec2, vec3::Vec3};

const MOVEMENT_SPEED: f32 = 0.05;

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

    pub fn get_movement(&mut self, window: &minifb::Window, prev_mouse_pos: Option<(f32, f32)>) {
        let forward = self.direction();
        let mut movement = Vec3::new(0.0, 0.0, 0.0);
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = forward.cross(&world_up).normalize();

        let mut look = Vec2::new(0.0, 0.0);

        // Movement keys
        if window.is_key_down(Key::W) {
            movement = movement + forward;
        }
        if window.is_key_down(Key::A) {
            movement = movement - right;
        }
        if window.is_key_down(Key::S) {
            movement = movement - forward;
        }
        if window.is_key_down(Key::D) {
            movement = movement + right;
        }
        if window.is_key_down(Key::Space) {
            movement = movement + world_up;
        }
        if window.is_key_down(Key::LeftShift) {
            movement = movement - world_up;
        }

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if let Some((prev_x, prev_y)) = prev_mouse_pos {
                let delta_x = x - prev_x;
                let delta_y = y - prev_y;
                look.x += delta_x;
                look.y -= delta_y;
            }
        }
        // Apply movement
        if movement.length() > 0.0 {
            movement = movement.normalize() * MOVEMENT_SPEED;
            self.position = self.position + movement;
        }

        // Apply look
        self.yaw += look.x;
        self.pitch += look.y;
        self.pitch = self.pitch.clamp(-89.9, 89.9);
    }
}
