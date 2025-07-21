use minifb::{Window, WindowOptions};

pub struct InitWindowResult(pub Window, pub Vec<u32>);

/// Initializes a window and a raster buffer of the given size
/// Returns a struct containing the window and the raster buffer
/// * `height`: height in pixles of the window
/// * `width`: width in pixles of the window
pub fn init_window(width: usize, height: usize, color: u32) -> InitWindowResult {
    let window = Window::new(
        "3d Render",
        width,
        height,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    InitWindowResult(window, init_raster(height, width, color))
}

/// Initializes a raster buffer of the given size
///
/// * `height`: height in pixles of the window
/// * `width`: width in pixles of the window
/// * `color`: color of the buffer
pub fn init_raster(height: usize, width: usize, color: u32) -> Vec<u32> {
    vec![color; height * width]
}

pub fn clear_raster(buffer: &mut [u32], color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}
