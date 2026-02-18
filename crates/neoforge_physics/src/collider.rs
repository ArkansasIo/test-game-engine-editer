//! Collider: shapes and properties

use crate::types::Vec3;
use std::sync::Arc;

pub enum ColliderShape {
    Sphere(f32),
    Box(Vec3),
    Capsule { half_height: f32, radius: f32 },
    Mesh, // Placeholder for mesh data
}

pub struct Collider {
    pub shape: ColliderShape,
    pub is_trigger: bool,
    pub friction: f32,
    pub restitution: f32,
}

impl Collider {
    pub fn new(shape: ColliderShape, is_trigger: bool) -> Self {
        Self {
            shape,
            is_trigger,
            friction: 0.5,
            restitution: 0.5,
        }
    }
    pub fn set_friction(&mut self, friction: f32) {
        self.friction = friction;
    }
    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }
}

pub type ColliderHandle = Arc<Collider>;
pub type ColliderId = u64;
