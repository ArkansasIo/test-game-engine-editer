//! Narrow phase collision detection

use crate::collider::Collider;
use crate::collision::contact::Contact;

pub struct NarrowPhase;

impl NarrowPhase {
    pub fn test_collision(a: &Collider, b: &Collider) -> Option<Contact> {
        // ...precise collision test
        None
    }
}
