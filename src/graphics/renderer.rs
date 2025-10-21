use minifb::Window;

use crate::{
    graphics::rasterizer,
    models::{triangle::Triangle, vec3::Vec3},
    window::window::init_window,
};

#[derive(Debug)]
pub struct Renderer {
    width: usize,
    height: usize,
    pub window: Window,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            window: init_window(width, height),
        }
    }
    /// Draws the raster buffer to the window and clears buffers for the next frame
    pub fn draw_raster(&mut self, raster: &mut [u32], z_buffer: &mut [f32]) {
        self.window
            .update_with_buffer(raster, self.width, self.height)
            .unwrap();
        raster.fill(0);
        z_buffer.fill(f32::NEG_INFINITY);
    }

    /// Renders a single triangle with perspective and viewport transforms
    pub fn render_tri(&self, tri: &Triangle, z_buffer: &mut [f32], raster: &mut [u32]) {
        // Early cull triangles behind the camera
        if tri.v0.z >= 0.0 || tri.v1.z >= 0.0 || tri.v2.z >= 0.0 {
            return;
        }

        // Perspective divide
        let tri_proj = Triangle::new(
            Vec3::new(tri.v0.x / -tri.v0.z, tri.v0.y / -tri.v0.z, tri.v0.z),
            Vec3::new(tri.v1.x / -tri.v1.z, tri.v1.y / -tri.v1.z, tri.v1.z),
            Vec3::new(tri.v2.x / -tri.v2.z, tri.v2.y / -tri.v2.z, tri.v2.z),
        );

        // Transform to screen coordinates using Renderer dimensions
        let tri_screen = tri_proj.viewport_transform(self.width as f32, self.height as f32);

        // Back-face culling
        if !tri_screen.is_front_facing() {
            return;
        }

        // Rasterize the triangle
        rasterizer::rasterizer(raster, &tri_screen, z_buffer, self.width, self.height);
    }
}
