use crate::{
    camera::camera::Camera,
    graphics::renderer::Renderer,
    loaders::obj_loader::load_obj_file,
    logging::fps::FpsCounter,
    models::{triangle::Triangle, vec3::Vec3},
};

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

    pub fn render_loop(&mut self) {
        while self.renderer.window.is_open() {
            // Update FPS
            if let Some(fps) = self.fps_counter.tick() {
                println!("FPS: {fps}");
            }

            // Handle camera movement
            self.camera.get_movement(&self.renderer.window);

            let view = self.camera.view_matrix();

            // Render all triangles
            for tri in &self.tris {
                let tri_view = tri.transform(view);
                self.renderer
                    .render_tri(&tri_view, &mut self.z_buffer, &mut self.raster);
            }

            // Draw the frame and clear buffers
            self.renderer
                .draw_raster(&mut self.raster, &mut self.z_buffer);
        }
    }
}
