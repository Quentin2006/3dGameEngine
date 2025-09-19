use crate::models::color;
use crate::models::triangle::Triangle;
use crate::models::vec2::Vec2;
use color::Color;

use crate::HEIGHT;
use crate::WIDTH;

struct BoundingBoxData(usize, usize, usize, usize);

/// this fucntion will take in a triangle that is already in screen space and draw the triangle
///
/// * `raster`: raster to draw to
/// * `(v0, v1, v2)`: tuple of vec3's to draw the triangle from
pub fn rasterizer(
    raster_chunk: &mut [u32],
    tri: Triangle,
    z_chunk: &mut [f32],
    start_index: usize,
    end_index: usize,
) {
    let v0 = Vec2::new(tri.v0.x, tri.v0.y);
    let v1 = Vec2::new(tri.v1.x, tri.v1.y);
    let v2 = Vec2::new(tri.v2.x, tri.v2.y);

    let BoundingBoxData(mut min_x, mut min_y, mut max_x, mut max_y) = bounding_box(v0, v1, v2);

    // clamp to screen
    if (min_x as isize) < 0 {
        min_x = 0;
    }
    if (min_y as isize) < 0 {
        min_y = 0;
    }
    if max_x >= WIDTH {
        max_x = WIDTH - 1;
    }
    if max_y >= HEIGHT {
        max_y = HEIGHT - 1;
    }

    // compute the triangle's global start/end indices (inclusive ranges)
    let tri_start = min_y * WIDTH + min_x;
    let tri_end = max_y * WIDTH + max_x + 1; // make tri_end exclusive

    // do they overlap?
    if tri_end <= start_index || tri_start >= end_index {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !point_in_triangle(x, y, (v0, v1, v2)) {
                continue;
            }

            // barycentric / depth
            let (u, v, w) = barycentric_coords(v0, v1, v2, Vec2::new(x as f32, y as f32));
            let z = u * tri.v0.z + v * tri.v1.z + w * tri.v2.z;

            let global_idx = y * WIDTH + x;

            // only handle pixels that belong to this chunk
            if global_idx < start_index || global_idx >= end_index {
                continue;
            }

            let local_idx = global_idx - start_index; // index into the small chunk slice

            if z > z_chunk[local_idx] {
                z_chunk[local_idx] = z;
                raster_chunk[local_idx] = interpolate_color(
                    v0,
                    v1,
                    v2,
                    color::RED,
                    color::GREEN,
                    color::BLUE,
                    Vec2::new(x as f32, y as f32),
                )
                .to_u32();
            }
        }
    }
}

/// will calculate the bounding box of the given triangle
///
/// * `v0`: vertex of triangle
/// * `v1`: vertex of triangle
/// * `v2`: vertex of triangle
fn bounding_box(v0: Vec2, v1: Vec2, v2: Vec2) -> BoundingBoxData {
    let min_x = f32::max(0.0, f32::min(f32::min(v0.x, v1.x), v2.x)) as usize;
    let min_y = f32::max(0.0, f32::min(f32::min(v0.y, v1.y), v2.y)) as usize;
    let max_x = f32::min(WIDTH as f32 - 1.0, f32::max(f32::max(v0.x, v1.x), v2.x)) as usize;
    let max_y = f32::min(HEIGHT as f32 - 1.0, f32::max(f32::max(v0.y, v1.y), v2.y)) as usize;
    BoundingBoxData(min_x, min_y, max_x, max_y)
}

/// will check if a point is in the triangle
///
/// * `x`: x value of point
/// * `y`: y value of point
fn point_in_triangle(
    x: usize,
    y: usize,
    (projected_v0, projected_v1, projected_v2): (Vec2, Vec2, Vec2),
) -> bool {
    // we know if a point is in the triangle if its to the right of each vector
    let right_of_v01 = is_to_left(x, y, projected_v0, projected_v1);
    let right_of_v12 = is_to_left(x, y, projected_v1, projected_v2);
    let right_of_v20 = is_to_left(x, y, projected_v2, projected_v0);
    right_of_v01 && right_of_v12 && right_of_v20
}

/// determines if a point is to the left of a vector
///
/// * `x`: x value of point
/// * `y`: y value of point
/// * `v0`: vector start
/// * `v1`: vector end
fn is_to_left(x: usize, y: usize, v0: Vec2, v1: Vec2) -> bool {
    (v1.x - v0.x) * (y as f32 - v0.y) - (v1.y - v0.y) * (x as f32 - v0.x) >= 0.0
}

/// will calculate the barycentric coordinates of the given point
///
/// * `a`: point a in the triangle
/// * `b`: point b in the triangle
/// * `c`: point c in the triangle
/// * `p`: point to calculate the barycentric coordinates for
fn barycentric_coords(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> (f32, f32, f32) {
    // compute vectors
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    // compute dot products
    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    // compute barycentric coordinates
    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}

/// this will interpolate the color of a point between the colors of the triangle
///
/// * `a`: vertex a of the triangle
/// * `b`: vertex b of the triangle
/// * `c`: vertex c of the triangle
/// * `a_color`: weight of vertex a
/// * `b_color`: weight of vertex b
/// * `c_color`: weight of vertex c
/// * `p`: point to interpolate the color for
fn interpolate_color(
    a: Vec2,
    b: Vec2,
    c: Vec2,
    a_color: Color,
    b_color: Color,
    c_color: Color,
    p: Vec2,
) -> Color {
    let (u, v, w) = barycentric_coords(a, b, c, p);

    let r = (u * a_color.r as f32 + v * b_color.r as f32 + w * c_color.r as f32).round() as u8;
    let g = (u * a_color.g as f32 + v * b_color.g as f32 + w * c_color.g as f32).round() as u8;
    let b = (u * a_color.b as f32 + v * b_color.b as f32 + w * c_color.b as f32).round() as u8;

    Color::new(r, g, b)
}
