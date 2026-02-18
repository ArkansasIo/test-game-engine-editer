#![allow(dead_code)]

use crate::editor_api::types::{
    EBuildTask, ELayoutPreset, EViewportViewMode, FBuildProgress, FSourceControlState,
};

pub trait IEditorApp {
    fn startup(&mut self);
    fn shutdown(&mut self);
}

pub trait ILayoutService {
    fn set_preset(&mut self, preset: ELayoutPreset);
    fn active_preset(&self) -> ELayoutPreset;
}

pub trait IBuildService {
    fn enqueue_build(&mut self, task: EBuildTask);
    fn progress(&self) -> &FBuildProgress;
}

pub trait ISourceControlService {
    fn connect(&mut self, provider_name: &str);
    fn disconnect(&mut self);
    fn state(&self) -> &FSourceControlState;
}

pub trait IViewportService {
    fn set_view_mode(&mut self, mode: EViewportViewMode);
    fn view_mode(&self) -> EViewportViewMode;
}

#[derive(Default)]
pub struct BuildService {
    progress: FBuildProgress,
}

impl IBuildService for BuildService {
    fn enqueue_build(&mut self, task: EBuildTask) {
        self.progress.active = Some(task);
        self.progress.total_steps = 100;
        self.progress.completed_steps = 100;
    }

    fn progress(&self) -> &FBuildProgress {
        &self.progress
    }
}

#[derive(Default)]
pub struct SourceControlService {
    state: FSourceControlState,
}

impl ISourceControlService for SourceControlService {
    fn connect(&mut self, provider_name: &str) {
        self.state.connected = true;
        self.state.provider_name = provider_name.to_owned();
        self.state.checked_out_files = 0;
    }

    fn disconnect(&mut self) {
        self.state = FSourceControlState::default();
    }

    fn state(&self) -> &FSourceControlState {
        &self.state
    }
}

pub struct LayoutService {
    preset: ELayoutPreset,
}

impl Default for LayoutService {
    fn default() -> Self {
        Self {
            preset: ELayoutPreset::Default,
        }
    }
}

impl ILayoutService for LayoutService {
    fn set_preset(&mut self, preset: ELayoutPreset) {
        self.preset = preset;
    }

    fn active_preset(&self) -> ELayoutPreset {
        self.preset
    }
}

pub struct ViewportService {
    mode: EViewportViewMode,
}

impl Default for ViewportService {
    fn default() -> Self {
        Self {
            mode: EViewportViewMode::Lit,
        }
    }
}

impl IViewportService for ViewportService {
    fn set_view_mode(&mut self, mode: EViewportViewMode) {
        self.mode = mode;
    }

    fn view_mode(&self) -> EViewportViewMode {
        self.mode
    }
}
