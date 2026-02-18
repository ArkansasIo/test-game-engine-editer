// Plane
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: super::vec3::Vec3,
    pub d: f32,
}

impl Plane {
    pub fn new(normal: super::vec3::Vec3, d: f32) -> Self {
        Self { normal, d }
    }
}
