use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorEntry {
    pub name: String,
    pub kind: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub project_name: String, // World/scene model (minimal stub)
    pub actors: Vec<ActorEntry>, // Content browser (minimal stub)
    pub assets: Vec<String>, // Output log
    pub log_entries: Vec<String>,
}

impl Default for ProjectState {
    fn default() -> Self {
        Self {
            project_name: "ExampleMap".to_string(),
            actors: vec![
                ActorEntry { name: "Light".into(), kind: "DirectionalLight".into(), selected: false },
                ActorEntry { name: "SkySphere".into(), kind: "Sky".into(), selected: false },
                ActorEntry { name: "PlayerStart".into(), kind: "Spawn".into(), selected: false },
                ActorEntry { name: "Floor".into(), kind: "StaticMeshActor".into(), selected: true },
            ],
            assets: vec![
                "SM_Chair".into(),
                "SM_Table".into(),
                "SM_Rock".into(),
            ],
            log_entries: vec![
                "[LogEditor] Lighting Build Complete".into(),
                "[LogAsset] Imported New Mesh 'SM_Chair'".into(),
                "[LogBlueprint] Compile Successful".into(),
            ],
        }
    }
}
