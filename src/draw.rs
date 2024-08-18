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

pub fn filled_triangle(p1: &Vector2, p2: &Vector2, p3: &Vector2, canvas: &mut Canvas<Window>) {
    let (p1, p2) = if p1.y > p2.y { (p1, p2) } else { (p2, p1) };
    let (p1, p3) = if p1.y > p3.y { (p1, p3) } else { (p3, p1) };
    let (p2, p3) = if p2.y > p3.y { (p2, p3) } else { (p3, p2) };

    let x2 = draw_flat_bottom_triangle(p1, p2, p3, canvas);
    let p3 = &Vector2::new(x2, p2.y);

    let _ = draw_flat_top_triangle(p1, p2, p3, canvas);

    canvas.set_draw_color((0, 0, 0));
}

pub fn draw_flat_bottom_triangle(
    p1: &Vector2,
    p2: &Vector2,
    p3: &Vector2,
    canvas: &mut Canvas<Window>,
) -> f64 {
    let d = p3.sub(p2);
    let m1 = d.x / d.y;
    let mut x1 = p3.x;

    let d = p3.sub(p1);
    let m2 = d.x / d.y;
    let mut x2 = p3.x;

    println!("{}, {} {}", p1.y, p2.y, p3.y);

    for y_off in (p3.y as u64)..(p2.y as u64) {
        let y_off = y_off as f64;
        canvas.set_draw_color((100, 100, 100));

        line(&Vector2::new(x1, y_off), &Vector2::new(x2, y_off), canvas);

        x1 += m1;
        x2 += m2;
    }

    x2
}

pub fn draw_flat_top_triangle(
    p1: &Vector2,
    p2: &Vector2,
    p3: &Vector2,
    canvas: &mut Canvas<Window>,
) -> f64 {
    let d = p2.sub(p1);
    let m1 = d.x / d.y;
    let mut x1 = p2.x;

    let d = p3.sub(p1);
    let m2 = d.x / d.y;
    let mut x2 = p3.x;

    for y_off in (p2.y as u64)..(p1.y as u64) {
        let y_off = y_off as f64;
        canvas.set_draw_color((100, 100, 100));

        line(&Vector2::new(x1, y_off), &Vector2::new(x2, y_off), canvas);

        x1 += m1;
        x2 += m2;
    }

    0.0
}
