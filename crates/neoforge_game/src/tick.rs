//! Main loop timing and fixed/update tick helpers.

#[derive(Debug, Clone, Copy)]
pub struct TickConfig {
    pub fixed_delta_sec: f32,
    pub max_substeps: u32,
}

impl Default for TickConfig {
    fn default() -> Self {
        Self {
            fixed_delta_sec: 1.0 / 60.0,
            max_substeps: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TickClock {
    pub config: TickConfig,
    accumulator: f32,
    pub frame_index: u64,
}

impl Default for TickClock {
    fn default() -> Self {
        Self {
            config: TickConfig::default(),
            accumulator: 0.0,
            frame_index: 0,
        }
    }
}

impl TickClock {
    pub fn with_config(config: TickConfig) -> Self {
        Self {
            config,
            accumulator: 0.0,
            frame_index: 0,
        }
    }

    pub fn begin_frame(&mut self, dt_sec: f32) {
        self.accumulator += dt_sec.max(0.0);
        self.frame_index += 1;
    }

    pub fn consume_fixed_steps(&mut self) -> u32 {
        let mut steps = 0;
        while self.accumulator >= self.config.fixed_delta_sec && steps < self.config.max_substeps {
            self.accumulator -= self.config.fixed_delta_sec;
            steps += 1;
        }
        steps
    }

    pub fn alpha(&self) -> f32 {
        (self.accumulator / self.config.fixed_delta_sec).clamp(0.0, 1.0)
    }
}
