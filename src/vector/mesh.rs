use super::vector::Vector3;

pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<(usize, usize)>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, indices: Vec<(usize, usize)>) -> Self {
        Mesh { vertices, indices }
    }
}
