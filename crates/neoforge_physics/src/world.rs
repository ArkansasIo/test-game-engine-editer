//! PhysicsWorld: manages simulation, bodies, colliders, constraints

use crate::rigid_body::{RigidBody, RigidBodyHandle, RigidBodyId};
use crate::collider::{Collider, ColliderHandle, ColliderId};
use crate::constraints::Constraint;
use crate::query::PhysicsQuery;

pub struct PhysicsWorld {
    // ...fields for bodies, colliders, constraints, etc.
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            // ...init fields
        }
    }
    pub fn step(&mut self, dt: f32) {
        // ...advance simulation
    }
    pub fn add_rigid_body(&mut self, body: RigidBodyHandle) -> RigidBodyId {
        // ...add body
        0
    }
    pub fn remove_rigid_body(&mut self, id: RigidBodyId) {
        // ...remove body
    }
    pub fn add_collider(&mut self, collider: ColliderHandle) -> ColliderId {
        // ...add collider
        0
    }
    pub fn remove_collider(&mut self, id: ColliderId) {
        // ...remove collider
    }
    pub fn add_constraint(&mut self, constraint: Constraint) {
        // ...add constraint
    }
    pub fn query(&self) -> &PhysicsQuery {
        // ...return query system
        unimplemented!()
    }
}
