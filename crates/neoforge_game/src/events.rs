//! Events and event bus for gameplay/runtime/editor systems.

use std::collections::VecDeque;

use crate::{
    actor::ActorId,
    audio::SoundCommand,
    effects::EffectCommand,
    level::LevelId,
    resource::AssetHandle,
};

#[derive(Debug, Clone)]
pub enum Event {
    LevelLoaded(LevelId),
    LevelUnloaded(LevelId),
    ActorSpawned(ActorId),
    ActorDestroyed(ActorId),
    AssetLoaded(AssetHandle),
    AssetUnloaded(AssetHandle),
    PlaySound(SoundCommand),
    TriggerEffect(EffectCommand),
    GameplayTag(String),
    Log(String),
}

#[derive(Debug, Default, Clone)]
pub struct EventBus {
    queue: VecDeque<Event>,
}

impl EventBus {
    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }

    pub fn drain(&mut self) -> Vec<Event> {
        self.queue.drain(..).collect()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
