//! Component data for actors/entities.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentKind {
    Transform,
    Render,
    Physics,
    AudioEmitter,
    Light,
    Script,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub kind: ComponentKind,
    pub enabled: bool,
    pub name: String,
}

impl Component {
    pub fn new(kind: ComponentKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            enabled: true,
            name: name.into(),
        }
    }
}
