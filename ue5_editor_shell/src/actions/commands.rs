use crate::{state, ui};

#[derive(Clone)]
pub enum EditorCommand {
    NewProject,
    SaveProject,
    BuildLighting,
    CompileBlueprints,
    PlayInEditor,
    StopPlay,
    AddActor,
    DuplicateActor,
    DeleteActor,
    ToggleActorVisibility,
    RenameActor(String),
    SelectActor(usize),
    MoveActor { x: f32, y: f32, z: f32 },
    RotateActor { pitch: f32, yaw: f32, roll: f32 },
    ScaleActor { x: f32, y: f32, z: f32 },
    SelectContent(usize),
    AddContent(String),
    RemoveContent,
    SetContentFilter(String),
    SetMode(state::EditorMode),
    TogglePanelBlueprint,
    TogglePanelOutliner,
    TogglePanelDetails,
    TogglePanelContent,
    TogglePanelOutput,
    TogglePanelSettings,
    ToggleGrid,
    ToggleSnap,
    SetSnapValue(f32),
    SetAutosave(bool),
    SetAutosaveMinutes(u32),
    SetRealtimeViewport(bool),
    Undo,
    Redo,
    ResetLayout,
    AddLog(String),
}

pub fn drain_and_apply(
    project: &mut state::ProjectState,
    ui_state: &mut ui::UiState,
    _registry: &crate::actions::registry::ActionRegistry,
) {
    let queue = std::mem::take(&mut ui_state.pending_commands);
    for cmd in queue {
        apply_command(project, ui_state, cmd);
    }
    ui_state.sanitize_selection(project);
}

fn apply_command(project: &mut state::ProjectState, ui_state: &mut ui::UiState, command: EditorCommand) {
    match command {
        EditorCommand::NewProject => {
            ui_state.push_undo_snapshot(project);
            *project = state::ProjectState::default();
            ui_state.selected_actor = if project.actors.is_empty() { None } else { Some(0) };
            ui_state.selected_content = if project.content_items.is_empty() { None } else { Some(0) };
            project.log("[LogEditor] New project created.");
        }
        EditorCommand::SaveProject => {
            project.dirty = false;
            project.log("[LogSave] Project saved.");
            ui_state.status_text = "Saved".to_owned();
        }
        EditorCommand::BuildLighting => {
            project.stats.shader_jobs = 0;
            project.log("[LogEditor] Lighting build complete.");
        }
        EditorCommand::CompileBlueprints => {
            project.log("[LogBlueprint] Compile successful.");
        }
        EditorCommand::PlayInEditor => {
            project.is_playing = true;
            project.log("[LogPIE] Play In Editor started.");
        }
        EditorCommand::StopPlay => {
            project.is_playing = false;
            project.log("[LogPIE] Play In Editor stopped.");
        }
        EditorCommand::AddActor => {
            ui_state.push_undo_snapshot(project);
            let name = format!("Actor_{}", project.actors.len() + 1);
            project.add_actor(&name);
            ui_state.selected_actor = Some(project.actors.len() - 1);
            project.log(format!("[LogActor] Added actor \"{}\".", name));
        }
        EditorCommand::DuplicateActor => {
            if let Some(idx) = ui_state.selected_actor {
                if let Some(actor) = project.actors.get(idx).cloned() {
                    ui_state.push_undo_snapshot(project);
                    let new_id = project.add_actor(&(actor.name.clone() + "_Copy"));
                    if let Some(new_actor) = project.actors.last_mut() {
                        new_actor.transform = actor.transform;
                        new_actor.visible = actor.visible;
                        new_actor.locked = actor.locked;
                        new_actor.id = new_id;
                    }
                    ui_state.selected_actor = Some(project.actors.len() - 1);
                    project.log("[LogActor] Duplicated selected actor.");
                }
            }
        }
        EditorCommand::DeleteActor => {
            if let Some(idx) = ui_state.selected_actor {
                if idx < project.actors.len() {
                    ui_state.push_undo_snapshot(project);
                    let name = project.actors[idx].name.clone();
                    project.actors.remove(idx);
                    ui_state.selected_actor = if project.actors.is_empty() {
                        None
                    } else {
                        Some(idx.min(project.actors.len() - 1))
                    };
                    project.dirty = true;
                    project.log(format!("[LogActor] Deleted actor \"{}\".", name));
                }
            }
        }
        EditorCommand::ToggleActorVisibility => {
            if let Some(actor) = ui_state.selected_actor.and_then(|i| project.actors.get_mut(i)) {
                actor.visible = !actor.visible;
                project.dirty = true;
            }
        }
        EditorCommand::RenameActor(name) => {
            if let Some(actor) = ui_state.selected_actor.and_then(|i| project.actors.get_mut(i)) {
                actor.name = name;
                project.dirty = true;
            }
        }
        EditorCommand::SelectActor(idx) => {
            if idx < project.actors.len() {
                ui_state.selected_actor = Some(idx);
            }
        }
        EditorCommand::MoveActor { x, y, z } => {
            if let Some(actor) = ui_state.selected_actor.and_then(|i| project.actors.get_mut(i)) {
                actor.transform.location = state::Vec3 { x, y, z };
                project.dirty = true;
            }
        }
        EditorCommand::RotateActor { pitch, yaw, roll } => {
            if let Some(actor) = ui_state.selected_actor.and_then(|i| project.actors.get_mut(i)) {
                actor.transform.rotation = state::Vec3 {
                    x: pitch,
                    y: yaw,
                    z: roll,
                };
                project.dirty = true;
            }
        }
        EditorCommand::ScaleActor { x, y, z } => {
            if let Some(actor) = ui_state.selected_actor.and_then(|i| project.actors.get_mut(i)) {
                actor.transform.scale = state::Vec3 { x, y, z };
                project.dirty = true;
            }
        }
        EditorCommand::SelectContent(idx) => {
            if idx < project.content_items.len() {
                ui_state.selected_content = Some(idx);
            }
        }
        EditorCommand::AddContent(name) => {
            ui_state.push_undo_snapshot(project);
            project
                .content_items
                .push(state::ContentItem::new(&name, state::ContentKind::StaticMesh));
            ui_state.selected_content = Some(project.content_items.len() - 1);
            project.dirty = true;
            project.log(format!("[LogContent] Added content \"{}\".", name));
        }
        EditorCommand::RemoveContent => {
            if let Some(idx) = ui_state.selected_content {
                if idx < project.content_items.len() {
                    ui_state.push_undo_snapshot(project);
                    let item = project.content_items.remove(idx);
                    ui_state.selected_content = if project.content_items.is_empty() {
                        None
                    } else {
                        Some(idx.min(project.content_items.len() - 1))
                    };
                    project.dirty = true;
                    project.log(format!("[LogContent] Removed content \"{}\".", item.name));
                }
            }
        }
        EditorCommand::SetContentFilter(text) => ui_state.content_filter = text,
        EditorCommand::SetMode(mode) => {
            project.mode = mode;
            project.log("[LogEditor] Mode switched.");
        }
        EditorCommand::TogglePanelBlueprint => ui_state.show_blueprint = !ui_state.show_blueprint,
        EditorCommand::TogglePanelOutliner => ui_state.show_outliner = !ui_state.show_outliner,
        EditorCommand::TogglePanelDetails => ui_state.show_details = !ui_state.show_details,
        EditorCommand::TogglePanelContent => ui_state.show_content_browser = !ui_state.show_content_browser,
        EditorCommand::TogglePanelOutput => ui_state.show_output_log = !ui_state.show_output_log,
        EditorCommand::TogglePanelSettings => ui_state.show_settings = !ui_state.show_settings,
        EditorCommand::ToggleGrid => project.settings.show_grid = !project.settings.show_grid,
        EditorCommand::ToggleSnap => project.settings.snap_enabled = !project.settings.snap_enabled,
        EditorCommand::SetSnapValue(v) => project.settings.snap_value = v.max(1.0),
        EditorCommand::SetAutosave(v) => project.settings.autosave_enabled = v,
        EditorCommand::SetAutosaveMinutes(v) => project.settings.autosave_minutes = v.max(1),
        EditorCommand::SetRealtimeViewport(v) => project.settings.realtime_viewport = v,
        EditorCommand::Undo => {
            if let Some(previous) = ui_state.undo_stack.pop() {
                ui_state.redo_stack.push(project.clone());
                *project = previous;
                project.log("[LogUndo] Undo");
            }
        }
        EditorCommand::Redo => {
            if let Some(next) = ui_state.redo_stack.pop() {
                ui_state.undo_stack.push(project.clone());
                *project = next;
                project.log("[LogUndo] Redo");
            }
        }
        EditorCommand::ResetLayout => {
            ui_state.reset_layout();
            ui_state.status_text = "Layout reset".to_owned();
        }
        EditorCommand::AddLog(line) => {
            project.log(line);
        }
    }
}
