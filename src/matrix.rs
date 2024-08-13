pub type Matrix<const ROWS: usize, const COLS: usize> = [[f64; COLS]; ROWS];

pub fn get_projection_matrix(aspect_ratio: f64, fov: f64, z_far: f64, z_near: f64) -> Matrix<4, 4> {
    let fov_rad = 1.0 / (fov.to_radians().tan());

    let q = z_far / (z_far - z_near);

    return [
        [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
        [0.0, fov_rad, 0.0, 0.0],
        [0.0, 0.0, q, 1.0],
        [0.0, 0.0, -z_near * q, 0.0],
    ];
}
