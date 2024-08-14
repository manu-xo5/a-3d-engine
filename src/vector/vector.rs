use super::super::window;

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

    pub fn scale(&self, mag: f64) -> Self {
        Vector3::new(self.x * mag, self.y * mag, self.z * mag)
    }

    pub fn translation(&self, t: &Self) -> Self {
        Vector3::new(self.x + t.x, self.y + t.y, self.z + t.z)
        // self.mul_matrix([
        //     [1.0, 0.0, 0.0, t.x],
        //     [0.0, 1.0, 0.0, t.y],
        //     [0.0, 0.0, 1.0, t.z],
        //     [0.0, 0.0, 0.0, 1.0],
        // ])
    }

    pub fn rotate_x(&self, angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        self.mul_matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_y(&self, angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        self.mul_matrix([
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_z(&self, angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        self.mul_matrix([
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn project(&self) -> Self {
        let zf = 1.0; //if self.z == 0.0 { 1.0 } else { 1.0 / self.z };
        let xf = window::SCREEN_W as f64 / 2.0;
        let yf = window::SCREEN_H as f64 / 2.0;

        Vector3::new((self.x + 1.0) * xf, (self.y + 1.0) * yf, self.z)
    }

    pub fn mul_matrix(&self, matrix: [[f64; 4]; 4]) -> Self {
        let x =
            self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0] + matrix[3][0];
        let y =
            self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1] + matrix[3][1];
        let z =
            self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2] + matrix[3][2];

        let w =
            self.x * matrix[0][3] + self.y * matrix[1][3] + self.z * matrix[2][3] + matrix[3][3];

        if w == 0.0 {
            Vector3 { x, y, z }
        } else {
            Vector3 {
                x: x / w,
                y: y / w,
                z: z / w,
            }
        }
    }

    pub fn to_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
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
