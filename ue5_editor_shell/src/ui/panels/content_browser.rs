use crate::state::ProjectState;

pub fn draw(ui: &mut egui::Ui, project: &mut ProjectState) {
    ui.horizontal(|ui| {
        ui.add(egui::Button::new("Content ▾"));
        ui.separator();
        ui.label(egui::RichText::new("StarterContent / Props").weak());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_sized([220.0, 24.0], egui::TextEdit::singleline(&mut "".to_string()).hint_text("Search..."));
        });
    });
    ui.add_space(8.0);
    // Asset tiles (mock)
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            for a in &project.assets {
                ui.group(|ui| {
                    ui.set_min_size(egui::vec2(140.0, 96.0));
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label("🧊");
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new(a).strong());
                    });
                });
                ui.add_space(8.0);
            }
        });
    });
}
