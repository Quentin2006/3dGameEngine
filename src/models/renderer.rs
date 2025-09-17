use minifb::Window;

use crate::{HEIGHT, WIDTH, models::mat4::Mat4, rasterizer, window};

use super::{camera::Camera, triangle::Triangle, vec3::Vec3};

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
            window: window::init_window(width, height),
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
    view: Mat4,
) {
    let tri_view = tri.transform(view);

    // triangles must be in front of camera (your convention)
    if tri_view.v0.z >= 0.0 || tri_view.v1.z >= 0.0 || tri_view.v2.z >= 0.0 {
        return;
    }

    let tri_proj = Triangle::new(
        Vec3::new(
            tri_view.v0.x / -tri_view.v0.z,
            tri_view.v0.y / -tri_view.v0.z,
            tri_view.v0.z,
        ),
        Vec3::new(
            tri_view.v1.x / -tri_view.v1.z,
            tri_view.v1.y / -tri_view.v1.z,
            tri_view.v1.z,
        ),
        Vec3::new(
            tri_view.v2.x / -tri_view.v2.z,
            tri_view.v2.y / -tri_view.v2.z,
            tri_view.v2.z,
        ),
    );

    let tri_screen = tri_proj.viewport_transform(WIDTH as f32, HEIGHT as f32);

    if !tri_screen.is_front_facing() {
        return;
    }

    rasterizer::rasterizer(raster_chunk, tri_screen, z_chunk, start_index, end_index);
}

// pub fn render(&mut self, triangles: &mut Vec<Triangle>, raster: &mut [u32], camera: &Camera) {
//     let view = camera.view_matrix();
//     // draw triangles
//     for tri in triangles {
//         // apply camera transformation
//
//         let tri_view = tri.transform(view);
//
//         // skip triangles behind camera
//         if tri_view.v0.z >= 0.0 || tri_view.v1.z >= 0.0 || tri_view.v2.z >= 0.0 {
//             continue;
//         }
//
//         // apply basic divide projection, we do -z so our prespective isnt flipped
//         // NOTE: move to matrix multiplication
//         let tri_proj = Triangle::new(
//             Vec3::new(
//                 tri_view.v0.x / -tri_view.v0.z,
//                 tri_view.v0.y / -tri_view.v0.z,
//                 tri_view.v0.z,
//             ),
//             Vec3::new(
//                 tri_view.v1.x / -tri_view.v1.z,
//                 tri_view.v1.y / -tri_view.v1.z,
//                 tri_view.v1.z,
//             ),
//             Vec3::new(
//                 tri_view.v2.x / -tri_view.v2.z,
//                 tri_view.v2.y / -tri_view.v2.z,
//                 tri_view.v2.z,
//             ),
//         );
//
//         // apply viewport transformation, x and will will correspond to the screen, and z value
//         // will be used for z-buffering
//         let tri_screen = tri_proj.viewport_transform(self.width as f32, self.height as f32);
//
//         if !tri_screen.is_front_facing() {
//             continue;
//         }
//
//         // rasterization
//         rasterizer::rasterizer(raster, tri_screen, &mut self.z_buffer);
//     }
//
//     self.window
//         .update_with_buffer(raster, self.width, self.height)
//         .unwrap();
//
//     self.clear(raster);
// }
