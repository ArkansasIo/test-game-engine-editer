use crate::editor_api::services::{
    BuildService, IBuildService, IEditorApp, ILayoutService, ISourceControlService, IViewportService,
    LayoutService, SourceControlService, ViewportService,
};
use crate::editor_api::types::{EBuildTask, ELayoutPreset, EViewportViewMode};

pub struct AppCore {
    started: bool,
    pub layout_service: LayoutService,
    pub build_service: BuildService,
    pub source_control_service: SourceControlService,
    pub viewport_service: ViewportService,
}

impl Default for AppCore {
    fn default() -> Self {
        Self {
            started: false,
            layout_service: LayoutService::default(),
            build_service: BuildService::default(),
            source_control_service: SourceControlService::default(),
            viewport_service: ViewportService::default(),
        }
    }
}

impl IEditorApp for AppCore {
    fn startup(&mut self) {
        self.started = true;
    }

    fn shutdown(&mut self) {
        self.started = false;
    }
}

impl AppCore {
    pub fn apply_layout_preset(&mut self, preset: ELayoutPreset) {
        self.layout_service.set_preset(preset);
    }

    pub fn active_layout_preset(&self) -> ELayoutPreset {
        self.layout_service.active_preset()
    }

    pub fn enqueue_build(&mut self, task: EBuildTask) {
        self.build_service.enqueue_build(task);
    }

    pub fn connect_source_control(&mut self, provider_name: &str) {
        self.source_control_service.connect(provider_name);
    }

    pub fn source_control_state(&self) -> &crate::editor_api::types::FSourceControlState {
        self.source_control_service.state()
    }

    pub fn disconnect_source_control(&mut self) {
        self.source_control_service.disconnect();
    }

    pub fn build_progress(&self) -> &crate::editor_api::types::FBuildProgress {
        self.build_service.progress()
    }

    pub fn set_view_mode(&mut self, mode: EViewportViewMode) {
        self.viewport_service.set_view_mode(mode);
    }

    pub fn view_mode(&self) -> EViewportViewMode {
        self.viewport_service.view_mode()
    }
}
