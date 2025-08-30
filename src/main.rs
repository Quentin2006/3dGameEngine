mod models;
mod obj_loader;

use models::camera::Camera;
use models::color::BLACK;
use models::fps::FpsCounter;
use models::renderer::Renderer;
use models::triangle::Triangle;
use models::vec3::Vec3;
use obj_loader::load_obj_file;

mod rasterizer;
mod window;

const WIDTH: usize = 1280;
const HEIGHT: usize = 1280;
fn main() {
    let mut triangles: Vec<Triangle> = load_obj_file("src/objects/Velociraptor.obj");

    let mut window = window::init_window(WIDTH, HEIGHT, BLACK.to_u32()).0;
    let mut counter = FpsCounter::new();
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), 0.0, 0.0);
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    while window.is_open() {
        if let Some(fps) = counter.tick() {
            println!("FPS: {fps}");
        }

        camera.get_movement(&window);

        renderer.render(&mut triangles, &camera);

        // update window
        window
            .update_with_buffer(&renderer.raster, WIDTH, HEIGHT)
            .unwrap();
    }
}
