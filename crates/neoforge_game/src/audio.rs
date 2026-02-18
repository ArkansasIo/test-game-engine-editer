//! Audio and sound playback system.

use std::collections::VecDeque;

use crate::resource::AssetHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCategory {
    Master,
    Music,
    Sfx,
    Voice,
    Ambience,
}

#[derive(Debug, Clone)]
pub struct SoundCommand {
    pub asset: AssetHandle,
    pub volume: f32,
    pub pitch: f32,
    pub looped: bool,
    pub category: AudioCategory,
}

#[derive(Debug, Clone)]
pub struct ActiveSound {
    pub id: u64,
    pub command: SoundCommand,
}

#[derive(Debug, Clone)]
pub struct AudioSystem {
    next_sound_id: u64,
    pub master_volume: f32,
    queue: VecDeque<SoundCommand>,
    pub active: Vec<ActiveSound>,
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self {
            next_sound_id: 1,
            master_volume: 1.0,
            queue: VecDeque::new(),
            active: Vec::new(),
        }
    }
}

impl AudioSystem {
    pub fn enqueue(&mut self, cmd: SoundCommand) {
        self.queue.push_back(cmd);
    }

    pub fn stop_all(&mut self) {
        self.active.clear();
        self.queue.clear();
    }

    pub fn tick(&mut self) {
        while let Some(cmd) = self.queue.pop_front() {
            let id = self.next_sound_id;
            self.next_sound_id += 1;
            self.active.push(ActiveSound { id, command: cmd });
        }
    }
}
