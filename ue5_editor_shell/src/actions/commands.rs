use crate::{
    app::EditorApp,
    editor_api::types::{EBuildTask, ELayoutPreset, EViewportViewMode},
    state,
};

#[derive(Clone)]
pub enum EditorCommand {
    NewProject,
    SaveProject,
    BuildLighting,
    BuildGeometry,
    BuildNavigation,
    BuildAll,
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
    SetActiveDimension(state::SceneDimension),
    CreateScene {
        name: String,
        dimension: state::SceneDimension,
    },
    SelectScene(usize),
    CreateLevel {
        name: String,
    },
    SelectLevel(usize),
    CreateResource {
        name: String,
        kind: state::ResourceKind,
    },
    RemoveResource,
    SetContentFilter(String),
    SetMode(state::EditorMode),
    SetLayoutPreset(ELayoutPreset),
    SetViewportMode(EViewportViewMode),
    ConnectSourceControl(String),
    DisconnectSourceControl,
    TogglePanelBlueprint,
    TogglePanelOutliner,
    TogglePanelDetails,
    TogglePanelContent,
    TogglePanelOutput,
    TogglePanelTerminal,
    TogglePanelSettings,
    ToggleGrid,
    ToggleSnap,
    SetSnapValue(f32),
    SetAutosave(bool),
    SetAutosaveMinutes(u32),
    SetRealtimeViewport(bool),
    SetMenuOption(state::MenuOptionKey, bool),
    SetPlayClientCount(u32),
    ShowAboutWindow,
    ShowDeveloperWindow,
    ShowDocumentationWindow,
    ShowHelpInfoWindow,
    ExecuteTerminalCommand(String),
    ClearTerminal,
    Undo,
    Redo,
    ResetLayout,
    AddLog(String),
}

pub fn drain_and_apply(app: &mut EditorApp) {
    let queue = std::mem::take(&mut app.ui_state.pending_commands);
    for cmd in queue {
        apply_command(app, cmd);
    }
    app.ui_state.sanitize_selection(&app.project);
}

fn apply_command(app: &mut EditorApp, command: EditorCommand) {
    let project = &mut app.project;
    let ui_state = &mut app.ui_state;

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
            if project.menu_options.file_auto_save_on_build {
                project.dirty = false;
                project.log("[LogSave] Auto-saved before build.");
            }
            app.app_core.enqueue_build(EBuildTask::Lighting);
            project.stats.shader_jobs = 0;
            project.log("[LogBuild] Lighting build complete.");
        }
        EditorCommand::BuildGeometry => {
            if project.menu_options.file_auto_save_on_build {
                project.dirty = false;
                project.log("[LogSave] Auto-saved before build.");
            }
            app.app_core.enqueue_build(EBuildTask::Geometry);
            project.log("[LogBuild] Geometry build complete.");
        }
        EditorCommand::BuildNavigation => {
            if project.menu_options.file_auto_save_on_build {
                project.dirty = false;
                project.log("[LogSave] Auto-saved before build.");
            }
            app.app_core.enqueue_build(EBuildTask::Navigation);
            project.log("[LogBuild] Navigation build complete.");
        }
        EditorCommand::BuildAll => {
            if project.menu_options.file_auto_save_on_build {
                project.dirty = false;
                project.log("[LogSave] Auto-saved before build.");
            }
            app.app_core.enqueue_build(EBuildTask::All);
            project.log("[LogBuild] Build all complete.");
        }
        EditorCommand::CompileBlueprints => {
            project.log("[LogBlueprint] Compile successful.");
        }
        EditorCommand::PlayInEditor => {
            project.is_playing = true;
            if project.menu_options.play_start_in_simulate {
                project.log("[LogPIE] Simulate mode started.");
            } else {
                project.log("[LogPIE] Play In Editor started.");
            }
            if project.menu_options.play_multiplayer_pie {
                project.log(format!(
                    "[LogPIE] Multiplayer PIE enabled with {} client(s).",
                    project.menu_options.play_client_count
                ));
            }
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
        EditorCommand::SetActiveDimension(dimension) => {
            project.active_dimension = dimension;
            project.log(format!("[LogScene] Switched to {} view.", dimension.label()));
        }
        EditorCommand::CreateScene { name, dimension } => {
            let final_name = if name.trim().is_empty() {
                format!("Scene_{}", project.scenes.len() + 1)
            } else {
                name.trim().to_owned()
            };
            ui_state.push_undo_snapshot(project);
            project.create_scene(final_name.clone(), dimension);
            ui_state.selected_scene = Some(project.active_scene);
            project.log(format!(
                "[LogScene] Created {} scene \"{}\".",
                dimension.label(),
                final_name
            ));
        }
        EditorCommand::SelectScene(idx) => {
            if let Some(scene) = project.scenes.get(idx) {
                project.active_scene = idx;
                project.active_dimension = scene.dimension;
                ui_state.selected_scene = Some(idx);
                project.log(format!("[LogScene] Active scene set to \"{}\".", scene.name));
            }
        }
        EditorCommand::CreateLevel { name } => {
            let final_name = if name.trim().is_empty() {
                format!("Level_{}", project.levels.len() + 1)
            } else {
                name.trim().to_owned()
            };
            let (scene_name, dimension) = project
                .scenes
                .get(project.active_scene)
                .map(|s| (s.name.clone(), s.dimension))
                .unwrap_or_else(|| ("DefaultScene".to_owned(), project.active_dimension));
            ui_state.push_undo_snapshot(project);
            project.create_level(final_name.clone(), scene_name.clone(), dimension);
            ui_state.selected_level = Some(project.active_level);
            project.log(format!(
                "[LogLevel] Created level \"{}\" in scene \"{}\".",
                final_name, scene_name
            ));
        }
        EditorCommand::SelectLevel(idx) => {
            if let Some(level) = project.levels.get(idx) {
                project.active_level = idx;
                project.current_map = level.name.clone();
                project.active_dimension = level.dimension;
                ui_state.selected_level = Some(idx);
                project.log(format!("[LogLevel] Active level set to \"{}\".", level.name));
            }
        }
        EditorCommand::CreateResource { name, kind } => {
            let final_name = if name.trim().is_empty() {
                format!("{}_{:03}", kind.label(), project.resources.len() + 1)
            } else {
                name.trim().to_owned()
            };
            ui_state.push_undo_snapshot(project);
            project.create_resource(final_name.clone(), kind);
            ui_state.selected_resource = Some(project.resources.len() - 1);
            project.log(format!(
                "[LogResource] Created resource \"{}\" ({})",
                final_name,
                kind.label()
            ));
        }
        EditorCommand::RemoveResource => {
            if let Some(idx) = ui_state.selected_resource {
                if idx < project.resources.len() {
                    ui_state.push_undo_snapshot(project);
                    let removed = project.resources.remove(idx);
                    ui_state.selected_resource = if project.resources.is_empty() {
                        None
                    } else {
                        Some(idx.min(project.resources.len() - 1))
                    };
                    project.log(format!(
                        "[LogResource] Removed resource \"{}\".",
                        removed.name
                    ));
                    project.dirty = true;
                }
            }
        }
        EditorCommand::SetContentFilter(text) => ui_state.content_filter = text,
        EditorCommand::SetMode(mode) => {
            project.mode = mode;
            project.log("[LogEditor] Mode switched.");
        }
        EditorCommand::SetLayoutPreset(preset) => {
            app.app_core.apply_layout_preset(preset);
            ui_state.status_text = format!("Layout preset: {:?}", preset);
        }
        EditorCommand::SetViewportMode(mode) => {
            app.app_core.set_view_mode(mode);
            project.view_mode = match mode {
                EViewportViewMode::Lit => state::ViewMode::Lit,
                EViewportViewMode::Unlit => state::ViewMode::Unlit,
                EViewportViewMode::Wireframe => state::ViewMode::Wireframe,
                EViewportViewMode::DetailLighting => state::ViewMode::Lit,
                EViewportViewMode::BufferVisualization => state::ViewMode::Lit,
            };
        }
        EditorCommand::ConnectSourceControl(provider) => {
            app.app_core.connect_source_control(&provider);
            project.log(format!("[LogSourceControl] Connected to {}.", provider));
        }
        EditorCommand::DisconnectSourceControl => {
            app.app_core.disconnect_source_control();
            project.log("[LogSourceControl] Disconnected.");
        }
        EditorCommand::TogglePanelBlueprint => ui_state.show_blueprint = !ui_state.show_blueprint,
        EditorCommand::TogglePanelOutliner => ui_state.show_outliner = !ui_state.show_outliner,
        EditorCommand::TogglePanelDetails => ui_state.show_details = !ui_state.show_details,
        EditorCommand::TogglePanelContent => ui_state.show_content_browser = !ui_state.show_content_browser,
        EditorCommand::TogglePanelOutput => ui_state.show_output_log = !ui_state.show_output_log,
        EditorCommand::TogglePanelTerminal => ui_state.show_terminal = !ui_state.show_terminal,
        EditorCommand::TogglePanelSettings => ui_state.show_settings = !ui_state.show_settings,
        EditorCommand::ToggleGrid => project.settings.show_grid = !project.settings.show_grid,
        EditorCommand::ToggleSnap => project.settings.snap_enabled = !project.settings.snap_enabled,
        EditorCommand::SetSnapValue(v) => project.settings.snap_value = v.max(1.0),
        EditorCommand::SetAutosave(v) => project.settings.autosave_enabled = v,
        EditorCommand::SetAutosaveMinutes(v) => project.settings.autosave_minutes = v.max(1),
        EditorCommand::SetRealtimeViewport(v) => project.settings.realtime_viewport = v,
        EditorCommand::SetMenuOption(key, value) => {
            project.set_menu_option(key, value);
            project.dirty = true;
        }
        EditorCommand::SetPlayClientCount(v) => {
            project.menu_options.play_client_count = v.max(1);
        }
        EditorCommand::ShowAboutWindow => {
            ui_state.show_about_window = true;
            project.log("[LogHelp] Opened About window.");
        }
        EditorCommand::ShowDeveloperWindow => {
            ui_state.show_developer_window = true;
            project.log("[LogHelp] Opened Developer window.");
        }
        EditorCommand::ShowDocumentationWindow => {
            ui_state.show_documentation_window = true;
            project.log("[LogHelp] Opened Documentation window.");
        }
        EditorCommand::ShowHelpInfoWindow => {
            ui_state.show_help_info_window = true;
            project.log("[LogHelp] Opened Help Info window.");
        }
        EditorCommand::ExecuteTerminalCommand(cmd) => {
            let input = cmd.trim().to_owned();
            if input.is_empty() {
                return;
            }
            project.terminal.command_history.push(input.clone());
            project.terminal_log(
                state::TerminalSeverity::Info,
                "Terminal",
                format!("> {}", input),
            );
            match input.as_str() {
                "help" => {
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Terminal",
                        "Commands: help, clear, errors, warnings, play, stop, build, stats, list scenes, list levels, list resources",
                    );
                }
                "clear" => {
                    project.terminal.entries.clear();
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Terminal",
                        "Terminal output cleared.",
                    );
                }
                "errors" => {
                    let count = project
                        .terminal
                        .entries
                        .iter()
                        .filter(|e| e.severity == state::TerminalSeverity::Error)
                        .count();
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Diagnostics",
                        format!("Error count: {}", count),
                    );
                }
                "warnings" => {
                    let count = project
                        .terminal
                        .entries
                        .iter()
                        .filter(|e| e.severity == state::TerminalSeverity::Warning)
                        .count();
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Diagnostics",
                        format!("Warning count: {}", count),
                    );
                }
                "play" => {
                    project.is_playing = true;
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Runtime",
                        "Play In Editor started.",
                    );
                }
                "stop" => {
                    project.is_playing = false;
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Runtime",
                        "Play In Editor stopped.",
                    );
                }
                "build" => {
                    app.app_core.enqueue_build(EBuildTask::All);
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Build",
                        "Build all triggered from terminal.",
                    );
                }
                "stats" => {
                    project.terminal_log(
                        state::TerminalSeverity::Info,
                        "Stats",
                        format!(
                            "FPS={}, Shaders={}, Actors={}, Resources={}",
                            project.stats.fps,
                            project.stats.shader_jobs,
                            project.actors.len(),
                            project.resources.len()
                        ),
                    );
                }
                "list scenes" => {
                    let msg = project
                        .scenes
                        .iter()
                        .map(|s| format!("{}({})", s.name, s.dimension.label()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    project.terminal_log(state::TerminalSeverity::Info, "Scenes", msg);
                }
                "list levels" => {
                    let msg = project
                        .levels
                        .iter()
                        .map(|l| format!("{}[{}]", l.name, l.scene_name))
                        .collect::<Vec<_>>()
                        .join(", ");
                    project.terminal_log(state::TerminalSeverity::Info, "Levels", msg);
                }
                "list resources" => {
                    let msg = project
                        .resources
                        .iter()
                        .map(|r| format!("{}:{}", r.name, r.kind.label()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    project.terminal_log(state::TerminalSeverity::Info, "Resources", msg);
                }
                _ => {
                    project.terminal_log(
                        state::TerminalSeverity::Error,
                        "Terminal",
                        format!("Unknown command: {}", input),
                    );
                }
            }
        }
        EditorCommand::ClearTerminal => {
            project.terminal.entries.clear();
            project.terminal_log(
                state::TerminalSeverity::Info,
                "Terminal",
                "Terminal output cleared.",
            );
        }
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
            app.app_core.apply_layout_preset(ELayoutPreset::Default);
            ui_state.status_text = "Layout reset".to_owned();
        }
        EditorCommand::AddLog(line) => {
            project.log(line);
        }
    }
}
