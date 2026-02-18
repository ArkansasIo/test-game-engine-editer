//! Transform utilities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub pos: [f32; 3],
    pub rot: [f32; 4], // quaternion
    pub scale: [f32; 3],
}

impl Transform {
    pub fn identity() -> Self {
        Self { pos: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0, 1.0], scale: [1.0, 1.0, 1.0] }
    }
}
