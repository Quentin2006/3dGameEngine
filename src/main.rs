mod models;
use std::time::{Duration, Instant};

use models::camera::Camera;
use models::color::Color;
use models::triangle::Triangle;
use models::vec2::Vec2;
use models::vec3::Vec3;

mod rasterizer;
mod window;

const WIDTH: usize = 1280;
const HEIGHT: usize = 1280;

fn is_front_facing((v0, v1, v2): &(Vec2, Vec2, Vec2)) -> bool {
    (v1.x - v0.x) * (v2.y - v0.y) - (v2.x - v0.x) * (v1.y - v0.y) > 0.0
}

fn main() {
    let black = Color::new(0, 0, 0);

    let window::InitWindowResult(mut window, mut raster): window::InitWindowResult =
        window::init_window(WIDTH, HEIGHT, black.to_u32());

    let mut triangles: Vec<Triangle> = vec![];

    for _ in 0..10 {
        triangles.push(Triangle::random(-0.5, -0.5, -0.5, 0.5, 0.5, 0.5));
    }

    // NOTE: this looks in the -z direction
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 0.0, 0.0);
    let mut prev_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Discard);

    let mut last_time = Instant::now();
    let mut frames = 0;

    while window.is_open() {
        let cur_time = Instant::now();

        if cur_time.duration_since(last_time) >= Duration::from_secs(1) {
            println!("{frames}");
            last_time = cur_time;
            frames = 0;
        }
        frames += 1;

        // make sure buffer is cleared
        window::clear_raster(&mut raster, black.to_u32());

        camera.get_movement(&window, prev_mouse_pos);
        prev_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Discard);

        let view = camera.view_matrix();
        // draw triangles
        for tri in triangles.iter_mut() {
            // apply camera transformation
            let tri_view = tri.transform(view);

            // apply basic divide projection, we do -z so our prespective isnt flipped
            // NOTE: move to matrix multiplication
            let tri_proj = Triangle::new(
                Vec3::new(
                    tri_view.v0.x / -tri_view.v0.z,
                    tri_view.v0.y / -tri_view.v0.z,
                    tri_view.v0.z,
                ),
                Vec3::new(
                    tri_view.v1.x / -tri_view.v1.z,
                    tri_view.v1.y / -tri_view.v1.z,
                    tri_view.v1.z,
                ),
                Vec3::new(
                    tri_view.v2.x / -tri_view.v2.z,
                    tri_view.v2.y / -tri_view.v2.z,
                    tri_view.v2.z,
                ),
            );
            // skip triangles behind camera
            if tri_view.v0.z >= 0.0 || tri_view.v1.z >= 0.0 || tri_view.v2.z >= 0.0 {
                continue;
            }

            // apply viewport transformation
            let tri_screen = tri_proj.viewport_transform(WIDTH as f32, HEIGHT as f32);

            if !is_front_facing(&tri_screen) {
                continue;
            }

            // rasterization
            rasterizer::rasterizer(&mut raster, tri_screen);
        }

        // update window
        window.update_with_buffer(&raster, WIDTH, HEIGHT).unwrap();
    }
}
