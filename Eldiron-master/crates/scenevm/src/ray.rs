//! Ray utilities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: [f32; 3],
    pub dir: [f32; 3],
}

impl Ray {
    pub fn new(origin: [f32; 3], dir: [f32; 3]) -> Self {
        Self { origin, dir }
    }
}
