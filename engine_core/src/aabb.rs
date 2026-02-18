// Axis-Aligned Bounding Box (AABB)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: super::vec3::Vec3,
    pub max: super::vec3::Vec3,
}

impl Aabb {
    pub fn new(min: super::vec3::Vec3, max: super::vec3::Vec3) -> Self {
        Self { min, max }
    }
    pub fn contains(&self, point: super::vec3::Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
}
