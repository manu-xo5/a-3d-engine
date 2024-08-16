use super::draw;
use super::matrix::*;
use super::vector::vector::{Vector2, Vector3};
use super::window;
use super::Mesh;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn transform_world_space(
    vec: &Vector3,
    camera_angle: &Vector3,
    camera_position: &Vector3,
) -> Vector3 {
    let mut vec = vec.clone();

    // vec = vec * (camera_angle.normalize());
    vec *= translate((camera_position.x, camera_position.y, camera_position.z));
    vec *= translate((0.0, 0.0, 2.0));

    vec
}

pub fn transform_projection(vec: &Vector3) -> Vector2 {
    let mut vec = vec.clone();
    let xf = window::SCREEN_W as f64 / 2.0;
    let yf = window::SCREEN_H as f64 / 2.0;

    vec *= projection_matrix();
    vec *= translate((1.0, 1.0, 0.0));
    vec *= scale((xf, yf, 0.0));

    Vector2::new(vec.x, vec.y)
}

pub fn pump(pos: &Vector3, angle: &Vector3, meshes: Vec<&Mesh>, canvas: &mut Canvas<Window>) {
    for mesh in meshes.into_iter() {
        let mut normals = vec![];
        for i in (0..mesh.vertices.len()).step_by(2) {
            let next_i = i + 1;

            let v1 = mesh.vertices[i];
            let v2 = mesh.vertices[next_i];

            normals.push(v2.dot(&v1));
        }

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
