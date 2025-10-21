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
    println!("Initializing render pipeline...");
    println!("Loading OBJ file: {}", OBJ_PATH);

    let mut render_pipeline = RenderPipeline::new(
        Vec3::new(1000.0, 500.0, 0.0), // camera position
        0.0,                           // pitch
        180.0,                         // yaw
        WIDTH,
        HEIGHT,
        OBJ_PATH,
    );

    println!("Starting render loop...");
    render_pipeline.render_loop();
}
