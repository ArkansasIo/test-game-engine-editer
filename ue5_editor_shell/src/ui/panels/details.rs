use crate::app::EditorApp;

pub fn draw(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Details").strong());
    });
    ui.separator();
    let selected_index = app.ui_state.selected_actor.unwrap_or(0);
    let actor = app.project.actors.get(selected_index);
    if let Some(actor) = actor {
        ui.label(egui::RichText::new(&actor.name).strong());
        ui.add_space(8.0);
        egui::CollapsingHeader::new("Transform").default_open(true).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Location");
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 0.0));
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 0.0));
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 0.0));
            });
            ui.horizontal(|ui| {
                ui.label("Rotation");
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 0.0));
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 0.0));
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 0.0));
            });
            ui.horizontal(|ui| {
                ui.label("Scale");
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 1.0));
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 1.0));
                ui.add_sized([68.0, 22.0], egui::DragValue::new(&mut 1.0));
            });
        });
        ui.add_space(8.0);
        egui::CollapsingHeader::new("Rendering").default_open(false).show(ui, |ui| {
            ui.checkbox(&mut true, "Visible");
            ui.add(egui::Slider::new(&mut 0.5, 0.0..=1.0).text("Roughness"));
        });
    } else {
        ui.label(egui::RichText::new("No selection").weak());
    }
}
