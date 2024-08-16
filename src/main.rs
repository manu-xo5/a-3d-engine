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
use std::f64::consts::PI;
use vector::mesh::Mesh;
use vector::vector::{Vector2, Vector3};

fn main() -> Result<(), String> {
    let (window, sdl) = window::create_window();
    let mut event_pipe = sdl.event_pump().unwrap();
    let mut canvas = window::get_canvas(window);

    let mut obj = Mesh::from_file("bed-model.obj");
    let mut obj2: Mesh = Mesh::new(
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

    let mut camera_pos = Vector3::new(0.1, 0.1, 0.1);
    let mut camera_angle = Vector3::new(0.0, 0.0, 1.0);
    let mut keystate = KeyState::new();

    'gameloop: loop {
        canvas.clear();
        for event in event_pipe.poll_iter() {
            match event {
                Event::MouseMotion { xrel, yrel, .. } => {
                    let _wrap = |x: f64| {
                        let x = x - (x % (2.0 * PI));
                        if x > PI {
                            x - PI
                        } else {
                            x
                        }
                    };

                    let dt = 1e-3;
                    let x_rot = yrel as f64 * dt;
                    let y_rot = xrel as f64 * dt;

                    println!("{} {}", camera_angle.x, camera_angle.y);
                    camera_angle = matrix::rotate_y(y_rot) * matrix::rotate_z(x_rot) * camera_angle;
                    println!("{} {}", camera_angle.x, camera_angle.y);
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
            camera_pos = camera_pos.translation(&Vector3::new(0.0, 0.0, -d))
        }
        if keystate.s {
            camera_pos = camera_pos.translation(&Vector3::new(0.0, 0.0, d))
        }

        if keystate.a {
            camera_pos = camera_pos.translation(&Vector3::new(d, 0.0, 0.0))
        }
        if keystate.d {
            camera_pos = camera_pos.translation(&Vector3::new(-d, 0.0, 0.0))
        }
        if keystate.j {
            camera_angle = matrix::rotate_x(d) * camera_angle /* * matrix::rotate_z(0.0) */
        }
        if keystate.k {
            camera_angle = matrix::rotate_x(-d) * camera_angle /* * matrix::rotate_z(0.0) */
            // obj2 = obj2.mul(&matrix::rotate_z(-0.05)); /* * matrix::rotate_z(0.0) */
        }
        if keystate.h {
            camera_angle = matrix::rotate_y(d) * camera_angle /* * matrix::rotate_z(0.0) */;
            println!("{:?}", camera_angle);
        }
        if keystate.l {
            camera_angle = matrix::rotate_y(-d) * camera_angle /* * matrix::rotate_z(0.0) */
            // obj2 = obj2.mul(&matrix::rotate_z(-0.05)); /* * matrix::rotate_z(0.0) */
        }

        pipeline::pump(&camera_pos, &camera_angle, vec![&obj, &obj2], &mut canvas);

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
