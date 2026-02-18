//! 2x2 matrix math
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    pub m: [[f32; 2]; 2],
}

impl Mat2 {
    pub fn identity() -> Self {
        Self { m: [[1.0, 0.0], [0.0, 1.0]] }
    }
    pub fn mul_vec2(&self, v: [f32; 2]) -> [f32; 2] {
        [
            self.m[0][0] * v[0] + self.m[0][1] * v[1],
            self.m[1][0] * v[0] + self.m[1][1] * v[1],
        ]
    }
}
