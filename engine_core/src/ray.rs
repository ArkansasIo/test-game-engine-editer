// Ray
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: super::vec3::Vec3,
    pub direction: super::vec3::Vec3,
}

impl Ray {
    pub fn new(origin: super::vec3::Vec3, direction: super::vec3::Vec3) -> Self {
        Self { origin, direction }
    }
}
