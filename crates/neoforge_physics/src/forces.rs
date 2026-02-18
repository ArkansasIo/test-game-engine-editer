//! Force generators (gravity, custom)

use crate::rigid_body::RigidBody;
use crate::types::Vec3;

pub trait ForceGenerator {
    fn apply(&self, body: &mut RigidBody, dt: f32);
}

pub struct Gravity {
    pub acceleration: Vec3,
}

impl ForceGenerator for Gravity {
    fn apply(&self, body: &mut RigidBody, _dt: f32) {
        // ...apply gravity
    }
}
