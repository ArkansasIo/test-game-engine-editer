use crate::{actions::commands::EditorCommand, app::EditorApp, state::EditorMode};

pub fn draw(ctx: &egui::Context, app: &mut EditorApp) {
    draw_left_modes(ctx, app);

    if app.ui_state.show_outliner || app.ui_state.show_details || app.ui_state.show_settings {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(320.0)
            .show(ctx, |ui| {
                if app.ui_state.show_outliner {
                    draw_outliner(ui, app);
                    ui.separator();
                }
                if app.ui_state.show_details {
                    draw_details(ui, app);
                    ui.separator();
                }
                if app.ui_state.show_settings {
                    draw_settings(ui, app);
                }
            });
    }

    if app.ui_state.show_content_browser || app.ui_state.show_output_log {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(240.0)
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    if app.ui_state.show_content_browser {
                        draw_content_browser(&mut columns[0], app);
                    } else {
                        columns[0].centered_and_justified(|ui| {
                            ui.label("Content Browser hidden");
                        });
                    }

                    if app.ui_state.show_output_log {
                        draw_output_log(&mut columns[1], app);
                    } else {
                        columns[1].centered_and_justified(|ui| {
                            ui.label("Output Log hidden");
                        });
                    }
                });
            });
    }

    draw_viewport(ctx, app);
}

fn draw_left_modes(ctx: &egui::Context, app: &mut EditorApp) {
    egui::SidePanel::left("modes_panel")
        .resizable(true)
        .default_width(220.0)
        .show(ctx, |ui| {
            ui.heading("Modes");
            ui.separator();
            mode_card(ui, app, "Place Actors", EditorMode::Select);
            mode_card(ui, app, "Landscape", EditorMode::Landscape);
            mode_card(ui, app, "Foliage", EditorMode::Foliage);
            mode_card(ui, app, "Modeling", EditorMode::Modeling);
            ui.separator();
            ui.label(egui::RichText::new("Tools").strong());
            ui.horizontal_wrapped(|ui| {
                if ui.button("+ Actor").clicked() {
                    app.ui_state.enqueue(EditorCommand::AddActor);
                }
                if ui.button("Duplicate").clicked() {
                    app.ui_state.enqueue(EditorCommand::DuplicateActor);
                }
                if ui.button("Delete").clicked() {
                    app.ui_state.enqueue(EditorCommand::DeleteActor);
                }
                if ui.button("Toggle Visible").clicked() {
                    app.ui_state.enqueue(EditorCommand::ToggleActorVisibility);
                }
            });
        });
}

fn draw_outliner(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("World Outliner");
    ui.horizontal(|ui| {
        ui.label("Search");
        ui.text_edit_singleline(&mut app.ui_state.actor_search);
    });

    egui::ScrollArea::vertical()
        .max_height(200.0)
        .show(ui, |ui| {
            for (i, actor) in app.project.actors.iter().enumerate() {
                if !app.ui_state.actor_search.is_empty()
                    && !actor
                        .name
                        .to_lowercase()
                        .contains(&app.ui_state.actor_search.to_lowercase())
                {
                    continue;
                }
                let selected = app.ui_state.selected_actor == Some(i);
                let label = if actor.visible {
                    actor.name.clone()
                } else {
                    format!("{} (hidden)", actor.name)
                };
                if ui.selectable_label(selected, label).clicked() {
                    app.ui_state.enqueue(EditorCommand::SelectActor(i));
                    app.ui_state.rename_actor_buffer = actor.name.clone();
                }
            }
        });
}

fn draw_details(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Details");
    let Some(selected) = app.ui_state.selected_actor else {
        ui.label("No actor selected");
        return;
    };
    if selected >= app.project.actors.len() {
        ui.label("No actor selected");
        return;
    }

    let actor = &app.project.actors[selected];
    if app.ui_state.rename_actor_buffer.is_empty() {
        app.ui_state.rename_actor_buffer = actor.name.clone();
    }

    ui.horizontal(|ui| {
        ui.label("Name");
        if ui
            .text_edit_singleline(&mut app.ui_state.rename_actor_buffer)
            .lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
        {
            app.ui_state
                .enqueue(EditorCommand::RenameActor(app.ui_state.rename_actor_buffer.clone()));
        }
    });

    ui.horizontal(|ui| {
        if ui.button("Apply Name").clicked() {
            app.ui_state
                .enqueue(EditorCommand::RenameActor(app.ui_state.rename_actor_buffer.clone()));
        }
        if ui
            .button(if actor.visible { "Hide" } else { "Show" })
            .clicked()
        {
            app.ui_state.enqueue(EditorCommand::ToggleActorVisibility);
        }
    });

    let mut loc = actor.transform.location;
    let mut rot = actor.transform.rotation;
    let mut scale = actor.transform.scale;

    ui.separator();
    ui.label(egui::RichText::new("Transform").strong());
    if vec3_controls(ui, "Location", &mut loc) {
        app.ui_state.enqueue(EditorCommand::MoveActor {
            x: loc.x,
            y: loc.y,
            z: loc.z,
        });
    }
    if vec3_controls(ui, "Rotation", &mut rot) {
        app.ui_state.enqueue(EditorCommand::RotateActor {
            pitch: rot.x,
            yaw: rot.y,
            roll: rot.z,
        });
    }
    if vec3_controls(ui, "Scale", &mut scale) {
        app.ui_state.enqueue(EditorCommand::ScaleActor {
            x: scale.x,
            y: scale.y,
            z: scale.z,
        });
    }
}

fn draw_settings(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Project Settings");
    ui.horizontal(|ui| {
        let mut v = app.project.settings.show_grid;
        if ui.checkbox(&mut v, "Show Grid").changed() {
            app.ui_state.enqueue(EditorCommand::ToggleGrid);
        }
    });
    ui.horizontal(|ui| {
        let mut v = app.project.settings.snap_enabled;
        if ui.checkbox(&mut v, "Enable Snap").changed() {
            app.ui_state.enqueue(EditorCommand::ToggleSnap);
        }
    });
    ui.horizontal(|ui| {
        ui.label("Snap Value");
        let mut snap = app.project.settings.snap_value;
        if ui.add(egui::DragValue::new(&mut snap).speed(0.25)).changed() {
            app.ui_state.enqueue(EditorCommand::SetSnapValue(snap));
        }
    });
    ui.horizontal(|ui| {
        let mut v = app.project.settings.autosave_enabled;
        if ui.checkbox(&mut v, "Autosave").changed() {
            app.ui_state.enqueue(EditorCommand::SetAutosave(v));
        }
    });
    ui.horizontal(|ui| {
        ui.label("Autosave (min)");
        let mut mins = app.project.settings.autosave_minutes;
        if ui.add(egui::DragValue::new(&mut mins).speed(1.0)).changed() {
            app.ui_state.enqueue(EditorCommand::SetAutosaveMinutes(mins));
        }
    });
    ui.horizontal(|ui| {
        let mut v = app.project.settings.realtime_viewport;
        if ui.checkbox(&mut v, "Realtime Viewport").changed() {
            app.ui_state.enqueue(EditorCommand::SetRealtimeViewport(v));
        }
    });
    ui.separator();
    ui.label(egui::RichText::new("Loaded Modules").strong());
    egui::ScrollArea::vertical()
        .max_height(120.0)
        .show(ui, |ui| {
            for module in &app.modules {
                ui.label(format!(
                    "{} [cmd:{} tabs:{} svc:{}]",
                    module.name, module.provides_commands, module.provides_tabs, module.provides_services
                ));
            }
        });
}

fn draw_content_browser(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Content Browser");
    ui.horizontal(|ui| {
        ui.label("Filter");
        if ui
            .text_edit_singleline(&mut app.ui_state.content_filter)
            .changed()
        {
            app.ui_state
                .enqueue(EditorCommand::SetContentFilter(app.ui_state.content_filter.clone()));
        }
    });
    ui.horizontal(|ui| {
        ui.label("Add");
        ui.text_edit_singleline(&mut app.ui_state.add_content_buffer);
        if ui.button("Import").clicked() {
            if !app.ui_state.add_content_buffer.trim().is_empty() {
                app.ui_state.enqueue(EditorCommand::AddContent(
                    app.ui_state.add_content_buffer.trim().to_owned(),
                ));
                app.ui_state.add_content_buffer.clear();
            }
        }
        if ui.button("Remove").clicked() {
            app.ui_state.enqueue(EditorCommand::RemoveContent);
        }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        for (i, item) in app.project.content_items.iter().enumerate() {
            if !app.ui_state.content_filter.is_empty()
                && !item
                    .name
                    .to_lowercase()
                    .contains(&app.ui_state.content_filter.to_lowercase())
            {
                continue;
            }
            let selected = app.ui_state.selected_content == Some(i);
            let label = format!("{} ({})", item.name, item.kind.label());
            if ui.selectable_label(selected, label).clicked() {
                app.ui_state.enqueue(EditorCommand::SelectContent(i));
            }
        }
    });
}

fn draw_output_log(ui: &mut egui::Ui, app: &mut EditorApp) {
    ui.heading("Output Log");
    ui.horizontal(|ui| {
        if ui.button("Clear").clicked() {
            app.project.output_log.clear();
            app.project.log("[LogEditor] Log cleared.");
        }
        if ui.button("Test Event").clicked() {
            app.ui_state
                .enqueue(EditorCommand::AddLog("[LogTest] Simulated event fired.".to_owned()));
        }
    });
    egui::ScrollArea::vertical().show(ui, |ui| {
        for line in app.project.output_log.iter().rev() {
            ui.label(line);
        }
    });
}

fn draw_viewport(ctx: &egui::Context, app: &mut EditorApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Perspective");
            ui.separator();
            ui.label(match app.project.mode {
                EditorMode::Select => "Mode: Place Actors",
                EditorMode::Landscape => "Mode: Landscape",
                EditorMode::Foliage => "Mode: Foliage",
                EditorMode::Modeling => "Mode: Modeling",
            });
            ui.separator();
            ui.label(if app.project.settings.realtime_viewport {
                "Realtime"
            } else {
                "Paused"
            });
            ui.separator();
            ui.label(format!("FPS: {}", app.project.stats.fps));
            ui.separator();
            ui.label(match app.project.view_mode {
                crate::state::ViewMode::Lit => "Lit",
                crate::state::ViewMode::Unlit => "Unlit",
                crate::state::ViewMode::Wireframe => "Wireframe",
            });
        });
        ui.separator();

        let available = ui.available_rect_before_wrap();
        let painter = ui.painter();
        painter.rect_filled(available, 4.0, egui::Color32::from_rgb(32, 38, 46));
        if app.project.settings.show_grid {
            let spacing = 28.0;
            let mut x = available.left();
            while x < available.right() {
                painter.line_segment(
                    [egui::pos2(x, available.top()), egui::pos2(x, available.bottom())],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 62, 74)),
                );
                x += spacing;
            }
            let mut y = available.top();
            while y < available.bottom() {
                painter.line_segment(
                    [egui::pos2(available.left(), y), egui::pos2(available.right(), y)],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 62, 74)),
                );
                y += spacing;
            }
        }
        painter.text(
            available.center(),
            egui::Align2::CENTER_CENTER,
            if app.project.is_playing {
                "Play In Editor"
            } else {
                "Viewport Preview"
            },
            egui::TextStyle::Heading.resolve(ui.style()),
            egui::Color32::from_gray(210),
        );
        ui.allocate_rect(available, egui::Sense::hover());
    });
}

fn mode_card(ui: &mut egui::Ui, app: &mut EditorApp, label: &str, mode: EditorMode) {
    let selected = app.project.mode == mode;
    if ui
        .add_sized([180.0, 26.0], egui::SelectableLabel::new(selected, label))
        .clicked()
    {
        app.ui_state.enqueue(EditorCommand::SetMode(mode));
    }
}

fn vec3_controls(ui: &mut egui::Ui, label: &str, value: &mut crate::state::Vec3) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label);
        changed |= ui
            .add(egui::DragValue::new(&mut value.x).speed(0.1).prefix("X "))
            .changed();
        changed |= ui
            .add(egui::DragValue::new(&mut value.y).speed(0.1).prefix("Y "))
            .changed();
        changed |= ui
            .add(egui::DragValue::new(&mut value.z).speed(0.1).prefix("Z "))
            .changed();
    });
    changed
}
