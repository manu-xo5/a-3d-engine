use sdl2::render::Canvas;
use sdl2::video::Window;

use super::vector::Vector3;
use crate::draw;

pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<(usize, usize)>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, indices: Vec<(usize, usize)>) -> Self {
        Mesh { vertices, indices }
    }

    pub fn rotate(&mut self, _angle: f64) -> &mut Self {
        for i in 0..self.vertices.len() {
            // self.vertices[i] = self.vertices[i].rotate_y(0.1);
            self.vertices[i] = self.vertices[i].rotate_x(0.04);
        }

        self
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color((255, 255, 255));

        let offset = 0.0;
        for (i, i2) in self.indices.iter() {
            let p1 = self.vertices[*i]
                .translation(&Vector3::new(offset, offset, offset))
                .project()
                .to_vector2();

            let p2 = self.vertices[*i2]
                .translation(&Vector3::new(offset, offset, offset))
                .project()
                .to_vector2();

            canvas.set_draw_color((255, 255, 255));
            draw::line(&p1, &p2, canvas);
        }
    }
}
