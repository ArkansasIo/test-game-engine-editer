use serde::{Deserialize, Serialize};

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
    let payload = serde_json::json!({
        "seed": req.seed,
        "mode": req.mode,
        "note": "stub output; integrate TownGeneratorOS here"
    });
    TownResult { payload }
}
