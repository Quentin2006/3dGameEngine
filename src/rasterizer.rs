use crate::models::color::Color;
use crate::models::triangle::Triangle;
use crate::models::vec2::Vec2;
use crate::models::vec3::Vec3;

pub const WIDTH: usize = 3440;
pub const HEIGHT: usize = 1440;

pub fn draw_pixel(buffer: &mut [u32], x: usize, y: usize, color: u32) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    buffer[y * WIDTH + x] = color;
}

pub fn project_vertex(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}

pub fn edge_check(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    let u: Vec2 = v1.subtract(&v0);
    let v: Vec2 = p.subtract(&v0);
    u.cross(&v)
}

pub fn point_in_triangle(v0: Vec2, v1: Vec2, v2: Vec2, p: Vec2) -> bool {
    let e0 = edge_check(v1, v0, p);
    let e1 = edge_check(v1, v2, p);
    let e2 = edge_check(v2, v0, p);
    (e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0) || (e0 <= 0.0 && e1 <= 0.0 && e2 <= 0.0)
}

pub fn draw_triangle(buffer: &mut [u32], tri: &Triangle, color: u32) {
    // project the vertices of the triangle
    let v0 = project_vertex(tri.v0);
    let v1 = project_vertex(tri.v1);
    let v2 = project_vertex(tri.v2);

    // get bounding box of the triangle
    let min_x = v0.x.min(v1.x).min(v2.x).floor().max(0.0) as usize;
    let max_x = v0.x.max(v1.x).max(v2.x).ceil().min((WIDTH - 1) as f32) as usize;
    let min_y = v0.y.min(v1.y).min(v2.y).floor().max(0.0) as usize;
    let max_y = v0.y.max(v1.y).max(v2.y).ceil().min((HEIGHT - 1) as f32) as usize;

    // create colors for the vertices
    let c0 = Color::new(255, 0, 0); // red at v0
    let c1 = Color::new(0, 255, 0); // green at v1
    let c2 = Color::new(0, 0, 255); // blue at v2

    let area = edge_check(v0, v1, v2);

    // draw the triangle by iterating over the bounding box
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            // move point to the center of the pixel
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            if point_in_triangle(v0, v1, v2, p) {
                let w0 = edge_check(v1, v2, p).abs() / area;
                let w1 = edge_check(v2, v0, p).abs() / area;
                let w2 = edge_check(v0, v1, p).abs() / area;

                let r = (w0 * c0.r as f32 + w1 * c1.r as f32 + w2 * c2.r as f32) as u8;
                let g = (w0 * c0.g as f32 + w1 * c1.g as f32 + w2 * c2.g as f32) as u8;
                let b = (w0 * c0.b as f32 + w1 * c1.b as f32 + w2 * c2.b as f32) as u8;

                draw_pixel(buffer, x, y, Color::new(r, g, b).to_u32());
            }
        }
    }
}
