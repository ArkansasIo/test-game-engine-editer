// Types: Enums, structs, traits, interfaces for editor concepts

// Enums
pub enum EEditorTabId {
    ContentBrowser,
    Outliner,
    Details,
    Viewport,
    OutputLog,
}

pub enum EAssetType {
    StaticMesh,
    Material,
    Blueprint,
    Texture,
}

// Structs
pub struct FAppConfig {
    pub recent_projects: Vec<String>,
}

pub struct FCommandInfo {
    pub id: String,
    pub label: String,
    pub tooltip: String,
    pub shortcut: Option<String>,
}

// Traits/Interfaces
pub trait ITabSpawner {
    fn spawn_tab(&self, tab_id: EEditorTabId);
}

pub trait ICommandTarget {
    fn can_execute(&self, command: &FCommandInfo) -> bool;
    fn execute(&self, command: &FCommandInfo);
}
