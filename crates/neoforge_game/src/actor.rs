//! Actor/entity model.

use crate::component::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorId(pub u64);

#[derive(Debug, Clone)]
pub struct Actor {
    pub id: ActorId,
    pub name: String,
    pub active: bool,
    pub components: Vec<Component>,
}

impl Actor {
    pub fn new(id: ActorId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            active: true,
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
}
