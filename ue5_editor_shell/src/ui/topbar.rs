use crate::{
    actions::commands::EditorCommand,
    app::EditorApp,
    editor_api::types::{ELayoutPreset, EViewportViewMode},
    state::EditorMode,
};

pub fn draw_top_menubar(ctx: &egui::Context, app: &mut EditorApp) {
    egui::TopBottomPanel::top("top_menubar")
        .exact_height(28.0)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_menu(ui, app);
                edit_menu(ui, app);
                window_menu(ui, app);
                tools_menu(ui, app);
                build_menu(ui, app);
                select_menu(ui, app);
                actor_menu(ui, app);
                components_menu(ui, app);
                level_menu(ui, app);
                blueprints_menu(ui, app);
                materials_fx_menu(ui, app);
                cinematics_menu(ui, app);
                play_menu(ui, app);
                help_menu(ui, app);

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

fn file_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("File", |ui| {
        action(ui, app, "New Level / Project", EditorCommand::NewProject);
        action(ui, app, "Open Level", log_command("Open Level triggered."));
        action(ui, app, "Open Recent", log_command("Open Recent triggered."));
        action(ui, app, "Save", EditorCommand::SaveProject);
        action(ui, app, "Save All", log_command("Save All triggered."));
        action(ui, app, "Save Current Level As", log_command("Save Current Level As triggered."));
        ui.separator();
        action(ui, app, "Package Project", log_command("Package Project entry point."));
        action(ui, app, "Cook Content", log_command("Cook Content entry point."));
        ui.separator();
        action(ui, app, "Import", log_command("Import pipeline entry point."));
        action(ui, app, "Export", log_command("Export pipeline entry point."));
        ui.separator();
        ui.menu_button("Source Control", |ui| {
            action(
                ui,
                app,
                "Connect",
                EditorCommand::ConnectSourceControl("Git".to_owned()),
            );
            action(ui, app, "Disconnect", EditorCommand::DisconnectSourceControl);
            action(ui, app, "Check Out", log_command("Check Out selected files."));
            action(ui, app, "Submit", log_command("Submit changelist."));
            action(ui, app, "Revert", log_command("Revert selected files."));
            action(ui, app, "History", log_command("Open source history."));
        });
        ui.separator();
        action(ui, app, "Switch Project", log_command("Switch Project triggered."));
        action(ui, app, "Exit", log_command("Exit requested."));
    });
}

fn edit_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Edit", |ui| {
        action(ui, app, "Undo", EditorCommand::Undo);
        action(ui, app, "Redo", EditorCommand::Redo);
        ui.separator();
        action(ui, app, "Cut", log_command("Cut action."));
        action(ui, app, "Copy", log_command("Copy action."));
        action(ui, app, "Paste", log_command("Paste action."));
        action(ui, app, "Duplicate", EditorCommand::DuplicateActor);
        action(ui, app, "Delete", EditorCommand::DeleteActor);
        ui.separator();
        action(ui, app, "Editor Preferences", EditorCommand::TogglePanelSettings);
        action(ui, app, "Project Settings", EditorCommand::TogglePanelSettings);
        action(ui, app, "Plugins", log_command("Plugins manager opened."));
        action(ui, app, "Keyboard Shortcuts", log_command("Keyboard shortcuts panel opened."));
        action(ui, app, "Editor Utility Tools", log_command("Editor utility tools opened."));
    });
}

fn window_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Window", |ui| {
        ui.menu_button("Layouts", |ui| {
            action(ui, app, "Default", EditorCommand::SetLayoutPreset(ELayoutPreset::Default));
            action(ui, app, "Animation", EditorCommand::SetLayoutPreset(ELayoutPreset::Animation));
            action(ui, app, "Modeling", EditorCommand::SetLayoutPreset(ELayoutPreset::Modeling));
            action(ui, app, "Debug", EditorCommand::SetLayoutPreset(ELayoutPreset::Debug));
            action(ui, app, "Reset", EditorCommand::ResetLayout);
        });
        ui.separator();
        action(ui, app, "Content Browser", EditorCommand::TogglePanelContent);
        action(ui, app, "World Outliner", EditorCommand::TogglePanelOutliner);
        action(ui, app, "Details", EditorCommand::TogglePanelDetails);
        action(ui, app, "Output Log", EditorCommand::TogglePanelOutput);
        action(ui, app, "Message Log", log_command("Message Log opened."));
        ui.separator();
        action(ui, app, "Session Frontend", log_command("Session Frontend opened."));
        action(ui, app, "Profiler", log_command("Profiler opened."));
        action(ui, app, "Additional Viewport", log_command("Spawn additional viewport."));
    });
}

fn tools_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Tools", |ui| {
        action(ui, app, "Modeling Tools", log_command("Modeling tools activated."));
        action(ui, app, "Landscape", EditorCommand::SetMode(EditorMode::Landscape));
        action(ui, app, "Foliage", EditorCommand::SetMode(EditorMode::Foliage));
        action(ui, app, "Fracture (Chaos)", log_command("Fracture tools opened."));
        action(ui, app, "Nanite / LOD Tools", log_command("Nanite/LOD tools opened."));
        action(ui, app, "Blueprint Tools", log_command("Blueprint tools opened."));
        action(ui, app, "AI Tools", log_command("AI tools opened."));
        action(ui, app, "DataTable Editor", log_command("DataTable editor opened."));
        action(ui, app, "Curve Editor", log_command("Curve editor opened."));
        action(ui, app, "Automation Test Runner", log_command("Automation test runner opened."));
        action(ui, app, "Localization", log_command("Localization dashboard opened."));
        action(ui, app, "Asset Audit", log_command("Asset audit opened."));
        action(ui, app, "Size Map", log_command("Size map opened."));
        action(ui, app, "Reference Viewer", log_command("Reference viewer opened."));
    });
}

fn build_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Build", |ui| {
        action(ui, app, "Build Lighting", EditorCommand::BuildLighting);
        action(ui, app, "Build Geometry / Paths", EditorCommand::BuildGeometry);
        action(ui, app, "Build HLODs", log_command("Build HLODs started."));
        action(ui, app, "Build Reflection Captures", log_command("Build reflection captures started."));
        action(ui, app, "Build Navigation", EditorCommand::BuildNavigation);
        action(ui, app, "Build All", EditorCommand::BuildAll);
        action(ui, app, "Build Configuration", log_command("Build configuration opened."));
    });
}

fn select_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Select", |ui| {
        action(ui, app, "Select All", log_command("Select all actors."));
        action(ui, app, "Select None", log_command("Select none."));
        action(ui, app, "Invert Selection", log_command("Invert selection."));
        action(ui, app, "By Class", log_command("Select by class."));
        action(ui, app, "By Tag", log_command("Select by tag."));
        action(ui, app, "By Layer", log_command("Select by layer."));
        action(ui, app, "Actors Using This Asset", log_command("Select actors using asset."));
        action(ui, app, "Selection Filters", log_command("Selection filters opened."));
    });
}

fn actor_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Actor", |ui| {
        action(ui, app, "Add Actor", EditorCommand::AddActor);
        action(ui, app, "Convert Actor", log_command("Convert actor opened."));
        action(ui, app, "Merge Actors", log_command("Merge actors opened."));
        action(ui, app, "Replace Selected", log_command("Replace selected actors."));
        action(ui, app, "Attach", log_command("Attach actor."));
        action(ui, app, "Detach", log_command("Detach actor."));
        action(ui, app, "Group", log_command("Group actors."));
        action(ui, app, "Ungroup", log_command("Ungroup actors."));
        action(ui, app, "Pivot Tools", log_command("Pivot tools opened."));
        action(ui, app, "Snap Tools", log_command("Snap tools opened."));
    });
}

fn components_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Components", |ui| {
        action(ui, app, "Add Component", log_command("Add component action."));
        action(ui, app, "Edit Component Hierarchy", log_command("Component hierarchy opened."));
        action(ui, app, "Promote to Blueprint", log_command("Promote to blueprint."));
        action(ui, app, "Convert to BP Class", log_command("Convert to BP class."));
    });
}

fn level_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Level", |ui| {
        action(ui, app, "Level Settings", log_command("Level settings opened."));
        action(ui, app, "World Partition / Data Layers", log_command("World partition opened."));
        action(ui, app, "Streaming", log_command("Level streaming tools opened."));
        action(ui, app, "Lighting Scenarios", log_command("Lighting scenarios opened."));
        action(ui, app, "Level Blueprint", EditorCommand::TogglePanelBlueprint);
        action(ui, app, "World Settings", EditorCommand::TogglePanelSettings);
    });
}

fn blueprints_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Blueprints", |ui| {
        action(ui, app, "Open Blueprint Editor", EditorCommand::TogglePanelBlueprint);
        action(ui, app, "Create Blueprint Class", log_command("Create blueprint class wizard."));
        action(ui, app, "Interfaces / Libraries", log_command("Blueprint libraries manager."));
        action(ui, app, "Debug", log_command("Blueprint debug tools opened."));
        action(ui, app, "Compile", EditorCommand::CompileBlueprints);
        action(ui, app, "Refresh Nodes", log_command("Refresh nodes."));
        action(ui, app, "Reparent Blueprint", log_command("Reparent blueprint dialog."));
    });
}

fn materials_fx_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Materials/FX", |ui| {
        action(ui, app, "Material Editor", log_command("Material editor opened."));
        action(ui, app, "Material Instances", log_command("Material instance editor opened."));
        action(ui, app, "Niagara Systems/Emitters", log_command("Niagara editor opened."));
        action(ui, app, "Post Process / Color Grading", log_command("Post process tools opened."));
    });
}

fn cinematics_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Cinematics", |ui| {
        action(ui, app, "Sequencer", log_command("Sequencer opened."));
        action(ui, app, "Take Recorder", log_command("Take recorder opened."));
        action(ui, app, "Camera Rig Tools", log_command("Camera rig tools opened."));
        action(ui, app, "Render Queue", log_command("Render queue opened."));
    });
}

fn play_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Play", |ui| {
        action(ui, app, "PIE", EditorCommand::PlayInEditor);
        action(ui, app, "Simulate", log_command("Simulate started."));
        action(ui, app, "Standalone Game", log_command("Standalone game started."));
        action(ui, app, "VR Preview", log_command("VR preview started."));
        action(ui, app, "Multiplayer PIE Settings", log_command("Multiplayer PIE settings opened."));
        ui.separator();
        ui.menu_button("Viewport Mode", |ui| {
            action(ui, app, "Lit", EditorCommand::SetViewportMode(EViewportViewMode::Lit));
            action(ui, app, "Unlit", EditorCommand::SetViewportMode(EViewportViewMode::Unlit));
            action(
                ui,
                app,
                "Wireframe",
                EditorCommand::SetViewportMode(EViewportViewMode::Wireframe),
            );
        });
    });
}

fn help_menu(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.menu_button("Help", |ui| {
        action(ui, app, "Documentation", log_command("Documentation opened."));
        action(ui, app, "Samples", log_command("Samples opened."));
        action(ui, app, "About / Credits", log_command("About dialog opened."));
        action(ui, app, "Report a Bug", log_command("Bug report flow opened."));
        ui.separator();
        ui.label("Hotkeys:");
        for line in app.action_registry.action_hints() {
            ui.label(line);
        }
        ui.label("Compile Blueprints [Ctrl+B]");
        ui.label("Delete Actor [Delete]");
    });
}

fn action(ui: &mut egui::Ui, app: &mut EditorApp, label: &str, cmd: EditorCommand) {
    if ui.button(label).clicked() {
        app.ui_state.enqueue(cmd);
        ui.close_menu();
    }
}

fn log_command(message: &'static str) -> EditorCommand {
    EditorCommand::AddLog(format!("[LogTools] {}", message))
}
