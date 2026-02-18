//! Physics queries (raycast, overlap, sweep)

use crate::types::Vec3;
use crate::collider::ColliderId;

pub struct PhysicsQuery;

pub struct RaycastHit;

impl PhysicsQuery {
    pub fn raycast(&self, _origin: Vec3, _dir: Vec3, _max_dist: f32) -> Option<RaycastHit> {
        // ...raycast logic
        None
    }
    pub fn overlap_sphere(&self, _center: Vec3, _radius: f32) -> Vec<ColliderId> {
        // ...overlap logic
        vec![]
    }
}
