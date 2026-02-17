use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    pub rel_path: String,
    pub abs_path: PathBuf,
    pub kind: String,
}

pub fn index_content_dir(content_dir: impl AsRef<Path>) -> Vec<AssetEntry> {
    let content_dir = content_dir.as_ref();
    if !content_dir.exists() {
        return vec![];
    }
    let mut out = Vec::new();
    for e in WalkDir::new(content_dir).into_iter().filter_map(|e| e.ok()) {
        if !e.file_type().is_file() { continue; }
        let abs = e.path().to_path_buf();
        let rel = abs.strip_prefix(content_dir).ok().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        let ext = abs.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        let kind = match ext.as_str() {
            "png" | "jpg" | "jpeg" | "webp" => "image",
            "json" => "json",
            "glb" | "gltf" => "model",
            "wav" | "ogg" | "mp3" => "audio",
            _ => "file",
        }.to_string();
        out.push(AssetEntry { rel_path: rel, abs_path: abs, kind });
    }
    out.sort_by(|a,b| a.rel_path.cmp(&b.rel_path));
    out
}
