use minifb::{Window, WindowOptions};

mod models;
use models::color::Color;
use models::triangle::Triangle;
use models::vec3::Vec3;

mod rasterizer;
use rasterizer::*;

pub fn clear_buffer(buffer: &mut [u32], color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

fn main() {
    let color = Color::new(0, 0, 0);
    let tri_color = Color::new(255, 0, 0);

    let mut raster: Vec<u32> = vec![color.to_u32(); WIDTH * HEIGHT];

    let mut window = Window::new(
        "3d Render",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    // NOTE: the tranagle must be made in CCW order
    let mut tri = Triangle::new(
        Vec3::new(100.0, 100.0, 0.0),
        Vec3::new(500.0, 300.0, 0.0),
        Vec3::new(1000.0, 1000.0, 0.0),
    );
    while window.is_open() {
        tri.add(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        clear_buffer(&mut raster, color.to_u32());
        draw_triangle(&mut raster, &tri, tri_color.to_u32());

        window.update_with_buffer(&raster, WIDTH, HEIGHT).unwrap();
    }
}
