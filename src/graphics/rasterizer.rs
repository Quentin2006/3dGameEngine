use crate::models::{color, triangle::Triangle, vec2::Vec2};
use color::Color;

pub struct BoundingBoxData(pub usize, pub usize, pub usize, pub usize);

pub fn rasterizer(
    raster: &mut [u32],
    tri: &Triangle,
    z_buffer: &mut [f32],
    width: usize,
    height: usize,
) {
    let BoundingBoxData(mut min_x, mut min_y, mut max_x, mut max_y) =
        bounding_box(tri, width, height);

    // clamp to screen
    min_x = min_x.max(0);
    min_y = min_y.max(0);
    max_x = max_x.min(width - 1);
    max_y = max_y.min(height - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !point_in_triangle(x, y, tri) {
                continue;
            }

            // compute barycentric coordinates once
            let (u, v, w) = barycentric_coords(tri, x, y);
            let z = u * tri.v0.z + v * tri.v1.z + w * tri.v2.z;

            let idx = y * width + x;
            if z > z_buffer[idx] {
                z_buffer[idx] = z;
                raster[idx] = interpolate_color_from_bary(
                    tri,
                    u,
                    v,
                    w,
                    &color::RED,
                    &color::GREEN,
                    &color::BLUE,
                )
                .to_u32();
            }
        }
    }
}

/// Calculate bounding box for triangle
pub fn bounding_box(tri: &Triangle, width: usize, height: usize) -> BoundingBoxData {
    let min_x = f32::max(0.0, f32::min(f32::min(tri.v0.x, tri.v1.x), tri.v2.x)) as usize;
    let min_y = f32::max(0.0, f32::min(f32::min(tri.v0.y, tri.v1.y), tri.v2.y)) as usize;
    let max_x = f32::min(
        width as f32 - 1.0,
        f32::max(f32::max(tri.v0.x, tri.v1.x), tri.v2.x),
    ) as usize;
    let max_y = f32::min(
        height as f32 - 1.0,
        f32::max(f32::max(tri.v0.y, tri.v1.y), tri.v2.y),
    ) as usize;
    BoundingBoxData(min_x, min_y, max_x, max_y)
}

/// Check if point is inside triangle using edge function
pub fn point_in_triangle(x: usize, y: usize, tri: &Triangle) -> bool {
    let p = Vec2::new(x as f32, y as f32);
    let v0 = Vec2::new(tri.v0.x, tri.v0.y);
    let v1 = Vec2::new(tri.v1.x, tri.v1.y);
    let v2 = Vec2::new(tri.v2.x, tri.v2.y);

    is_to_left(&p, &v0, &v1) && is_to_left(&p, &v1, &v2) && is_to_left(&p, &v2, &v0)
}

fn is_to_left(p: &Vec2, v0: &Vec2, v1: &Vec2) -> bool {
    (v1.x - v0.x) * (p.y - v0.y) - (v1.y - v0.y) * (p.x - v0.x) >= 0.0
}

/// Barycentric coordinates in 2D
fn barycentric_coords(tri: &Triangle, x: usize, y: usize) -> (f32, f32, f32) {
    let a = Vec2::new(tri.v0.x, tri.v0.y);
    let b = Vec2::new(tri.v1.x, tri.v1.y);
    let c = Vec2::new(tri.v2.x, tri.v2.y);
    let p = Vec2::new(x as f32, y as f32);

    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;
    if denom.abs() < f32::EPSILON {
        return (0.0, 0.0, 0.0);
    } // degenerate triangle

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}

/// Interpolate color using precomputed barycentric coords
fn interpolate_color_from_bary(
    _tri: &Triangle,
    u: f32,
    v: f32,
    w: f32,
    a_color: &Color,
    b_color: &Color,
    c_color: &Color,
) -> Color {
    let r = (u * a_color.r as f32 + v * b_color.r as f32 + w * c_color.r as f32).round() as u8;
    let g = (u * a_color.g as f32 + v * b_color.g as f32 + w * c_color.g as f32).round() as u8;
    let b = (u * a_color.b as f32 + v * b_color.b as f32 + w * c_color.b as f32).round() as u8;

    Color::new(r, g, b)
}
