use crate::app::EditorApp;

pub fn draw(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("World Outliner").strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_sized([160.0, 24.0], egui::TextEdit::singleline(&mut "".to_string()).hint_text("Search..."));
        });
    });
    ui.separator();
    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, actor) in app.project.actors.iter_mut().enumerate() {
            let selected = app.ui_state.selected_actor == Some(i);
            let label = format!("{}", actor.name);
            let resp = ui.selectable_label(selected, label);
            if resp.clicked() {
                app.ui_state.selected_actor = Some(i);
                // Add log entry to project log (field, not method)
                app.project.log_entries.push(format!("[Selection] Actor selected: {}", actor.name));
            }
        }
    });
}
