use crate::{
    models::{camera::Camera, fps::FpsCounter, renderer::Renderer, triangle::Triangle, vec3::Vec3},
    obj_loader::load_obj_file,
};

#[derive(Debug)]
pub struct RenderPipeline {
    camera: Camera,
    renderer: Renderer,
    tris: Vec<Triangle>,
    raster: Vec<u32>,
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
            fps_counter: FpsCounter::new(),
        }
    }

    pub fn render_loop(&mut self) {
        while self.renderer.window.is_open() {
            if let Some(fps) = self.fps_counter.tick() {
                println!("FPS: {fps}");
            }
            self.camera.get_movement(&self.renderer.window);
            self.renderer
                .render(&mut self.tris, &mut self.raster, &self.camera);
        }
    }
}
