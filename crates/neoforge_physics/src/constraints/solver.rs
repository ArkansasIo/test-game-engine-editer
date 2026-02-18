//! Constraint solver logic

use crate::constraints::Constraint;
use crate::rigid_body::RigidBody;

pub struct ConstraintSolver;

impl ConstraintSolver {
    pub fn solve(&mut self, _constraints: &[Constraint], _bodies: &mut [RigidBody], _dt: f32) {
        // ...solve constraints
    }
}
