use super::draw;
use super::matrix::*;
use super::vector::vector::Vector3;
use super::window;
use super::Mesh;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn culling(normal: &Vector3, vec: &Vector3, camera_position: &Vector3) -> bool {
    let vec = *vec - *camera_position;
    let _ = !(normal.normalize().dot(&vec) < 0.0);
    return false;
}

pub fn transform_world_space(
    vec: &Vector3,
    camera_angle: &Matrix<4, 4>,
    camera_position: &Vector3,
) -> Vector3 {
    let mut vec = vec.clone();

    let z_offset = 2.0;
    vec *= scale((1.0, -1.0, 1.0));

    vec *= translate((
        -camera_position.x,
        -camera_position.y,
        -camera_position.z + z_offset,
    ));

    vec *= translate((0.0, 0.0, 0.0)) * camera_angle;

    vec
}

pub fn transform_projection(vec: &Vector3) -> Vector3 {
    let mut vec = vec.clone();
    let xf = window::SCREEN_W as f64 / 2.0;
    let yf = window::SCREEN_H as f64 / 2.0;

    vec *= projection_matrix();
    vec *= translate((1.0, 1.0, 0.0));
    vec *= scale((xf, yf, 0.0));

    Vector3::new(vec.x, vec.y, vec.z)
}

pub fn pump(pos: &Vector3, angle: &Matrix<4, 4>, meshes: Vec<&Mesh>, canvas: &mut Canvas<Window>) {
    let get_vertex = |(i, j, k): (usize, usize, usize), vertices: &Vec<Vector3>| {
        (vertices[i], vertices[j], vertices[k])
    };

    for mesh in meshes.into_iter() {
        let vertices: Vec<Vector3> = (&mesh.vertices)
            .into_iter()
            .map(|v| transform_world_space(&v, angle, pos))
            .map(|v| transform_projection(&v))
            .collect();

        let () = (mesh.indices)
            .iter()
            .filter(|(i, j, k)| {
                let (p1, p2, p3) = get_vertex((*i, *j, *k), &vertices);
                let face_normal = (p1 - p2).cross(&(p2 - p3));
                !culling(&face_normal, &p2, &pos)
            })
            .for_each(|(i, j, k)| {
                let (p1, p2, p3) = get_vertex((*i, *j, *k), &vertices);
                canvas.set_draw_color((255, 200, 200));
                draw::filled_triangle(&p1.to_vector2(), &p2.to_vector2(), &p3.to_vector2(), canvas);

                // let (p1, p2, p3) = get_vertex((*i, *j, *k), &mesh.vertices);

                // let face_normal = (p1 - p2).cross(&(p2 - p3));
                // let c = (p1 + p2 + p3) / 3.0;
                // let face_normal = c + (face_normal.normalize() * 0.3);

                // let c = transform_projection(&transform_world_space(&c, angle, pos));
                // let face_normal =
                //     transform_projection(&transform_world_space(&face_normal, angle, pos));

                // canvas.set_draw_color((0, 0, 200));
                // draw::line(&c.to_vector2(), &face_normal.to_vector2(), canvas);
            });
    }
}
