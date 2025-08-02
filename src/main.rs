mod models;
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

    for _ in 0..100 {
        triangles.push(Triangle::random(
            0.0,
            0.0,
            0.0,
            WIDTH as f32 - 10.0,
            HEIGHT as f32 - 10.0,
            0.0,
        ));
    }

    while window.is_open() {
        window::clear_raster(&mut raster, black.to_u32());
        for triangle in triangles.iter() {
            rasterizer::draw_triangle(&mut raster, *triangle);
        }

        window.update_with_buffer(&raster, WIDTH, HEIGHT).unwrap();
    }
}
