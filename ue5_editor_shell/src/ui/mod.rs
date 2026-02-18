use crate::{actions::commands::EditorCommand, app::EditorApp, state::ProjectState};

pub mod blueprint;
pub mod dock;
pub mod help;
pub mod status_bar;
pub mod topbar;
pub mod toolbar;

#[derive(Default)]
pub struct Hotkeys;

impl Hotkeys {
    pub fn update(&mut self, ctx: &egui::Context, queue: &mut Vec<EditorCommand>) {
        let save = ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl);
        let undo = ctx.input(|i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl);
        let redo = ctx.input(|i| i.key_pressed(egui::Key::Y) && i.modifiers.ctrl);
        let play = ctx.input(|i| i.key_pressed(egui::Key::F5));
        let delete = ctx.input(|i| i.key_pressed(egui::Key::Delete));
        let compile = ctx.input(|i| i.key_pressed(egui::Key::B) && i.modifiers.ctrl);

        if save {
            queue.push(EditorCommand::SaveProject);
        }
        if undo {
            queue.push(EditorCommand::Undo);
        }
        if redo {
            queue.push(EditorCommand::Redo);
        }
        if play {
            queue.push(EditorCommand::PlayInEditor);
        }
        if delete {
            queue.push(EditorCommand::DeleteActor);
        }
        if compile {
            queue.push(EditorCommand::CompileBlueprints);
        }
    }
}

pub struct UiState {
    pub hotkeys: Hotkeys,
    pub pending_commands: Vec<EditorCommand>,
    pub selected_actor: Option<usize>,
    pub selected_content: Option<usize>,
    pub show_blueprint: bool,
    pub show_outliner: bool,
    pub show_details: bool,
    pub show_content_browser: bool,
    pub show_output_log: bool,
    pub show_settings: bool,
    pub actor_search: String,
    pub content_filter: String,
    pub rename_actor_buffer: String,
    pub add_content_buffer: String,
    pub new_scene_buffer: String,
    pub new_level_buffer: String,
    pub new_resource_buffer: String,
    pub theme: String,
    pub selected_scene: Option<usize>,
    pub selected_level: Option<usize>,
    pub selected_resource: Option<usize>,
    pub status_text: String,
    pub show_about_window: bool,
    pub show_developer_window: bool,
    pub show_documentation_window: bool,
    pub show_help_info_window: bool,
    pub undo_stack: Vec<ProjectState>,
    pub redo_stack: Vec<ProjectState>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            hotkeys: Hotkeys,
            pending_commands: Vec::new(),
            selected_actor: Some(0),
            selected_content: Some(0),
            show_blueprint: true,
            show_outliner: true,
            show_details: true,
            show_content_browser: true,
            show_output_log: true,
            show_settings: false,
            actor_search: String::new(),
            content_filter: String::new(),
            rename_actor_buffer: String::new(),
            add_content_buffer: String::new(),
            new_scene_buffer: String::new(),
            new_level_buffer: String::new(),
            new_resource_buffer: String::new(),
            selected_scene: Some(0),
            selected_level: Some(0),
            selected_resource: Some(0),
            status_text: "Ready".to_owned(),
            show_about_window: false,
            show_developer_window: false,
            show_documentation_window: false,
            show_help_info_window: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            theme: "Dark".to_string(),
        }
    }
}

impl UiState {
    pub fn theme_name(&self) -> &str {
        &self.theme
    }

    pub fn set_theme(&mut self, theme: &str) {
        self.theme = theme.to_string();
        // Theme application logic will be handled in the main app update loop
    }
    pub fn enqueue(&mut self, cmd: EditorCommand) {
        self.pending_commands.push(cmd);
    }

    pub fn sanitize_selection(&mut self, project: &ProjectState) {
        if let Some(idx) = self.selected_actor {
            if idx >= project.actors.len() {
                self.selected_actor = project.actors.len().checked_sub(1);
            }
        }
        if let Some(idx) = self.selected_content {
            if idx >= project.content_items.len() {
                self.selected_content = project.content_items.len().checked_sub(1);
            }
        }
        if let Some(idx) = self.selected_scene {
            if idx >= project.scenes.len() {
                self.selected_scene = project.scenes.len().checked_sub(1);
            }
        }
        if let Some(idx) = self.selected_level {
            if idx >= project.levels.len() {
                self.selected_level = project.levels.len().checked_sub(1);
            }
        }
        if let Some(idx) = self.selected_resource {
            if idx >= project.resources.len() {
                self.selected_resource = project.resources.len().checked_sub(1);
            }
        }
    }

    pub fn reset_layout(&mut self) {
        self.show_blueprint = true;
        self.show_outliner = true;
        self.show_details = true;
        self.show_content_browser = true;
        self.show_output_log = true;
        self.show_settings = false;
    }

    pub fn push_undo_snapshot(&mut self, project: &ProjectState) {
        self.undo_stack.push(project.clone());
        if self.undo_stack.len() > 40 {
            let trim = self.undo_stack.len() - 40;
            self.undo_stack.drain(0..trim);
        }
        self.redo_stack.clear();
    }
}

pub fn draw_docked_layout(ctx: &egui::Context, app: &mut EditorApp) {
    dock::draw(ctx, app);
    // Show settings panel as a modal window if requested
    if app.ui_state.show_settings {
        egui::Window::new("Settings")
            .open(&mut app.ui_state.show_settings)
            .resizable(true)
            .show(ctx, |ui| {
                crate::ui::panels::settings::draw(ui, app);
            });
    }
}

pub fn draw_blueprint_window(ctx: &egui::Context, app: &mut EditorApp) {
    blueprint::draw(ctx, app);
}

pub fn draw_help_windows(ctx: &egui::Context, app: &mut EditorApp) {
    help::draw(ctx, app);
}
