mod matrix;
use matrix::Matrix;

const SCREEN_W: u64 = 800;
const SCREEN_H: u64 = 600;

fn main() {
    let proj_matx =
        matrix::get_projection_matrix(SCREEN_H as f64 / SCREEN_W as f64, 90.0, 1000.0, 0.1);


    println!("Hello, world!");
}
