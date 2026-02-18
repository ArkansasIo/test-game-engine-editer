#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Select,
    Landscape,
    Foliage,
    Modeling,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContentKind {
    StaticMesh,
    Material,
    Blueprint,
    Texture,
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
            next_actor_id: 5,
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
