use crate::app::EditorApp;

pub fn draw_status_bar(ctx: &egui::Context, app: &mut EditorApp) {
    egui::TopBottomPanel::bottom("status_bar")
        .exact_height(24.0)
        .show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                let sc = app.app_core.source_control_state();
                ui.label(format!(
                    "Source Control: {}",
                    if sc.connected {
                        sc.provider_name.as_str()
                    } else {
                        "Disconnected"
                    }
                ));
                ui.separator();
                ui.label(format!("Shaders: {}", app.project.stats.shader_jobs));
                ui.separator();
                ui.label(format!("FPS: {}", app.project.stats.fps));
                ui.separator();
                ui.label(format!("Layout: {:?}", app.app_core.active_layout_preset()));
                ui.separator();
                ui.label(format!("View: {:?}", app.app_core.view_mode()));
                ui.separator();
                ui.label(format!(
                    "Scene/Level: {}/{}",
                    app.project
                        .scenes
                        .get(app.project.active_scene)
                        .map(|s| s.name.as_str())
                        .unwrap_or("None"),
                    app.project
                        .levels
                        .get(app.project.active_level)
                        .map(|l| l.name.as_str())
                        .unwrap_or("None")
                ));
                ui.separator();
                ui.label(format!("Resources: {}", app.project.resources.len()));
                ui.separator();
                if let Some(task) = app.app_core.build_progress().active {
                    ui.label(format!("Build: {:?} complete", task));
                } else {
                    ui.label("Build: Idle");
                }
                ui.separator();
                ui.label(&app.ui_state.status_text);
            });
        });
}
