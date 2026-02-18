use crate::state::ProjectState;

pub fn draw(ui: &mut egui::Ui, project: &mut ProjectState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Output Log").strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Clear").clicked() {
                project.log.clear();
            }
        });
    });
    ui.separator();
    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        for line in project.log.iter().rev() {
            ui.label(egui::RichText::new(line).text_style(egui::TextStyle::Monospace));
        }
    });
}
