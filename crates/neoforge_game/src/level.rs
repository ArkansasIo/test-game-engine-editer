//! Level/map management.

use crate::actor::ActorId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LevelId(pub u64);

#[derive(Debug, Clone)]
pub struct Level {
    pub id: LevelId,
    pub name: String,
    pub actors: Vec<ActorId>,
    pub loaded: bool,
}

impl Level {
    pub fn new(id: LevelId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            actors: Vec::new(),
            loaded: true,
        }
    }
}
