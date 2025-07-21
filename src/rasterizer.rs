use crate::models::color::Color;
use crate::models::triangle::Triangle;
use crate::models::vec2::Vec2;

use crate::WIDTH;

struct BoundingBoxData(usize, usize, usize, usize);

pub fn draw_triangle(raster: &mut [u32], tri: Triangle) {
    let (projected_v0, projected_v1, projected_v2): (Vec2, Vec2, Vec2) = project_triangle(tri);

    let BoundingBoxData(min_x, min_y, max_x, max_y): BoundingBoxData =
        bounding_box(projected_v0, projected_v1, projected_v2);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if point_in_triangle(x, y, (projected_v0, projected_v1, projected_v2)) {
                draw_pixel(x, y, raster);
            }
        }
    }
}

/// will project the given trianlge into screen space
/// currently only does orthogaphic projection
///
/// * `tri`: triangle to be projected
fn project_triangle(tri: Triangle) -> (Vec2, Vec2, Vec2) {
    (
        Vec2::new(tri.v0.x, tri.v0.y),
        Vec2::new(tri.v1.x, tri.v1.y),
        Vec2::new(tri.v2.x, tri.v2.y),
    )
}

/// will calculate the bounding box of the given triangle
///
/// * `v0`: vertex of triangle
/// * `v1`: vertex of triangle
/// * `v2`: vertex of triangle
fn bounding_box(v0: Vec2, v1: Vec2, v2: Vec2) -> BoundingBoxData {
    let min_x = f32::min(f32::min(v0.x, v1.x), v2.x) as usize;
    let min_y = f32::min(f32::min(v0.y, v1.y), v2.y) as usize;
    let max_x = f32::max(f32::max(v0.x, v1.x), v2.x) as usize;
    let max_y = f32::max(f32::max(v0.y, v1.y), v2.y) as usize;

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

/// will draw a pixel to the specified (x,y) values in the passed raster
///
/// * `x`: x value to write
/// * `y`: y value to write
/// * `raster`: raster to write to
fn draw_pixel(x: usize, y: usize, raster: &mut [u32]) {
    raster[y * WIDTH + x] = Color::new(255, 255, 255).to_u32();
}
