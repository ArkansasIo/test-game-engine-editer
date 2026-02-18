//! Broad phase collision detection

use crate::collider::{Collider, ColliderId};

pub struct BroadPhase;

impl BroadPhase {
    pub fn update(&mut self, _colliders: &[Collider]) {
        // ...update broad phase
    }
    pub fn potential_pairs(&self) -> Vec<(ColliderId, ColliderId)> {
        // ...return potential pairs
        vec![]
    }
}
