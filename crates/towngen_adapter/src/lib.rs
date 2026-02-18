use serde::{Deserialize, Serialize};
pub mod sources;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownRequest {
    pub seed: i64,
    pub mode: String, // "2d" or "3d"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownResult {
    pub payload: serde_json::Value,
}

/// Adapter boundary for TownGeneratorOS.
/// MVP returns stubbed output; replace with CLI/WASM bridge later.
pub fn generate(req: TownRequest) -> TownResult {
    let town_sources = sources::town_generator_os_sources();
    let eldiron_sources = sources::eldiron_sources();
    let payload = serde_json::json!({
        "seed": req.seed,
        "mode": req.mode,
        "note": "stub output; integrate TownGeneratorOS here",
        "sources": {
            "town_generator_os": {
                "root": town_sources.root_path.to_string_lossy(),
                "file_count": town_sources.files.len(),
                "sample": town_sources.files.iter().take(5).map(|f| f.relative_path.clone()).collect::<Vec<_>>()
            },
            "eldiron": {
                "root": eldiron_sources.root_path.to_string_lossy(),
                "file_count": eldiron_sources.files.len(),
                "sample": eldiron_sources.files.iter().take(5).map(|f| f.relative_path.clone()).collect::<Vec<_>>()
            }
        }
    });
    TownResult { payload }
}
