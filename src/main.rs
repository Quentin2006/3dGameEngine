mod models;
mod obj_loader;

use models::vec3::Vec3;

use crate::models::render_pipeline::RenderPipeline;

mod rasterizer;
mod window;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 1280;
fn main() {
    // let mut counter = FpsCounter::new();
    let mut render_pipeline = RenderPipeline::new(
        Vec3::new(1000.0, 500.0, 0.0),
        0.0,
        180.0,
        WIDTH,
        HEIGHT,
        "src/objects/Velociraptor.obj",
    );

    render_pipeline.render_loop();
}
