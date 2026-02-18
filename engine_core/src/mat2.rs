// 2x2 Matrix
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    pub m: [[f32; 2]; 2],
}

impl Mat2 {
    pub fn identity() -> Self {
        Self { m: [[1.0, 0.0], [0.0, 1.0]] }
    }
}
