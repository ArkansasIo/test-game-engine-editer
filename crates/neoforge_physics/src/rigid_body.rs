//! RigidBody: dynamic, static, kinematic bodies

use crate::types::Vec3;
use std::sync::Arc;

pub enum BodyType { Static, Dynamic, Kinematic }

pub struct RigidBody {
    pub body_type: BodyType,
    pub mass: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    // ...other properties
}

impl RigidBody {
    pub fn new(body_type: BodyType, mass: f32, position: Vec3) -> Self {
        Self {
            body_type,
            mass,
            position,
            velocity: Vec3::zero(),
            angular_velocity: Vec3::zero(),
        }
    }
    pub fn apply_force(&mut self, force: Vec3) {
        // ...apply force
    }
    pub fn apply_impulse(&mut self, impulse: Vec3) {
        // ...apply impulse
    }
    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }
    pub fn set_angular_velocity(&mut self, ang_vel: Vec3) {
        self.angular_velocity = ang_vel;
    }
}

pub type RigidBodyHandle = Arc<RigidBody>;
pub type RigidBodyId = u64;
