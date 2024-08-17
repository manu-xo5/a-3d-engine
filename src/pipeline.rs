use super::draw;
use super::matrix::*;
use super::vector::vector::{Vector2, Vector3};
use super::window;
use super::Mesh;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn culling(normal: &Vector3, vec: &Vector3, camera_position: &Vector3) -> bool {
    let vec = *vec - *camera_position;
    normal.normalize().dot(&vec) < 0.0
}

pub fn transform_world_space(
    vec: &Vector3,
    _camera_angle: &Vector3,
    camera_position: &Vector3,
) -> Vector3 {
    let mut vec = vec.clone();

    vec *= translate((camera_position.x, camera_position.y, camera_position.z))
        * translate((0.0, -2.0, 8.0));

    vec
}

pub fn transform_projection(vec: &Vector3) -> Vector3 {
    let mut vec = vec.clone();
    let xf = window::SCREEN_W as f64 / 2.0;
    let yf = window::SCREEN_H as f64 / 2.0;

    vec *= scale((1.0, -1.0, 1.0));
    vec *= projection_matrix();
    vec *= translate((1.0, 1.0, 0.0));
    vec *= scale((xf, yf, 0.0));

    Vector3::new(vec.x, vec.y, vec.z)
}

pub fn pump(pos: &Vector3, angle: &Vector3, meshes: Vec<&Mesh>, canvas: &mut Canvas<Window>) {
    for mesh in meshes.into_iter() {
        let vertices: Vec<Vector3> = (&mesh.vertices)
            .into_iter()
            .map(|v| transform_world_space(&v, angle, pos))
            .map(|v| transform_projection(&v))
            .collect();

        for (i, j, k) in (&mesh.indices).iter() {
            let p1 = vertices[*i];
            let p2 = vertices[*j];
            let p3 = vertices[*k];

            let face_normal = (p1 - p2).cross(&(p2 - p3)).normalize();

            if culling(&face_normal, &p1, &pos) {
                canvas.set_draw_color((255, 200, 200));
                draw::triangle(&p1.to_vector2(), &p2.to_vector2(), &p3.to_vector2(), canvas);
            }
        }
    }
}
