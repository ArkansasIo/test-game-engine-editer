use crate::{actions::commands::EditorCommand, app::EditorApp, state::EditorMode};

pub fn draw_top_menubar(ctx: &egui::Context, app: &mut EditorApp) {
    egui::TopBottomPanel::top("top_menubar")
        .exact_height(28.0)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        app.ui_state.enqueue(EditorCommand::NewProject);
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        app.ui_state.enqueue(EditorCommand::SaveProject);
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        app.ui_state.enqueue(EditorCommand::Undo);
                        ui.close_menu();
                    }
                    if ui.button("Redo").clicked() {
                        app.ui_state.enqueue(EditorCommand::Redo);
                        ui.close_menu();
                    }
                    if ui.button("Reset Layout").clicked() {
                        app.ui_state.enqueue(EditorCommand::ResetLayout);
                        ui.close_menu();
                    }
                });

                ui.menu_button("Window", |ui| {
                    if ui.button("Blueprint").clicked() {
                        app.ui_state.enqueue(EditorCommand::TogglePanelBlueprint);
                    }
                    if ui.button("Outliner").clicked() {
                        app.ui_state.enqueue(EditorCommand::TogglePanelOutliner);
                    }
                    if ui.button("Details").clicked() {
                        app.ui_state.enqueue(EditorCommand::TogglePanelDetails);
                    }
                    if ui.button("Content Browser").clicked() {
                        app.ui_state.enqueue(EditorCommand::TogglePanelContent);
                    }
                    if ui.button("Output Log").clicked() {
                        app.ui_state.enqueue(EditorCommand::TogglePanelOutput);
                    }
                    if ui.button("Project Settings").clicked() {
                        app.ui_state.enqueue(EditorCommand::TogglePanelSettings);
                    }
                });

                ui.menu_button("Build", |ui| {
                    if ui.button("Build Lighting").clicked() {
                        app.ui_state.enqueue(EditorCommand::BuildLighting);
                        ui.close_menu();
                    }
                    if ui.button("Compile Blueprints").clicked() {
                        app.ui_state.enqueue(EditorCommand::CompileBlueprints);
                        ui.close_menu();
                    }
                });

                ui.menu_button("Select", |ui| {
                    if ui.button("Select Mode").clicked() {
                        app.ui_state.enqueue(EditorCommand::SetMode(EditorMode::Select));
                        ui.close_menu();
                    }
                    if ui.button("Landscape Mode").clicked() {
                        app.ui_state
                            .enqueue(EditorCommand::SetMode(EditorMode::Landscape));
                        ui.close_menu();
                    }
                    if ui.button("Foliage Mode").clicked() {
                        app.ui_state
                            .enqueue(EditorCommand::SetMode(EditorMode::Foliage));
                        ui.close_menu();
                    }
                    if ui.button("Modeling Mode").clicked() {
                        app.ui_state
                            .enqueue(EditorCommand::SetMode(EditorMode::Modeling));
                        ui.close_menu();
                    }
                });

                ui.menu_button("Actor", |ui| {
                    if ui.button("Add Actor").clicked() {
                        app.ui_state.enqueue(EditorCommand::AddActor);
                        ui.close_menu();
                    }
                    if ui.button("Duplicate Actor").clicked() {
                        app.ui_state.enqueue(EditorCommand::DuplicateActor);
                        ui.close_menu();
                    }
                    if ui.button("Delete Actor").clicked() {
                        app.ui_state.enqueue(EditorCommand::DeleteActor);
                        ui.close_menu();
                    }
                });

                ui.menu_button("Blueprints", |ui| {
                    if ui.button("Compile").clicked() {
                        app.ui_state.enqueue(EditorCommand::CompileBlueprints);
                        ui.close_menu();
                    }
                    if ui.button("Open Blueprint Window").clicked() {
                        app.ui_state.enqueue(EditorCommand::TogglePanelBlueprint);
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    ui.label("Hotkeys:");
                    for line in app.action_registry.action_hints() {
                        ui.label(line);
                    }
                    ui.label("Compile Blueprints [Ctrl+B]");
                    ui.label("Delete Actor [Delete]");
                });

                ui.separator();
                ui.label(
                    egui::RichText::new(format!(
                        "{} - {}{}",
                        app.project.project_name,
                        app.project.current_map,
                        if app.project.dirty { " *" } else { "" }
                    ))
                    .weak(),
                );
            });
        });
}
