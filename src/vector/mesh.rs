use super::super::matrix::*;
use super::vector::Vector3;
use std::fs::File;
use std::io::prelude::*;

pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<(usize, usize)>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, indices: Vec<(usize, usize)>) -> Self {
        Mesh { vertices, indices }
    }

    pub fn mul(&self, mat: &Matrix<4, 4>) -> Mesh {
        let mut vertices: Vec<Vector3> = vec![];
        let mut indices: Vec<(usize, usize)> = vec![];

        for v in &self.vertices {
            vertices.push(mat * *v);
        }

        for i in &self.indices {
            indices.push(*i)
        }

        Mesh { vertices, indices }
    }

    pub fn from_file(filename: &str) -> Mesh {
        let mut vertices = vec![];
        let mut indices = vec![];
        let mut file = File::open(filename).unwrap();
        let mut data_str = String::new();

        file.read_to_string(&mut data_str).unwrap();

        let _coords_type = data_str.lines().next().unwrap().to_string();

        let mut reading_flag = 0;

        for line in data_str.lines() {
            if !line.starts_with("v") {
                if reading_flag == 1 {
                    break;
                } else {
                    continue;
                }
            }

            reading_flag = 1;

            let coords = line
                .split_at(2)
                .1
                .split(" ")
                .map(|x| x.parse::<f64>().unwrap_or(0.0))
                .collect::<Vec<f64>>();

            vertices.push(Vector3::new(coords[0], coords[1], coords[2]));
        }

        reading_flag = 0;
        for line in data_str.lines() {
            if !line.starts_with("f") {
                if reading_flag == 1 {
                    break;
                } else {
                    continue;
                }
            }

            reading_flag = 1;

            let faces = line
                .split_at(2)
                .1
                .split("/")
                .map(|x| x.parse::<usize>().unwrap_or(0))
                .collect::<Vec<usize>>();

            indices.push((faces[0], faces[1]));
            indices.push((faces[1], faces[2]));
            indices.push((faces[2], faces[0]));
        }

        Mesh::new(vertices, indices)
    }
}
