use crate::{actions, editor_api, state, ui};
use crate::app::theme::apply_editor_theme;
use crate::editor_api::services::IEditorApp;
pub mod theme;

pub struct EditorApp {
    pub project: state::ProjectState,
    pub action_registry: actions::registry::ActionRegistry,
    pub ui_state: ui::UiState,
    pub app_core: editor_api::app_core::AppCore,
    pub modules: Vec<editor_api::module_graph::EditorModule>,
}

impl EditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_editor_theme(&cc.egui_ctx);
        let project = state::ProjectState::default();
        let action_registry = actions::registry::ActionRegistry::new();
        let ui_state = ui::UiState::default();
        let mut app_core = editor_api::app_core::AppCore::default();
        app_core.startup();
        let modules = editor_api::module_graph::default_module_graph();
        Self {
            project,
            action_registry,
            ui_state,
            app_core,
            modules,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Global keyboard shortcuts
        self.ui_state.hotkeys.update(ctx, &mut self.ui_state.pending_commands);
        // Top menu + toolbar (UE-like)
        ui::topbar::draw_top_menubar(ctx, self);
        ui::toolbar::draw_toolbar(ctx, self);
        // Main docked layout
        ui::draw_docked_layout(ctx, self);
        // Global status bar
        ui::status_bar::draw_status_bar(ctx, self);
        // Floating Blueprint window (like UE Blueprint Editor)
        ui::draw_blueprint_window(ctx, self);
        // Execute queued actions (API bridge between UI and logic)
        actions::commands::drain_and_apply(self);
        let _ = self.action_registry.action_count();
    }
}
