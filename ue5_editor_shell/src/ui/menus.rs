
use crate::{
    app::EditorApp,
    actions::commands::EditorCommand,
    editor_api::types::{ELayoutPreset, EViewportViewMode},
    state::EditorMode,
};

pub fn show_menu_bar(ctx: &egui::Context, app: &mut EditorApp) {
    egui::TopBottomPanel::top("menu_bar")
        .exact_height(28.0)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_menu(ui, app);
                edit_menu(ui, app);
                window_menu(ui, app);
                tools_menu(ui, app);
                build_menu(ui, app);
                select_menu(ui, app);
                actor_menu(ui, app);
                components_menu(ui, app);
                level_menu(ui, app);
                blueprints_menu(ui, app);
                materials_fx_menu(ui, app);
                cinematics_menu(ui, app);
                play_menu(ui, app);
                help_menu(ui, app);
            });
        });
}

// All menu functions below are direct ports of topbar.rs logic
// ...existing code...
