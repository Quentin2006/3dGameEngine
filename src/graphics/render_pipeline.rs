use {core::f32, rayon::prelude::*};

use crate::{
    HEIGHT, WIDTH,
    camera::camera::Camera,
    graphics::renderer::{Renderer, render_tri},
    loaders::obj_loader::load_obj_file,
    logging::fps::FpsCounter,
    models::{triangle::Triangle, vec3::Vec3},
};

/// Runs the main render loop, managing the camera, renderer, and frame data.
///
/// * `camera`: the camera used for view transformations
/// * `renderer`: the renderer responsible for drawing to the window
/// * `tris`: the triangles loaded from the OBJ file
/// * `raster`: the raster buffer storing pixel color data
/// * `z_buffer`: the z-buffer for depth testing
/// * `fps_counter`: the FPS counter for performance monitoring
#[derive(Debug)]
pub struct RenderPipeline {
    camera: Camera,
    renderer: Renderer,
    tris: Vec<Triangle>,
    raster: Vec<u32>,
    z_buffer: Vec<f32>,
    fps_counter: FpsCounter,
}

impl RenderPipeline {
    pub fn new(
        position: Vec3,
        pitch: f32,
        yaw: f32,
        width: usize,
        height: usize,
        obj_file_path: &str,
    ) -> Self {
        Self {
            camera: Camera::new(position, pitch, yaw),
            renderer: Renderer::new(width, height),
            tris: load_obj_file(obj_file_path),
            raster: vec![0; width * height],
            z_buffer: vec![f32::NEG_INFINITY; width * height],
            fps_counter: FpsCounter::new(),
        }
    }

    /// runs the main render loop, handling camera movement, rendering triangles, and displaying
    /// the frame.
    pub fn render_loop(&mut self) {
        while self.renderer.window.is_open() {
            if let Some(fps) = self.fps_counter.tick() {
                println!("FPS: {fps}");
            }
            self.camera.get_movement(&self.renderer.window);

            let raster = &mut self.raster;
            let z_buffer = &mut self.z_buffer;
            let tris = &self.tris;
            let view = self.camera.view_matrix();

            const THREADS: usize = 12;
            let rows_per_chunk = HEIGHT / THREADS;
            let chunk_size = WIDTH * rows_per_chunk;

            z_buffer
                .par_chunks_mut(chunk_size)
                .zip(raster.par_chunks_mut(chunk_size))
                .enumerate()
                .for_each(|(i, (z_chunk, raster_chunk))| {
                    let start = i * chunk_size;
                    let end = start + raster_chunk.len(); // last chunk may be smaller

                    for tri in tris.iter() {
                        let tri = tri.transform(view);
                        render_tri(&tri, z_chunk, raster_chunk, start, end);
                    }
                });

            self.renderer
                .draw_raster(&mut self.raster, &mut self.z_buffer);
        }
    }
}
