use super::draw;
use super::matrix;
use super::matrix::*;
use super::vector::vector::{Vector2, Vector3};
use super::window;
use super::Mesh;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn transform_world_space(
    vec: &Vector3,
    camera_angle: &Matrix<4, 4>,
    _camera_position: &Vector3,
) -> Vector3 {
    let vec = vec.clone();
    // let vec = vec * *camera_angle;

    (camera_angle * vec) + *_camera_position
}

pub fn transform_projection(vec: &Vector3) -> Vector2 {
    let z_off = 2.0;
    let zf = if (vec.z + z_off) == 0.0 {
        1.0
    } else {
        1.0 / (vec.z + z_off)
    };

    let xf = window::SCREEN_W as f64 / 2.0;
    let yf = window::SCREEN_H as f64 / 2.0;

    Vector2::new((vec.x * zf + 1.0) * xf, (vec.y * zf + 1.0) * yf)
}

pub fn pump(pos: &Vector3, angle: &Matrix<4, 4>, meshes: Vec<&Mesh>, canvas: &mut Canvas<Window>) {
    for mesh in meshes.into_iter() {
        let vertices: Vec<Vector2> = (&mesh.vertices)
            .into_iter()
            .map(|v| transform_world_space(&v, angle, pos))
            .map(|v| transform_projection(&v))
            .collect();

        for (i, j) in (&mesh.indices).into_iter() {
            let p1 = vertices[*i];
            let p2 = vertices[*j];

            canvas.set_draw_color((255, 200, 200));
            draw::line(&p1, &p2, canvas);
        }
    }
}
