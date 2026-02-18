//! Visual and gameplay effects system.

use std::collections::VecDeque;

use crate::{actor::ActorId, resource::AssetHandle};

#[derive(Debug, Clone)]
pub enum EffectCommand {
    SpawnVfx {
        effect_asset: AssetHandle,
        owner: Option<ActorId>,
        duration_sec: f32,
    },
    CameraShake {
        intensity: f32,
        duration_sec: f32,
    },
    PostProcessPulse {
        profile: String,
        duration_sec: f32,
    },
}

#[derive(Debug, Clone)]
pub struct ActiveEffect {
    pub id: u64,
    pub command: EffectCommand,
    pub remaining_sec: f32,
}

#[derive(Debug, Clone)]
pub struct EffectSystem {
    next_effect_id: u64,
    queue: VecDeque<EffectCommand>,
    pub active: Vec<ActiveEffect>,
}

impl Default for EffectSystem {
    fn default() -> Self {
        Self {
            next_effect_id: 1,
            queue: VecDeque::new(),
            active: Vec::new(),
        }
    }
}

impl EffectSystem {
    pub fn enqueue(&mut self, cmd: EffectCommand) {
        self.queue.push_back(cmd);
    }

    pub fn tick(&mut self, dt_sec: f32) {
        while let Some(cmd) = self.queue.pop_front() {
            let id = self.next_effect_id;
            self.next_effect_id += 1;
            let duration = match &cmd {
                EffectCommand::SpawnVfx { duration_sec, .. } => *duration_sec,
                EffectCommand::CameraShake { duration_sec, .. } => *duration_sec,
                EffectCommand::PostProcessPulse { duration_sec, .. } => *duration_sec,
            };
            self.active.push(ActiveEffect {
                id,
                command: cmd,
                remaining_sec: duration.max(0.0),
            });
        }

        for fx in &mut self.active {
            fx.remaining_sec = (fx.remaining_sec - dt_sec.max(0.0)).max(0.0);
        }
        self.active.retain(|fx| fx.remaining_sec > 0.0);
    }
}
