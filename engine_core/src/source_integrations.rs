use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SourceManifest {
    pub name: &'static str,
    pub root: PathBuf,
    pub files: Vec<String>,
}

pub fn load_all_manifests() -> Vec<SourceManifest> {
    vec![
        manifest_for("Eldiron", workspace_root().join("Eldiron-master")),
        manifest_for(
            "TownGeneratorOS",
            workspace_root().join("TownGeneratorOS-master"),
        ),
    ]
}

pub fn manifest_for(name: &'static str, root: PathBuf) -> SourceManifest {
    let mut files = Vec::new();
    if root.exists() {
        walk(&root, &root, &mut files);
        files.sort();
    }
    SourceManifest { name, root, files }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .to_path_buf()
}

fn walk(root: &Path, current: &Path, out: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(root, &path, out);
        } else if path.is_file() {
            if let Ok(rel) = path.strip_prefix(root) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}
