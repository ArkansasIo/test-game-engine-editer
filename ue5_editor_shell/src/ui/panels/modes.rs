use crate::state::ProjectState;
use crate::ui::UiState;

pub fn draw(ui: &mut egui::Ui, project: &mut ProjectState, ui_state: &mut UiState) {
    ui.label(egui::RichText::new("Modes").strong());
    ui.separator();
    // UE-like mode buttons
    ui.vertical(|ui| {
        mode_button(ui, project, "Place Actors");
        mode_button(ui, project, "Landscape");
        mode_button(ui, project, "Foliage");
        mode_button(ui, project, "Modeling");
    });
    ui.add_space(12.0);
    ui.separator();
    ui.label(egui::RichText::new("Quick Tools").strong());
    ui.add_space(6.0);
    ui.horizontal_wrapped(|ui| {
        quick(ui, project, "➕", "Add");
        quick(ui, project, "🧲", "Snap");
        quick(ui, project, "🧱", "Grid");
        quick(ui, project, "🖌️", "Paint");
    });
    ui.add_space(12.0);
    ui.separator();
    ui.label(egui::RichText::new("Hotkeys").strong());
    ui.label(egui::RichText::new("Ctrl+P : Toggle Search (stub)").weak());
        // Removed search_open (no such field)
        ui.add_space(8.0);
        ui.group(|ui| {
            ui.label(egui::RichText::new("Command Search (stub)").strong());
            ui.add_sized([ui.available_width(), 24.0], egui::TextEdit::singleline(&mut "".to_string()).hint_text("Type a command..."));
        });
}

fn mode_button(ui: &mut egui::Ui, project: &mut ProjectState, name: &str) {
    if ui.add_sized([ui.available_width(), 28.0], egui::Button::new(name)).clicked() {
           project.log(format!("[Mode] {}", name));
            project.log(format!("[Mode] {}", name));
    }
}

fn quick(ui: &mut egui::Ui, project: &mut ProjectState, icon: &str, name: &str) {
    if ui.add_sized([72.0, 28.0], egui::Button::new(format!("{icon} {name}"))).clicked() {
           project.log(format!("[Tool] {}", name));
            project.log(format!("[Tool] {}", name));
    }
}
