use crate::state::ProjectState;

pub fn draw(ui: &mut egui::Ui, project: &mut ProjectState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("World Outliner").strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_sized([160.0, 24.0], egui::TextEdit::singleline(&mut "".to_string()).hint_text("Search..."));
        });
    });
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        for actor in &mut project.actors {
            let selected = actor.selected;
            let label = format!("{} ({})", actor.name, actor.kind);
            let resp = ui.selectable_label(selected, label);
            if resp.clicked() {
                for a in &mut project.actors {
                    a.selected = false;
                }
                actor.selected = true;
                project.log.push(format!("[Selection] Actor selected: {}", actor.name));
            }
        }
    });
}
