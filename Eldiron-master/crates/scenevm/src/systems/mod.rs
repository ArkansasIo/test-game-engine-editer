pub mod build;
pub mod command;
pub mod georeference;
pub mod selection;
pub mod undo;

use crate::systems::{
    build::BuildSystem, command::CommandRouter, georeference::GeoReferenceSystem,
    selection::SelectionService, undo::UndoService,
};

/// High-level editor/runtime systems bundle.
///
/// This is intentionally lightweight and independent from rendering internals so it can be
/// plugged into tools, game runtime, and tests without GPU setup.
#[derive(Debug)]
pub struct EngineSystems {
    pub georeference: GeoReferenceSystem,
    pub commands: CommandRouter,
    pub selection: SelectionService,
    pub undo: UndoService,
    pub build: BuildSystem,
}

impl Default for EngineSystems {
    fn default() -> Self {
        let mut commands = CommandRouter::default();
        commands.seed_defaults();
        Self {
            georeference: GeoReferenceSystem::default(),
            commands,
            selection: SelectionService::default(),
            undo: UndoService::default(),
            build: BuildSystem::default(),
        }
    }
}
