use crate::app::EditorApp;

/// Draws the settings panel, including theme selection.
pub fn draw(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Settings");
    ui.separator();
    // Theme selection
    egui::ComboBox::from_label("Theme")
        .selected_text(app.ui_state.theme_name())
        .show_ui(ui, |ui| {
            if ui.selectable_label(app.ui_state.theme_name() == "Dark", "Dark").clicked() {
                app.ui_state.set_theme("Dark");
            }
            if ui.selectable_label(app.ui_state.theme_name() == "Light", "Light").clicked() {
                app.ui_state.set_theme("Light");
            }
            if ui.selectable_label(app.ui_state.theme_name() == "Classic", "Classic").clicked() {
                app.ui_state.set_theme("Classic");
            }
        });
    ui.separator();
    // ...other settings...
}
