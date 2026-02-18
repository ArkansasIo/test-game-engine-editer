//! World simulation context connecting levels, actors, systems, and events.

use std::collections::HashMap;

use crate::{
    actor::{Actor, ActorId},
    events::{Event, EventBus},
    level::{Level, LevelId},
    subsystem::{EngineConfig, EngineSystems},
    tick::{TickClock, TickConfig},
};

#[derive(Debug, Clone)]
pub struct World {
    pub config: EngineConfig,
    pub systems: EngineSystems,
    pub events: EventBus,
    pub levels: HashMap<LevelId, Level>,
    pub actors: HashMap<ActorId, Actor>,
    pub current_level: Option<LevelId>,
    next_level_id: u64,
    next_actor_id: u64,
    pub clock: TickClock,
}

impl Default for World {
    fn default() -> Self {
        let config = EngineConfig::default();
        Self {
            config: config.clone(),
            systems: EngineSystems::default(),
            events: EventBus::default(),
            levels: HashMap::new(),
            actors: HashMap::new(),
            current_level: None,
            next_level_id: 1,
            next_actor_id: 1,
            clock: TickClock::with_config(TickConfig {
                fixed_delta_sec: config.fixed_timestep_sec,
                max_substeps: config.max_substeps,
            }),
        }
    }
}

impl World {
    pub fn create_level(&mut self, name: impl Into<String>) -> LevelId {
        let id = LevelId(self.next_level_id);
        self.next_level_id += 1;
        self.levels.insert(id, Level::new(id, name));
        self.current_level = Some(id);
        self.events.push(Event::LevelLoaded(id));
        id
    }

    pub fn spawn_actor(&mut self, name: impl Into<String>) -> ActorId {
        let id = ActorId(self.next_actor_id);
        self.next_actor_id += 1;
        self.actors.insert(id, Actor::new(id, name));
        if let Some(level_id) = self.current_level {
            if let Some(level) = self.levels.get_mut(&level_id) {
                level.actors.push(id);
            }
        }
        self.events.push(Event::ActorSpawned(id));
        id
    }

    pub fn destroy_actor(&mut self, id: ActorId) {
        if self.actors.remove(&id).is_some() {
            for level in self.levels.values_mut() {
                level.actors.retain(|a| *a != id);
            }
            self.events.push(Event::ActorDestroyed(id));
        }
    }

    pub fn tick(&mut self, dt_sec: f32) {
        self.clock.begin_frame(dt_sec);
        let steps = self.clock.consume_fixed_steps();
        for _ in 0..steps {
            self.fixed_update(self.config.fixed_timestep_sec);
        }
        self.update(dt_sec);
    }

    fn fixed_update(&mut self, dt_sec: f32) {
        self.systems.effects.tick(dt_sec);
    }

    fn update(&mut self, _dt_sec: f32) {
        self.systems.audio.tick();
    }
}
