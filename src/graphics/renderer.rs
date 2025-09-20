use minifb::Window;

use crate::{
    HEIGHT, WIDTH,
    graphics::rasterizer,
    models::{mat4::Mat4, triangle::Triangle, vec3::Vec3},
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

    /// will draw raster and stage it for next render
    ///
    /// * `raster`: raster to be rendered
    pub fn draw_raster(&mut self, raster: &mut [u32], z_buffer: &mut [f32]) {
        self.window
            .update_with_buffer(raster, self.width, self.height)
            .unwrap();
        raster.fill(0);
        z_buffer.fill(f32::NEG_INFINITY);
    }

    // self.window
    //     .update_with_buffer(raster, self.width, self.height)
    //     .unwrap();
    //
    // self.clear(raster);
}

pub fn render_tri(
    tri: &Triangle,
    z_chunk: &mut [f32],
    raster_chunk: &mut [u32],
    start_index: usize,
    end_index: usize,
) {
    // triangles must be in front of camera (your convention)
    if tri.v0.z >= 0.0 || tri.v1.z >= 0.0 || tri.v2.z >= 0.0 {
        return;
    }

    let tri_proj = Triangle::new(
        Vec3::new(tri.v0.x / -tri.v0.z, tri.v0.y / -tri.v0.z, tri.v0.z),
        Vec3::new(tri.v1.x / -tri.v1.z, tri.v1.y / -tri.v1.z, tri.v1.z),
        Vec3::new(tri.v2.x / -tri.v2.z, tri.v2.y / -tri.v2.z, tri.v2.z),
    );

    let tri_screen = tri_proj.viewport_transform(WIDTH as f32, HEIGHT as f32);

    if !tri_screen.is_front_facing() {
        return;
    }
    rasterizer::rasterizer(raster_chunk, &tri_screen, z_chunk, start_index, end_index);
}
