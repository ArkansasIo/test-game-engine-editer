use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}};
use uuid::Uuid;

use crate::log::LogBuffer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub id: Uuid,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub engine: EngineInfo,
}

#[derive(thiserror::Error, Debug)]
pub enum ProjectError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid project: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone)]
pub struct Project {
    pub root: PathBuf,
    pub meta: ProjectMeta,
}

impl Project {
    pub const META_FILE: &'static str = "Project.neoforge.json";

    pub fn create(root: impl AsRef<Path>, name: &str, log: &LogBuffer) -> Result<Project, ProjectError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root)?;
        for d in ["Content", "Maps", "Graphs", "Plugins", "Saved"] {
            fs::create_dir_all(root.join(d))?;
        }

        let now = chrono::Utc::now().to_rfc3339();
        let meta = ProjectMeta {
            id: Uuid::new_v4(),
            name: name.to_string(),
            created_at: now.clone(),
            updated_at: now.clone(),
            engine: EngineInfo { name: "neoforge".into(), version: "0.1.0".into() },
        };
        let meta_path = root.join(Self::META_FILE);
        fs::write(&meta_path, serde_json::to_vec_pretty(&meta)?)?;
        log.push(format!("Created project '{}' at {}", name, root.display()));
        Ok(Project { root, meta })
    }

    pub fn open(root: impl AsRef<Path>, log: &LogBuffer) -> Result<Project, ProjectError> {
        let root = root.as_ref().to_path_buf();
        let meta_path = root.join(Self::META_FILE);
        if !meta_path.exists() {
            return Err(ProjectError::Invalid(format!("missing {}", Self::META_FILE)));
        }
        let meta: ProjectMeta = serde_json::from_slice(&fs::read(&meta_path)?)?;
        log.push(format!("Opened project '{}' at {}", meta.name, root.display()));
        Ok(Project { root, meta })
    }

    pub fn graphs_dir(&self) -> PathBuf { self.root.join("Graphs") }
    pub fn content_dir(&self) -> PathBuf { self.root.join("Content") }
}

