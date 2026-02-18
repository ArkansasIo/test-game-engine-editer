use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SourceFileEntry {
    pub relative_path: String,
}

#[derive(Debug, Clone)]
pub struct SourceIndex {
    pub root_label: &'static str,
    pub root_path: PathBuf,
    pub files: Vec<SourceFileEntry>,
}

pub fn town_generator_os_sources() -> SourceIndex {
    build_index("TownGeneratorOS", workspace_root().join("TownGeneratorOS-master"))
}

pub fn eldiron_sources() -> SourceIndex {
    build_index("Eldiron", workspace_root().join("Eldiron-master"))
}

fn build_index(label: &'static str, root_path: PathBuf) -> SourceIndex {
    let mut files = Vec::new();
    if root_path.exists() {
        collect_files(&root_path, &root_path, &mut files);
        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    }
    SourceIndex {
        root_label: label,
        root_path,
        files,
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .to_path_buf()
}

fn collect_files(root: &Path, current: &Path, out: &mut Vec<SourceFileEntry>) {
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, out);
        } else if path.is_file() {
            let Ok(rel) = path.strip_prefix(root) else {
                continue;
            };
            out.push(SourceFileEntry {
                relative_path: rel.to_string_lossy().replace('\\', "/"),
            });
        }
    }
}
