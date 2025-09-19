mod camera;
mod graphics;
mod loaders;
mod logging;
mod models;
mod window;

use graphics::render_pipeline::RenderPipeline;
use models::vec3::Vec3;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 1280;
pub const OBJ_PATH: &str = "assets/Velociraptor.obj";

fn main() {
    let mut render_pipeline = RenderPipeline::new(
        Vec3::new(1000.0, 500.0, 0.0),
        0.0,
        180.0,
        WIDTH,
        HEIGHT,
        OBJ_PATH,
    );

    render_pipeline.render_loop();
}
