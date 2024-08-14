use crate::window;

pub type Matrix<const ROWS: usize, const COLS: usize> = [[f64; COLS]; ROWS];

pub fn projection_matrix() -> Matrix<4, 4> {
    let aspect_ratio: f64 = window::SCREEN_H as f64 / window::SCREEN_W as f64;
    let fov: f64 = 90.0;

    let fov_rad: f64 = 1.0 / (fov.to_radians().tan());

    let q = window::Z_FAR / (window::Z_FAR - window::Z_NEAR);

    return [
        [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
        [0.0, fov_rad, 0.0, 0.0],
        [0.0, 0.0, q, 1.0],
        [0.0, 0.0, -window::Z_NEAR * q, 0.0],
    ];
}
