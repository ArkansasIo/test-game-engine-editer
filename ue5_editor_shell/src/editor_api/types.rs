#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EEditorTabId {
    LevelEditor,
    BlueprintEditor,
    MaterialEditor,
    NiagaraEditor,
    AnimationControlRig,
    Sequencer,
    ModelingTools,
    ContentBrowser,
    WorldOutliner,
    Details,
    OutputLog,
    MessageLog,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EAssetType {
    StaticMesh,
    Material,
    Blueprint,
    Texture,
    Audio,
    Level,
    NiagaraSystem,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ESelectionKind {
    Actor,
    Component,
    Asset,
    Node,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ETransformGizmoMode {
    Translate,
    Rotate,
    Scale,
    Universal,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ESnapMode {
    Grid,
    Angle,
    Surface,
    Vertex,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EBuildTask {
    Lighting,
    Geometry,
    Paths,
    Hlods,
    ReflectionCaptures,
    Navigation,
    Shaders,
    Cook,
    Package,
    All,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EViewportViewMode {
    Lit,
    Unlit,
    Wireframe,
    DetailLighting,
    BufferVisualization,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ELayoutPreset {
    Default,
    Animation,
    Modeling,
    Debug,
}

#[derive(Clone, Debug)]
pub struct FCommandInfo {
    pub id: &'static str,
    pub label: &'static str,
    pub tooltip: &'static str,
    pub shortcut: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct FBuildProgress {
    pub active: Option<EBuildTask>,
    pub completed_steps: u32,
    pub total_steps: u32,
}

#[derive(Clone, Debug)]
pub struct FSourceControlState {
    pub connected: bool,
    pub provider_name: String,
    pub checked_out_files: u32,
}

impl Default for FBuildProgress {
    fn default() -> Self {
        Self {
            active: None,
            completed_steps: 0,
            total_steps: 0,
        }
    }
}

impl Default for FSourceControlState {
    fn default() -> Self {
        Self {
            connected: false,
            provider_name: "None".to_owned(),
            checked_out_files: 0,
        }
    }
}
