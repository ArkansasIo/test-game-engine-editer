//! Physics integration methods

use crate::rigid_body::RigidBody;

pub trait Integrator {
    fn integrate(&mut self, body: &mut RigidBody, dt: f32);
}

pub struct SemiImplicitEuler;

impl Integrator for SemiImplicitEuler {
    fn integrate(&mut self, body: &mut RigidBody, dt: f32) {
        // ...semi-implicit Euler integration
    }
}
