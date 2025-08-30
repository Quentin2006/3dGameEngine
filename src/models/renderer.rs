use crate::rasterizer;

use super::{camera::Camera, triangle::Triangle, vec3::Vec3};

#[derive(Debug)]
pub struct Renderer {
    width: usize,
    height: usize,
    z_buffer: Vec<f32>,
    pub raster: Vec<u32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            z_buffer: vec![f32::NEG_INFINITY; width * height],
            raster: vec![0; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.raster.fill(0);
        self.z_buffer.fill(f32::NEG_INFINITY);
    }

    pub fn render(&mut self, triangles: &mut Vec<Triangle>, camera: &Camera) {
        self.clear();
        let view = camera.view_matrix();
        // draw triangles
        for tri in triangles {
            // apply camera transformation
            let tri_view = tri.transform(view);

            // skip triangles behind camera
            if tri_view.v0.z >= 0.0 || tri_view.v1.z >= 0.0 || tri_view.v2.z >= 0.0 {
                continue;
            }

            // apply basic divide projection, we do -z so our prespective isnt flipped
            // NOTE: move to matrix multiplication
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

            // apply viewport transformation, x and will will correspond to the screen, and z value
            // will be used for z-buffering
            let tri_screen = tri_proj.viewport_transform(self.width as f32, self.height as f32);

            if !tri_screen.is_front_facing() {
                continue;
            }

            // rasterization
            rasterizer::rasterizer(&mut self.raster, tri_screen, &mut self.z_buffer);
        }
    }
}
