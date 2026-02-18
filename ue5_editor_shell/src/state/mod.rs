#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Select,
    Landscape,
    Foliage,
    Modeling,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Lit,
    Unlit,
    Wireframe,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContentKind {
    StaticMesh,
    Material,
    Blueprint,
    Texture,
}

#[derive(Clone, Copy)]
pub enum MenuOptionKey {
    FileAutoSaveOnBuild,
    FileConfirmOnExit,
    EditMultiClipboard,
    EditTransactionHistory,
    WindowRestoreLastLayout,
    WindowOpenTabsForeground,
    ToolsExperimental,
    ToolsAutoNavmesh,
    BuildIncludeShaders,
    BuildIncremental,
    SelectHiddenActors,
    SelectStrictTypeFilter,
    ActorSnapOnSpawn,
    ActorAutoGroupDuplicates,
    ComponentsShowIcons,
    ComponentsAllowDynamicAdd,
    LevelWorldPartition,
    LevelDataLayers,
    BlueprintLiveCompile,
    BlueprintBreakOnError,
    MaterialsRealtimePreview,
    MaterialsAutoCompileFx,
    CinematicsAutoKeying,
    CinematicsLockCamera,
    PlayStartInSimulate,
    PlayMultiplayerPie,
    HelpTipsOnStartup,
    HelpUsageAnalytics,
}

#[derive(Clone)]
pub struct MenuOptions {
    pub file_auto_save_on_build: bool,
    pub file_confirm_on_exit: bool,
    pub edit_multi_clipboard: bool,
    pub edit_transaction_history: bool,
    pub window_restore_last_layout: bool,
    pub window_open_tabs_foreground: bool,
    pub tools_experimental: bool,
    pub tools_auto_navmesh: bool,
    pub build_include_shaders: bool,
    pub build_incremental: bool,
    pub select_hidden_actors: bool,
    pub select_strict_type_filter: bool,
    pub actor_snap_on_spawn: bool,
    pub actor_auto_group_duplicates: bool,
    pub components_show_icons: bool,
    pub components_allow_dynamic_add: bool,
    pub level_world_partition: bool,
    pub level_data_layers: bool,
    pub blueprint_live_compile: bool,
    pub blueprint_break_on_error: bool,
    pub materials_realtime_preview: bool,
    pub materials_auto_compile_fx: bool,
    pub cinematics_auto_keying: bool,
    pub cinematics_lock_camera: bool,
    pub play_start_in_simulate: bool,
    pub play_multiplayer_pie: bool,
    pub help_tips_on_startup: bool,
    pub help_usage_analytics: bool,
    pub play_client_count: u32,
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Transform {
    pub location: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

#[derive(Clone)]
pub struct SceneActor {
    pub id: u32,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub transform: Transform,
}

#[derive(Clone)]
pub struct ContentItem {
    pub name: String,
    pub kind: ContentKind,
}

#[derive(Clone)]
pub struct EditorSettings {
    pub show_grid: bool,
    pub realtime_viewport: bool,
    pub autosave_enabled: bool,
    pub autosave_minutes: u32,
    pub snap_enabled: bool,
    pub snap_value: f32,
}

#[derive(Clone)]
pub struct RuntimeStats {
    pub shader_jobs: u32,
    pub fps: u32,
}

#[derive(Clone)]
pub struct ProjectState {
    pub project_name: String,
    pub current_map: String,
    pub mode: EditorMode,
    pub is_playing: bool,
    pub dirty: bool,
    pub actors: Vec<SceneActor>,
    pub content_items: Vec<ContentItem>,
    pub output_log: Vec<String>,
    pub settings: EditorSettings,
    pub stats: RuntimeStats,
    pub view_mode: ViewMode,
    pub menu_options: MenuOptions,
    next_actor_id: u32,
}

impl Default for ProjectState {
    fn default() -> Self {
        Self {
            project_name: "Example Project".to_owned(),
            current_map: "ExampleMap".to_owned(),
            mode: EditorMode::Select,
            is_playing: false,
            dirty: false,
            actors: vec![
                SceneActor::new(1, "Light"),
                SceneActor::new(2, "SkySphere"),
                SceneActor::new(3, "PlayerStart"),
                SceneActor::new(4, "Floor"),
            ],
            content_items: vec![
                ContentItem::new("SM_Chair", ContentKind::StaticMesh),
                ContentItem::new("SM_Table", ContentKind::StaticMesh),
                ContentItem::new("SM_Rock", ContentKind::StaticMesh),
                ContentItem::new("M_Metal", ContentKind::Material),
                ContentItem::new("M_Wood", ContentKind::Material),
                ContentItem::new("BP_Door", ContentKind::Blueprint),
                ContentItem::new("T_Concrete", ContentKind::Texture),
            ],
            output_log: vec![
                "[LogEditor] Lighting Build Complete".to_owned(),
                "[LogAsset] Imported New Mesh \"SM_Chair\".".to_owned(),
                "[LogBlueprint] Compile Successful".to_owned(),
            ],
            settings: EditorSettings {
                show_grid: true,
                realtime_viewport: true,
                autosave_enabled: true,
                autosave_minutes: 5,
                snap_enabled: true,
                snap_value: 10.0,
            },
            stats: RuntimeStats {
                shader_jobs: 23,
                fps: 120,
            },
            view_mode: ViewMode::Lit,
            menu_options: MenuOptions::default(),
            next_actor_id: 5,
        }
    }
}

impl Default for MenuOptions {
    fn default() -> Self {
        Self {
            file_auto_save_on_build: true,
            file_confirm_on_exit: true,
            edit_multi_clipboard: false,
            edit_transaction_history: true,
            window_restore_last_layout: true,
            window_open_tabs_foreground: true,
            tools_experimental: false,
            tools_auto_navmesh: true,
            build_include_shaders: true,
            build_incremental: true,
            select_hidden_actors: false,
            select_strict_type_filter: false,
            actor_snap_on_spawn: true,
            actor_auto_group_duplicates: false,
            components_show_icons: true,
            components_allow_dynamic_add: true,
            level_world_partition: true,
            level_data_layers: true,
            blueprint_live_compile: true,
            blueprint_break_on_error: false,
            materials_realtime_preview: true,
            materials_auto_compile_fx: true,
            cinematics_auto_keying: false,
            cinematics_lock_camera: false,
            play_start_in_simulate: false,
            play_multiplayer_pie: false,
            help_tips_on_startup: true,
            help_usage_analytics: false,
            play_client_count: 1,
        }
    }
}

impl SceneActor {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_owned(),
            visible: true,
            locked: false,
            transform: Transform::default(),
        }
    }
}

impl ContentItem {
    pub fn new(name: &str, kind: ContentKind) -> Self {
        Self {
            name: name.to_owned(),
            kind,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            location: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 120.0,
            },
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }
}

impl ProjectState {
    pub fn log(&mut self, line: impl Into<String>) {
        self.output_log.push(line.into());
        if self.output_log.len() > 300 {
            let trim = self.output_log.len() - 300;
            self.output_log.drain(0..trim);
        }
    }

    pub fn add_actor(&mut self, base_name: &str) -> u32 {
        let id = self.next_actor_id;
        self.next_actor_id += 1;
        self.actors.push(SceneActor::new(id, base_name));
        self.dirty = true;
        id
    }

    pub fn menu_option(&self, key: MenuOptionKey) -> bool {
        match key {
            MenuOptionKey::FileAutoSaveOnBuild => self.menu_options.file_auto_save_on_build,
            MenuOptionKey::FileConfirmOnExit => self.menu_options.file_confirm_on_exit,
            MenuOptionKey::EditMultiClipboard => self.menu_options.edit_multi_clipboard,
            MenuOptionKey::EditTransactionHistory => self.menu_options.edit_transaction_history,
            MenuOptionKey::WindowRestoreLastLayout => self.menu_options.window_restore_last_layout,
            MenuOptionKey::WindowOpenTabsForeground => self.menu_options.window_open_tabs_foreground,
            MenuOptionKey::ToolsExperimental => self.menu_options.tools_experimental,
            MenuOptionKey::ToolsAutoNavmesh => self.menu_options.tools_auto_navmesh,
            MenuOptionKey::BuildIncludeShaders => self.menu_options.build_include_shaders,
            MenuOptionKey::BuildIncremental => self.menu_options.build_incremental,
            MenuOptionKey::SelectHiddenActors => self.menu_options.select_hidden_actors,
            MenuOptionKey::SelectStrictTypeFilter => self.menu_options.select_strict_type_filter,
            MenuOptionKey::ActorSnapOnSpawn => self.menu_options.actor_snap_on_spawn,
            MenuOptionKey::ActorAutoGroupDuplicates => self.menu_options.actor_auto_group_duplicates,
            MenuOptionKey::ComponentsShowIcons => self.menu_options.components_show_icons,
            MenuOptionKey::ComponentsAllowDynamicAdd => self.menu_options.components_allow_dynamic_add,
            MenuOptionKey::LevelWorldPartition => self.menu_options.level_world_partition,
            MenuOptionKey::LevelDataLayers => self.menu_options.level_data_layers,
            MenuOptionKey::BlueprintLiveCompile => self.menu_options.blueprint_live_compile,
            MenuOptionKey::BlueprintBreakOnError => self.menu_options.blueprint_break_on_error,
            MenuOptionKey::MaterialsRealtimePreview => self.menu_options.materials_realtime_preview,
            MenuOptionKey::MaterialsAutoCompileFx => self.menu_options.materials_auto_compile_fx,
            MenuOptionKey::CinematicsAutoKeying => self.menu_options.cinematics_auto_keying,
            MenuOptionKey::CinematicsLockCamera => self.menu_options.cinematics_lock_camera,
            MenuOptionKey::PlayStartInSimulate => self.menu_options.play_start_in_simulate,
            MenuOptionKey::PlayMultiplayerPie => self.menu_options.play_multiplayer_pie,
            MenuOptionKey::HelpTipsOnStartup => self.menu_options.help_tips_on_startup,
            MenuOptionKey::HelpUsageAnalytics => self.menu_options.help_usage_analytics,
        }
    }

    pub fn set_menu_option(&mut self, key: MenuOptionKey, value: bool) {
        match key {
            MenuOptionKey::FileAutoSaveOnBuild => self.menu_options.file_auto_save_on_build = value,
            MenuOptionKey::FileConfirmOnExit => self.menu_options.file_confirm_on_exit = value,
            MenuOptionKey::EditMultiClipboard => self.menu_options.edit_multi_clipboard = value,
            MenuOptionKey::EditTransactionHistory => self.menu_options.edit_transaction_history = value,
            MenuOptionKey::WindowRestoreLastLayout => self.menu_options.window_restore_last_layout = value,
            MenuOptionKey::WindowOpenTabsForeground => self.menu_options.window_open_tabs_foreground = value,
            MenuOptionKey::ToolsExperimental => self.menu_options.tools_experimental = value,
            MenuOptionKey::ToolsAutoNavmesh => self.menu_options.tools_auto_navmesh = value,
            MenuOptionKey::BuildIncludeShaders => self.menu_options.build_include_shaders = value,
            MenuOptionKey::BuildIncremental => self.menu_options.build_incremental = value,
            MenuOptionKey::SelectHiddenActors => self.menu_options.select_hidden_actors = value,
            MenuOptionKey::SelectStrictTypeFilter => self.menu_options.select_strict_type_filter = value,
            MenuOptionKey::ActorSnapOnSpawn => self.menu_options.actor_snap_on_spawn = value,
            MenuOptionKey::ActorAutoGroupDuplicates => self.menu_options.actor_auto_group_duplicates = value,
            MenuOptionKey::ComponentsShowIcons => self.menu_options.components_show_icons = value,
            MenuOptionKey::ComponentsAllowDynamicAdd => self.menu_options.components_allow_dynamic_add = value,
            MenuOptionKey::LevelWorldPartition => self.menu_options.level_world_partition = value,
            MenuOptionKey::LevelDataLayers => self.menu_options.level_data_layers = value,
            MenuOptionKey::BlueprintLiveCompile => self.menu_options.blueprint_live_compile = value,
            MenuOptionKey::BlueprintBreakOnError => self.menu_options.blueprint_break_on_error = value,
            MenuOptionKey::MaterialsRealtimePreview => self.menu_options.materials_realtime_preview = value,
            MenuOptionKey::MaterialsAutoCompileFx => self.menu_options.materials_auto_compile_fx = value,
            MenuOptionKey::CinematicsAutoKeying => self.menu_options.cinematics_auto_keying = value,
            MenuOptionKey::CinematicsLockCamera => self.menu_options.cinematics_lock_camera = value,
            MenuOptionKey::PlayStartInSimulate => self.menu_options.play_start_in_simulate = value,
            MenuOptionKey::PlayMultiplayerPie => self.menu_options.play_multiplayer_pie = value,
            MenuOptionKey::HelpTipsOnStartup => self.menu_options.help_tips_on_startup = value,
            MenuOptionKey::HelpUsageAnalytics => self.menu_options.help_usage_analytics = value,
        }
    }
}

impl ContentKind {
    pub fn label(self) -> &'static str {
        match self {
            ContentKind::StaticMesh => "Static Mesh",
            ContentKind::Material => "Material",
            ContentKind::Blueprint => "Blueprint",
            ContentKind::Texture => "Texture",
        }
    }
}
