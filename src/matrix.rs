use crate::window;
use std::ops::Mul;

pub struct Matrix<const ROWS: usize, const COLS: usize> {
    value: [[f64; COLS]; ROWS],
}

pub fn projection_matrix() -> Matrix<4, 4> {
    let aspect_ratio: f64 = window::SCREEN_H as f64 / window::SCREEN_W as f64;
    let fov: f64 = 90.0;

    let fov_rad: f64 = 1.0 / (fov.to_radians().tan());

    let q = window::Z_FAR / (window::Z_FAR - window::Z_NEAR);

    return Matrix::new([
        [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
        [0.0, fov_rad, 0.0, 0.0],
        [0.0, 0.0, q, 1.0],
        [0.0, 0.0, -window::Z_NEAR * q, 0.0],
    ]);
}

pub fn rotate_x(angle: f64) -> Matrix<4, 4> {
    let cos = angle.cos();
    let sin = angle.sin();

    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, -sin, 0.0],
        [0.0, sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_y(angle: f64) -> Matrix<4, 4> {
    let cos = angle.cos();
    let sin = angle.sin();

    Matrix::new([
        [cos, 0.0, sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_z(angle: f64) -> Matrix<4, 4> {
    let cos = angle.cos();
    let sin = angle.sin();

    Matrix::new([
        [cos, -sin, 0.0, 0.0],
        [sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

impl Matrix<4, 4> {
    pub fn new(value: [[f64; 4]; 4]) -> Self {
        Matrix { value }
    }
}

impl std::ops::Deref for Matrix<4, 4> {
    type Target = [[f64; 4]; 4];

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for Matrix<4, 4> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl Mul for Matrix<4, 4> {
    type Output = Matrix<4, 4>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::new([[0.0; 4]; 4]);

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }

        res
    }
}

impl Mul<super::vector::vector::Vector3> for &Matrix<4, 4> {
    type Output = super::vector::vector::Vector3;

    fn mul(self, rhs: super::vector::vector::Vector3) -> Self::Output {
        let w = rhs.x * self[3][0] + rhs.y * self[3][1] + rhs.z * self[3][2] + self[3][3];

        let w = if w == 0.0 { 1.0 } else { w };

        super::vector::vector::Vector3::new(
            (rhs.x * self[0][0] + rhs.y * self[0][1] + rhs.z * self[0][2] + self[0][3]) / w,
            (rhs.x * self[1][0] + rhs.y * self[1][1] + rhs.z * self[1][2] + self[1][3]) / w,
            (rhs.x * self[2][0] + rhs.y * self[2][1] + rhs.z * self[2][2] + self[2][3]) / w,
        )
    }
}

impl Mul<super::vector::vector::Vector3> for Matrix<4, 4> {
    type Output = super::vector::vector::Vector3;

    fn mul(self, rhs: super::vector::vector::Vector3) -> Self::Output {
        let w = rhs.x * self[3][0] + rhs.y * self[3][1] + rhs.z * self[3][2] + self[3][3];

        let w = if w == 0.0 { 1.0 } else { w };

        super::vector::vector::Vector3::new(
            (rhs.x * self[0][0] + rhs.y * self[0][1] + rhs.z * self[0][2] + self[0][3]) / w,
            (rhs.x * self[1][0] + rhs.y * self[1][1] + rhs.z * self[1][2] + self[1][3]) / w,
            (rhs.x * self[2][0] + rhs.y * self[2][1] + rhs.z * self[2][2] + self[2][3]) / w,
        )
    }
}
