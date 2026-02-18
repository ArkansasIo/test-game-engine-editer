use crate::{actions, state, ui};
use crate::app::theme::apply_editor_theme;
pub mod theme;

pub struct EditorApp {
    pub project: state::ProjectState,
    pub action_registry: actions::registry::ActionRegistry,
    pub ui_state: ui::UiState,
}

impl EditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_editor_theme(&cc.egui_ctx);
        let project = state::ProjectState::default();
        let action_registry = actions::registry::ActionRegistry::new();
        let ui_state = ui::UiState::default();
        Self { project, action_registry, ui_state }
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
        // Floating Blueprint window (like UE Blueprint Editor)
        ui::draw_blueprint_window(ctx, self);
        // Execute queued actions (API bridge between UI and logic)
        actions::commands::drain_and_apply(&mut self.project, &mut self.ui_state, &self.action_registry);
        let _ = self.action_registry.action_count();
    }
}
