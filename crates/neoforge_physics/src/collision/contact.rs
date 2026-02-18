//! Contact data and resolution

use crate::rigid_body::RigidBody;

pub struct Contact;

pub struct ContactManifold;

impl ContactManifold {
    pub fn resolve(&mut self, _bodies: &mut [RigidBody]) {
        // ...resolve contacts
    }
}
