//! Lightweight blueprint graph runtime hooks.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlueprintId(pub String);

#[derive(Debug, Clone)]
pub struct BlueprintGraph {
    pub id: BlueprintId,
    pub entry_node: String,
    pub nodes: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct BlueprintSystem {
    graphs: HashMap<BlueprintId, BlueprintGraph>,
}

impl BlueprintSystem {
    pub fn register(&mut self, graph: BlueprintGraph) {
        self.graphs.insert(graph.id.clone(), graph);
    }

    pub fn has(&self, id: &BlueprintId) -> bool {
        self.graphs.contains_key(id)
    }

    pub fn compile(&self, id: &BlueprintId) -> Result<(), String> {
        let Some(graph) = self.graphs.get(id) else {
            return Err(format!("Blueprint {:?} not found", id.0));
        };
        if graph.entry_node.is_empty() {
            return Err(format!("Blueprint {:?} has no entry node", id.0));
        }
        Ok(())
    }
}
