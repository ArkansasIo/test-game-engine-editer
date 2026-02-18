//! Global engine services and subsystems.

use crate::{audio::AudioSystem, effects::EffectSystem, resource::ResourceManager};

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub fixed_timestep_sec: f32,
    pub max_substeps: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            fixed_timestep_sec: 1.0 / 60.0,
            max_substeps: 4,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EngineSystems {
    pub resources: ResourceManager,
    pub audio: AudioSystem,
    pub effects: EffectSystem,
}
