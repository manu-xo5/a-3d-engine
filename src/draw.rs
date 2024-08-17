use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::vector::vector::Vector2;

pub fn rect(vector: &Vector2, length: u32, width: u32, canvas: &mut Canvas<Window>) {
    let _ = canvas
        .draw_rect(Rect::new(vector.x as i32, vector.y as i32, length, width))
        .ok()
        .unwrap_or_default();
    canvas.set_draw_color((0, 0, 0));
}

pub fn filled_rect(vector: &Vector2, length: u32, width: u32, canvas: &mut Canvas<Window>) {
    canvas
        .fill_rect(Rect::new(vector.x as i32, vector.y as i32, length, width))
        .unwrap();
    canvas.set_draw_color((0, 0, 0));
}

pub fn line(p1: &Vector2, p2: &Vector2, canvas: &mut Canvas<Window>) {
    canvas
        .draw_line(
            Point::new(p1.x as i32, p1.y as i32),
            Point::new(p2.x as i32, p2.y as i32),
        )
        .unwrap();
    canvas.set_draw_color((0, 0, 0));
}

pub fn triangle(p1: &Vector2, p2: &Vector2, p3: &Vector2, canvas: &mut Canvas<Window>) {
    canvas
        .draw_line(
            Point::new(p1.x as i32, p1.y as i32),
            Point::new(p2.x as i32, p2.y as i32),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(p2.x as i32, p2.y as i32),
            Point::new(p3.x as i32, p3.y as i32),
        )
        .unwrap();
    canvas
        .draw_line(
            Point::new(p3.x as i32, p3.y as i32),
            Point::new(p1.x as i32, p1.y as i32),
        )
        .unwrap();

    canvas.set_draw_color((0, 0, 0));
}
