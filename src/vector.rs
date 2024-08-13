#[derive(Debug, Copy, Clone)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn translation(t: &Self) -> [[f64; 4]; 4] {
        self.mul_matrix([
            [1.0, 0.0, 0.0, t.x],
            [0.0, 1.0, 0.0, t.y],
            [0.0, 0.0, 1.0, t.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotate_x() -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        self.mul_matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotate_y() -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        self.mul_matrix([
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotate_z() -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        self.mul_matrix([
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn mul_matrix(&self, matrix: [[f64; 4]; 4]) -> Self {
        let x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0];
        let y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1];
        let z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2];

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
}
