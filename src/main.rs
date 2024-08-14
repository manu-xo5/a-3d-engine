mod matrix;

mod window;

mod vector;

mod draw;

use core::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use vector::mesh::Mesh;
use vector::vector::Vector3;

fn main() -> Result<(), String> {
    let (window, sdl) = window::create_window();
    let mut event_pipe = sdl.event_pump().unwrap();
    let mut canvas = window::get_canvas(window);

    let mut obj: Mesh = Mesh::new(
        vec![
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(0.2, -0.5, -0.5),
            Vector3::new(0.5, 0.5, 0.5),
        ],
        vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 1), (3, 2)],
    );

    println!("Hello, world!");
    let mut i = 0.0;
    'gameloop: loop {
        i += 1.0;
        if i >= 360.0 {
            i = 0.0;
        }

        canvas.clear();
        for event in event_pipe.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => {
                    break 'gameloop;
                }
                _ => {}
            }
        }

        obj.rotate(i).render(&mut canvas);

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
