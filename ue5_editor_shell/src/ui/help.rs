use crate::app::EditorApp;

pub fn draw(ctx: &egui::Context, app: &mut EditorApp) {
    if app.ui_state.show_about_window {
        egui::Window::new("About")
            .open(&mut app.ui_state.show_about_window)
            .default_size([480.0, 320.0])
            .show(ctx, |ui| {
                ui.heading("UE5 Editor Shell (Rust)");
                ui.label("Version: 0.1.0");
                ui.label("Engine: egui + eframe");
                ui.separator();
                ui.label("This editor shell provides:");
                ui.label("- Menus, toolbar, viewport, outliner, details, content browser");
                ui.label("- Blueprint mock window");
                ui.label("- Command routing and options systems");
                ui.separator();
                ui.label("Project: Example Project");
            });
    }

    if app.ui_state.show_developer_window {
        egui::Window::new("Developer Info")
            .open(&mut app.ui_state.show_developer_window)
            .default_size([640.0, 380.0])
            .show(ctx, |ui| {
                ui.heading("Developer Information");
                ui.separator();
                ui.label("Architecture:");
                ui.monospace("App -> UI -> Command Queue -> Reducer -> Services/State");
                ui.label("Key modules:");
                ui.monospace("src/actions/commands.rs");
                ui.monospace("src/ui/topbar.rs");
                ui.monospace("src/ui/dock.rs");
                ui.monospace("src/editor_api/*");
                ui.separator();
                ui.label("Debug Hints:");
                ui.label("- Use Output Log for menu/command tracing");
                ui.label("- Use Settings panel to validate option state");
                ui.label("- Use cargo check -p ue5_editor_shell for compile validation");
            });
    }

    if app.ui_state.show_documentation_window {
        egui::Window::new("Documentation")
            .open(&mut app.ui_state.show_documentation_window)
            .default_size([680.0, 420.0])
            .show(ctx, |ui| {
                ui.heading("Documentation");
                ui.separator();
                ui.label("User Docs");
                ui.label("- File/Edit/Window/Tools/Build/Play menu flows");
                ui.label("- Viewport, Outliner, Details, Content Browser workflow");
                ui.separator();
                ui.label("Developer Docs");
                ui.label("- Command enum and apply reducer");
                ui.label("- AppCore service interfaces");
                ui.label("- Menu options stored in ProjectState::menu_options");
                ui.separator();
                ui.label("Recommended local docs:");
                ui.monospace("README.md");
                ui.monospace("ue5_editor_shell/src/actions/commands.rs");
                ui.monospace("ue5_editor_shell/src/state/mod.rs");
            });
    }

    if app.ui_state.show_help_info_window {
        egui::Window::new("Help Info")
            .open(&mut app.ui_state.show_help_info_window)
            .default_size([540.0, 360.0])
            .show(ctx, |ui| {
                ui.heading("Help Information");
                ui.separator();
                ui.label("Hotkeys:");
                for line in app.action_registry.action_hints() {
                    ui.label(line);
                }
                ui.label("Compile Blueprints [Ctrl+B]");
                ui.label("Delete Actor [Delete]");
                ui.separator();
                ui.label("Support:");
                ui.label("- Check Output Log for operation diagnostics");
                ui.label("- Use Help > Documentation for workflow notes");
                ui.label("- Use Help > Developer Info for module-level details");
            });
    }
}
