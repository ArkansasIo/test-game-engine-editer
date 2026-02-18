use crate::{
    actions::commands::EditorCommand,
    app::EditorApp,
    editor_api::types::ELayoutPreset,
    state::EditorMode,
};

pub fn draw_toolbar(ctx: &egui::Context, app: &mut EditorApp) {
    egui::TopBottomPanel::top("toolbar")
        .exact_height(56.0)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.add_sized([80.0, 34.0], egui::Button::new("Save")).clicked() {
                    app.ui_state.enqueue(EditorCommand::SaveProject);
                }
                if ui
                    .add_sized([96.0, 34.0], egui::Button::new("Build"))
                    .clicked()
                {
                    app.ui_state.enqueue(EditorCommand::BuildLighting);
                }
                if ui
                    .add_sized([110.0, 34.0], egui::Button::new("Blueprints"))
                    .clicked()
                {
                    app.ui_state.enqueue(EditorCommand::CompileBlueprints);
                }
                if ui
                    .add_sized([96.0, 34.0], egui::Button::new("Add Actor"))
                    .clicked()
                {
                    app.ui_state.enqueue(EditorCommand::AddActor);
                }

                ui.separator();
                if app.project.is_playing {
                    if ui.add_sized([82.0, 34.0], egui::Button::new("Stop")).clicked() {
                        app.ui_state.enqueue(EditorCommand::StopPlay);
                    }
                } else if ui
                    .add_sized([82.0, 34.0], egui::Button::new("Play"))
                    .clicked()
                {
                    app.ui_state.enqueue(EditorCommand::PlayInEditor);
                }

                ui.separator();
                mode_button(ui, app, "Select", EditorMode::Select);
                mode_button(ui, app, "Landscape", EditorMode::Landscape);
                mode_button(ui, app, "Foliage", EditorMode::Foliage);
                mode_button(ui, app, "Modeling", EditorMode::Modeling);

                ui.separator();
                if ui
                    .add_sized(
                        [132.0, 34.0],
                        egui::Button::new(if app.ui_state.show_blueprint {
                            "Hide Blueprint"
                        } else {
                            "Show Blueprint"
                        }),
                    )
                    .clicked()
                {
                    app.ui_state.enqueue(EditorCommand::TogglePanelBlueprint);
                }

                if ui
                    .add_sized([96.0, 34.0], egui::Button::new("Settings"))
                    .clicked()
                {
                    app.ui_state.enqueue(EditorCommand::TogglePanelSettings);
                }
                ui.separator();
                if ui.button("Layout:Default").clicked() {
                    app.ui_state
                        .enqueue(EditorCommand::SetLayoutPreset(ELayoutPreset::Default));
                }
                if ui.button("Layout:Debug").clicked() {
                    app.ui_state
                        .enqueue(EditorCommand::SetLayoutPreset(ELayoutPreset::Debug));
                }
                if ui.button("SC:Connect").clicked() {
                    app.ui_state
                        .enqueue(EditorCommand::ConnectSourceControl("Git".to_owned()));
                }
            });
        });
}

fn mode_button(ui: &mut egui::Ui, app: &mut EditorApp, label: &str, mode: EditorMode) {
    let selected = app.project.mode == mode;
    if ui
        .add_sized([90.0, 34.0], egui::SelectableLabel::new(selected, label))
        .clicked()
    {
        app.ui_state.enqueue(EditorCommand::SetMode(mode));
    }
}
