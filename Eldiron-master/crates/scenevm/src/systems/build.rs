#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildTask {
    Lighting,
    Geometry,
    Navigation,
    Shaders,
    AssetCook,
    Package,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildState {
    Idle,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone)]
pub struct BuildSystem {
    pub state: BuildState,
    pub active_task: Option<BuildTask>,
    pub progress_01: f32,
    pub message: String,
}

impl Default for BuildSystem {
    fn default() -> Self {
        Self {
            state: BuildState::Idle,
            active_task: None,
            progress_01: 0.0,
            message: "Idle".to_owned(),
        }
    }
}

impl BuildSystem {
    pub fn begin(&mut self, task: BuildTask) {
        self.state = BuildState::Running;
        self.active_task = Some(task);
        self.progress_01 = 0.0;
        self.message = format!("Started {:?}", task);
    }

    pub fn update_progress(&mut self, progress_01: f32, message: impl Into<String>) {
        self.progress_01 = progress_01.clamp(0.0, 1.0);
        self.message = message.into();
    }

    pub fn finish_success(&mut self, message: impl Into<String>) {
        self.state = BuildState::Succeeded;
        self.progress_01 = 1.0;
        self.message = message.into();
    }

    pub fn finish_failed(&mut self, message: impl Into<String>) {
        self.state = BuildState::Failed;
        self.message = message.into();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
