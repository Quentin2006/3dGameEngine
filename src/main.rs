mod models;
use models::camera::Camera;
use models::color::Color;
use models::triangle::Triangle;
use models::vec3::Vec3;

mod rasterizer;
mod window;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 1280;

fn main() {
    let black = Color::new(0, 0, 0);

    let window::InitWindowResult(mut window, mut raster): window::InitWindowResult =
        window::init_window(WIDTH, HEIGHT, black.to_u32());

    let mut triangles: Vec<Triangle> = vec![];

    for _ in 0..10 {
        triangles.push(Triangle::random(
            0.0,
            0.0,
            0.1,
            WIDTH as f32,
            HEIGHT as f32,
            10.0,
        ));
    }

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0);
    while window.is_open() {
        // make sure buffer is cleared
        window::clear_raster(&mut raster, black.to_u32());

        let forward = camera.direction();
        camera.position = camera.position + forward * 0.01; // move forward slowly
        let view = camera.view_matrix();

        // draw triangles
        for triangle in triangles.iter() {
            // Transform triangle vertices by view matrix (and projection)
            let transformed_triangle = triangle.transform(view);

            // Draw transformed triangle
            rasterizer::draw_triangle(&mut raster, transformed_triangle);
        }

        // update window
        window.update_with_buffer(&raster, WIDTH, HEIGHT).unwrap();
    }
}
