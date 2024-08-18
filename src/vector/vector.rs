use super::super::matrix::{rotate_x, Matrix};
use std::ops::{Add, Div, Mul, MulAssign, Sub};

#[derive(Debug, Copy, Clone)]

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let l = self.length();
        Vector3::new(self.x / l, self.y / l, self.z / l)
    }

    pub fn cross(&self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Vector3::new(x, y, z)
    }

    pub fn translation(&self, t: &Self) -> Self {
        *self + *t
    }

    pub fn to_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Vector3 {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        self * Vector3::new(rhs, rhs, rhs)
    }
}

impl Mul<Matrix<4, 4>> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix<4, 4>) -> Vector3 {
        rhs * self
    }
}

impl MulAssign<Matrix<4, 4>> for Vector3 {
    fn mul_assign(&mut self, rhs: Matrix<4, 4>) {
        let vector = rhs * *self;
        self.x = vector.x;
        self.y = vector.y;
        self.z = vector.z;
    }
}

impl Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f64) -> Vector3 {
        self * Matrix::new([
            [1.0, 0.0, 0.0, rhs],
            [0.0, 1.0, 0.0, rhs],
            [0.0, 0.0, 1.0, rhs],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Vector3 {
        self * Matrix::new([
            [1.0, 0.0, 0.0, rhs.x],
            [0.0, 1.0, 0.0, rhs.y],
            [0.0, 0.0, 1.0, rhs.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Vector3 {
        self.add(rhs * -1.0)
    }
}
impl Sub<f64> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f64) -> Vector3 {
        self.add(rhs * -1.0)
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Self) -> Vector3 {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        self * (1.0 / Vector3::new(rhs, rhs, rhs))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn from_angle(degs: f64) -> Vector2 {
        Vector2::new(degs.to_radians().cos(), degs.to_radians().sin() * -1.0)
    }

    pub fn add(&self, v: &Vector2) -> Vector2 {
        Vector2::new(&self.x + v.x, &self.y + v.y)
    }

    pub fn sub(&self, v: &Vector2) -> Vector2 {
        Vector2::new(&self.x - v.x, &self.y - v.y)
    }

    pub fn dot(&self, v: &Vector2) -> f64 {
        (&self.x * v.x) + (&self.y * v.y)
    }

    pub fn scale(&self, x: f64) -> Vector2 {
        Vector2::new(&self.x * x, &self.y * x)
    }

    pub fn length(&self) -> f64 {
        (&self.x * &self.x + &self.y * &self.y).sqrt()
    }

    pub fn norm(&self) -> Vector2 {
        let l = self.length();
        Vector2::new(&self.x / l, &self.y / l)
    }

    pub fn distance_to(&self, v: &Vector2) -> f64 {
        self.sub(v).length()
    }
}

impl Div<&Vector2> for Vector2 {
    type Output = Self;

    fn div(self, rhs: &Vector2) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
