use std::collections::HashMap;

use neoforge_core::log::LogBuffer;

pub struct HostContext {
    pub log: LogBuffer,
}

impl Default for HostContext {
    fn default() -> Self {
        Self { log: LogBuffer::default() }
    }
}

pub trait NodeExecutor: Send + Sync {
    fn evaluate(
        &self,
        ctx: &mut HostContext,
        node_data: &serde_json::Value,
        inputs: &HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, serde_json::Value>, String>;
}
