mod models;
mod obj_loader;
use core::f32;
use std::time::{Duration, Instant};

use models::camera::Camera;
use models::color::Color;
use models::triangle::Triangle;
use models::vec3::Vec3;
use obj_loader::load_obj_file;

mod rasterizer;
mod window;

const WIDTH: usize = 1280;
const HEIGHT: usize = 1280;
fn is_front_facing(tri: &Triangle) -> bool {
    (tri.v1.x - tri.v0.x) * (tri.v2.y - tri.v0.y) - (tri.v2.x - tri.v0.x) * (tri.v1.y - tri.v0.y)
        > 0.0
}
fn main() {
    let black = Color::new(0, 0, 0);

    let window::InitWindowResult(mut window, mut raster): window::InitWindowResult =
        window::init_window(WIDTH, HEIGHT, black.to_u32());

    let mut z_buffer: Vec<f32> = vec![f32::NEG_INFINITY; WIDTH * HEIGHT];

    let mut triangles: Vec<Triangle> = load_obj_file("src/objects/donut.obj");

    // NOTE: this looks in the -z direction
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 500.0), 0.0, 0.0);
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

        // make sure raster and zbuffer is cleared
        window::clear_raster(&mut raster, black.to_u32());

        for z in z_buffer.iter_mut() {
            *z = f32::NEG_INFINITY;
        }

        camera.get_movement(&window, prev_mouse_pos);
        prev_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Discard);

        let view = camera.view_matrix();
        // draw triangles
        for tri in triangles.iter_mut() {
            // apply camera transformation
            let tri_view = tri.transform(view);

            // skip triangles behind camera
            if tri_view.v0.z >= 0.0 || tri_view.v1.z >= 0.0 || tri_view.v2.z >= 0.0 {
                continue;
            }

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

            // apply viewport transformation, x and will will correspond to the screen, and z value
            // will be used for z-buffering
            let tri_screen = tri_proj.viewport_transform(WIDTH as f32, HEIGHT as f32);

            if !is_front_facing(&tri_screen) {
                continue;
            }

            // rasterization
            rasterizer::rasterizer(&mut raster, tri_screen, &mut z_buffer);
        }

        // update window
        window.update_with_buffer(&raster, WIDTH, HEIGHT).unwrap();
    }
}
