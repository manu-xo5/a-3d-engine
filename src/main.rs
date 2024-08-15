mod matrix;

mod window;

mod vector;

mod draw;

mod pipeline;

mod keystate;
use keystate::{KeyDirection, KeyState};

use core::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vector::mesh::Mesh;
use vector::vector::{Vector2, Vector3};

fn main() -> Result<(), String> {
    let (window, sdl) = window::create_window();
    let mut event_pipe = sdl.event_pump().unwrap();
    let mut canvas = window::get_canvas(window);

    let mut obj: Mesh = Mesh::new(
        vec![
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::new(0.5, -0.5, 0.5),
            Vector3::new(-0.5, -0.5, 0.5),
        ],
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ],
    );

    println!("Hello, world!");
    let prev_vertex = Vector3::new(0.0, 0.0, 0.0);

    let obj2 = Mesh::new(vec![prev_vertex.clone()], vec![]);
    let mut camera = Vector3::new(0.0, 0.0, 0.0);
    let mut keystate = KeyState::new();

    'gameloop: loop {
        canvas.clear();
        for event in event_pipe.poll_iter() {
            match event {
                Event::MouseWheel { precise_y, .. } => {
                    camera.z += precise_y as f64 * 2.0;
                }
                Event::MouseMotion {
                    mousestate,
                    xrel,
                    yrel,
                    ..
                } => {
                    if mousestate.left() {
                        println!("{:?} {}", camera.x, yrel);
                        let nc = camera
                            .rotate_x(yrel as f64 / 100.0)
                            .rotate_y(xrel as f64 / 100.0);

                        println!("nc {:?}", nc.x);
                        camera = nc;
                        println!(" c {:?}", camera.x);
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    let size = 10.0;
                    let p = Vector2::new((x / size as i32) as f64, (y / size as i32) as f64)
                        .scale(size)
                        .add(&Vector2::new(-size / 2.0, -size / 2.0));

                    let _x = (x - window::SCREEN_W as i32) as f64;
                    let _y = (y - window::SCREEN_H as i32) as f64;

                    canvas.set_draw_color((200, 200, 200));
                    draw::filled_rect(&p, size as u32, size as u32, &mut canvas);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => {
                    break 'gameloop;
                }
                Event::KeyUp { keycode, .. } => keystate.handle_event(keycode, KeyDirection::KeyUp),
                Event::KeyDown { keycode, .. } => {
                    keystate.handle_event(keycode, KeyDirection::KeyDown);
                }

                _ => {}
            }
        }

        let d = 0.04;
        if keystate.w {
            camera = camera.translation(&Vector3::new(0.0, 0.0, -d))
        }
        if keystate.s {
            camera = camera.translation(&Vector3::new(0.0, 0.0, d))
        }

        if keystate.a {
            camera = camera.translation(&Vector3::new(d, 0.0, 0.0))
        }
        if keystate.d {
            camera = camera.translation(&Vector3::new(-d, 0.0, 0.0))
        }
        if keystate.k {
            camera = camera.translation(&Vector3::new(0.0, -d, 0.0))
        }
        if keystate.j {
            camera = camera.translation(&Vector3::new(0.0, d, 0.0))
        }

        pipeline::pump(&camera, vec![&obj, &obj2], &mut canvas);

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

pub fn draw_grid(vertices: &Vec<Vector3>, canvas: &mut Canvas<Window>) {
    let size = 30;
    let offset = 10;
    for i in 1..(window::SCREEN_W / size) {
        let vs = Vector2::new((i * size) as f64 - (offset / 2) as f64, 0.0);
        canvas.set_draw_color((100, 100, 100));
        draw::line(
            &vs,
            &vs.add(&Vector2::new(0.0, window::SCREEN_H as f64)),
            canvas,
        );
    }

    for i in 1..(window::SCREEN_H / size) {
        let vs = Vector2::new(0.0, (i * size) as f64 - (offset / 2) as f64);
        canvas.set_draw_color((100, 100, 100));
        draw::line(
            &vs,
            &vs.add(&Vector2::new(window::SCREEN_W as f64, 0.0)),
            canvas,
        );
    }

    for v in vertices {
        let size = 10;
        canvas.set_draw_color((255, 0, 0));
        draw::rect(
            &Vector2::new(v.x, v.z).add(&Vector2::new(
                window::SCREEN_W as f64 / 2.0,
                window::SCREEN_H as f64 / 2.0,
            )),
            size,
            size,
            canvas,
        );
    }
}
