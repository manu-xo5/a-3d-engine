use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub const SCREEN_W: u64 = 800;
pub const SCREEN_H: u64 = 600;

pub const FOV: f64 = 90.0;

pub const Z_NEAR: f64 = 100.0;
pub const Z_FAR: f64 = 10.0;

pub fn create_window() -> (Window, Sdl) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("A 3D ENGINE", 800, 600).build().unwrap();

    (window, sdl)
}

pub fn get_canvas(window: Window) -> Canvas<Window> {
    window.into_canvas().build().unwrap()
}
