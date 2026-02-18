use crate::{
    app::EditorApp,
    actions::commands::EditorCommand,
    editor_api::types::{ELayoutPreset, EViewportViewMode},
    state::EditorMode,
};

pub fn show_submenu(ui: &mut egui::Ui, app: &mut EditorApp, menu: &str) {
    match menu {
        "File" => file_submenu(ui, app),
        "Edit" => edit_submenu(ui, app),
        "Window" => window_submenu(ui, app),
        "Tools" => tools_submenu(ui, app),
        "Build" => build_submenu(ui, app),
        "Select" => select_submenu(ui, app),
        "Actor" => actor_submenu(ui, app),
        "Components" => components_submenu(ui, app),
        "Level" => level_submenu(ui, app),
        "Blueprints" => blueprints_submenu(ui, app),
        "Materials/FX" => materials_fx_submenu(ui, app),
        "Cinematics" => cinematics_submenu(ui, app),
        "Play" => play_submenu(ui, app),
        "Help" => help_submenu(ui, app),
        _ => {},
    }
}

fn file_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("New Project").clicked() {
        app.project.log.push("[Submenu] New Project".into());
        ui.close_menu();
    }
    if ui.button("Open Project...").clicked() {
        app.project.log.push("[Submenu] Open Project".into());
        ui.close_menu();
    }
    if ui.button("Save").clicked() {
        app.project.log.push("[Submenu] Save".into());
        ui.close_menu();
    }
    ui.separator();
    if ui.button("Exit").clicked() {
        app.project.log.push("[Submenu] Exit".into());
        ui.close_menu();
    }
}

fn edit_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Undo").clicked() {
        app.project.log.push("[Submenu] Undo".into());
        ui.close_menu();
    }
    if ui.button("Redo").clicked() {
        app.project.log.push("[Submenu] Redo".into());
        ui.close_menu();
    }
    ui.separator();
    if ui.button("Cut").clicked() {
        app.project.log.push("[Submenu] Cut".into());
        ui.close_menu();
    }
    if ui.button("Copy").clicked() {
        app.project.log.push("[Submenu] Copy".into());
        ui.close_menu();
    }
    if ui.button("Paste").clicked() {
        app.project.log.push("[Submenu] Paste".into());
        ui.close_menu();
    }
}

fn window_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Reset Layout").clicked() {
        app.project.log.push("[Submenu] Reset Layout".into());
        ui.close_menu();
    }
    if ui.button("Show Output Log").clicked() {
        app.project.log.push("[Submenu] Show Output Log".into());
        ui.close_menu();
    }
}

fn tools_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Options").clicked() {
        app.project.log.push("[Submenu] Tools Options".into());
        ui.close_menu();
    }
}

fn build_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Build Lighting").clicked() {
        app.project.log.push("[Submenu] Build Lighting".into());
        ui.close_menu();
    }
    if ui.button("Build All").clicked() {
        app.project.log.push("[Submenu] Build All".into());
        ui.close_menu();
    }
}

fn select_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Select All").clicked() {
        app.project.log.push("[Submenu] Select All".into());
        ui.close_menu();
    }
    if ui.button("Deselect All").clicked() {
        app.project.log.push("[Submenu] Deselect All".into());
        ui.close_menu();
    }
}

fn actor_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Add Actor").clicked() {
        app.project.log.push("[Submenu] Add Actor".into());
        ui.close_menu();
    }
    if ui.button("Delete Actor").clicked() {
        app.project.log.push("[Submenu] Delete Actor".into());
        ui.close_menu();
    }
}

fn components_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Add Component").clicked() {
        app.project.log.push("[Submenu] Add Component".into());
        ui.close_menu();
    }
    if ui.button("Remove Component").clicked() {
        app.project.log.push("[Submenu] Remove Component".into());
        ui.close_menu();
    }
}

fn level_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Add Level").clicked() {
        app.project.log.push("[Submenu] Add Level".into());
        ui.close_menu();
    }
    if ui.button("Delete Level").clicked() {
        app.project.log.push("[Submenu] Delete Level".into());
        ui.close_menu();
    }
}

fn blueprints_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Open Blueprint Editor").clicked() {
        app.ui_state.blueprint_open = true;
        app.project.log.push("[Submenu] Open Blueprint Editor".into());
        ui.close_menu();
    }
    if ui.button("Compile Blueprints").clicked() {
        app.project.log.push("[Submenu] Compile Blueprints".into());
        ui.close_menu();
    }
}

fn materials_fx_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Add Material").clicked() {
        app.project.log.push("[Submenu] Add Material".into());
        ui.close_menu();
    }
    if ui.button("Delete Material").clicked() {
        app.project.log.push("[Submenu] Delete Material".into());
        ui.close_menu();
    }
}

fn cinematics_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Add Cinematic").clicked() {
        app.project.log.push("[Submenu] Add Cinematic".into());
        ui.close_menu();
    }
    if ui.button("Delete Cinematic").clicked() {
        app.project.log.push("[Submenu] Delete Cinematic".into());
        ui.close_menu();
    }
}

fn play_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("Play").clicked() {
        app.project.log.push("[Submenu] Play".into());
        ui.close_menu();
    }
    if ui.button("Pause").clicked() {
        app.project.log.push("[Submenu] Pause".into());
        ui.close_menu();
    }
}

fn help_submenu(ui: &mut egui::Ui, app: &mut EditorApp) {
    if ui.button("About").clicked() {
        app.project.log.push("[Submenu] About".into());
        ui.close_menu();
    }
    if ui.button("Documentation").clicked() {
        app.project.log.push("[Submenu] Documentation".into());
        ui.close_menu();
    }
}
