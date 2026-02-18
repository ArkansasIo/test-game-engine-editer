//! Collision utilities (stub)
pub fn aabb_intersect(a_min: [f32; 3], a_max: [f32; 3], b_min: [f32; 3], b_max: [f32; 3]) -> bool {
    (a_min[0] <= b_max[0] && a_max[0] >= b_min[0]) &&
    (a_min[1] <= b_max[1] && a_max[1] >= b_min[1]) &&
    (a_min[2] <= b_max[2] && a_max[2] >= b_min[2])
}
