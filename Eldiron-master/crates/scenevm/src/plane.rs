//! Plane utilities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: [f32; 3],
    pub d: f32,
}

impl Plane {
    pub fn new(normal: [f32; 3], d: f32) -> Self {
        Self { normal, d }
    }
}
