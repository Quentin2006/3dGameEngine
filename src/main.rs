mod models;
use models::color::Color;
use models::triangle::Triangle;
use models::vec3::Vec3;

mod rasterizer;
mod window;

pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 128;

fn main() {
    let black = Color::new(0, 0, 0);

    let window::InitWindowResult(mut window, mut raster): window::InitWindowResult =
        window::init_window(WIDTH, HEIGHT, black.to_u32());

    // NOTE: the tranagle must be made in CCW order
    let tri = Triangle::new(
        Vec3::new(112.0, 81.0, 0.0),
        Vec3::new(19.0, 104.0, 0.0),
        Vec3::new(18.0, 16.0, 0.0),
    );

    while window.is_open() {
        window::clear_raster(&mut raster, black.to_u32());
        rasterizer::draw_triangle(&mut raster, tri);

        window.update_with_buffer(&raster, WIDTH, HEIGHT).unwrap();
    }
}
