use minifb::{Window, WindowOptions};

/// Initializes a window
/// Returns a struct containing the window and the raster buffer
/// * `height`: height in pixles of the window
/// * `width`: width in pixles of the window
pub fn init_window(width: usize, height: usize) -> Window {
    Window::new(
        "3d Render",
        width,
        height,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap()
}
