use std::thread;

use super::{camera::Camera, triangle::Triangle, vec3::Vec3};
use crate::rasterizer;
use rayon::{ThreadPoolBuilder, prelude::*};

const THREADS: usize = 12;

#[derive(Debug)]
pub struct Renderer {
    width: usize,
    height: usize,
    z_buffer: Vec<f32>,
    pub raster: Vec<u32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        ThreadPoolBuilder::new()
            .num_threads(THREADS)
            .build_global()
            .unwrap();
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
        let partial_results: Vec<(Vec<u32>, Vec<f32>)> = triangles
            .par_iter()
            .filter_map(|tri| {
                // an uint32 showing what thread # we are, starting at 0
                let cur_thread = std::thread::current().id();

                // apply camera transform
                let tri_view = tri.transform(view);

                if tri_view.v0.z >= 0.0 || tri_view.v1.z >= 0.0 || tri_view.v2.z >= 0.0 {
                    return None;
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

                let tri_screen = tri_proj.viewport_transform(self.width as f32, self.height as f32);

                if !tri_screen.is_front_facing() {
                    return None;
                }

                // allocate local buffers for this triangle
                let mut local_raster =
                    vec![0; self.width * self.height / cur_thread.as_u64().get() as usize];
                let mut local_z = vec![f32::NEG_INFINITY; self.width * self.height];

                rasterizer::rasterizer(&mut local_raster, tri_screen, &mut local_z);

                Some((local_raster, local_z))
            })
            .collect();

        // Merge step
        for (local_raster, local_z) in partial_results {
            for i in 0..self.raster.len() {
                if local_z[i] > self.z_buffer[i] {
                    self.z_buffer[i] = local_z[i];
                    self.raster[i] = local_raster[i];
                }
            }
        }
    }
}
