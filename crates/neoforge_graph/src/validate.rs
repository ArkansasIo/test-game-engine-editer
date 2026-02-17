use std::collections::{HashMap, HashSet, VecDeque};

use crate::{model::{Graph, ValueType}, registry::NodeRegistry};

#[derive(thiserror::Error, Debug)]
pub enum GraphError {
    #[error("unknown node type: {0}")]
    UnknownNodeType(String),
    #[error("unknown pin: {0}")]
    UnknownPin(String),
    #[error("type mismatch: {0}")]
    TypeMismatch(String),
    #[error("cycle detected")]
    Cycle,
}

pub fn validate(graph: &Graph, reg: &NodeRegistry) -> Result<(), GraphError> {
    let mut node_type: HashMap<uuid::Uuid, &str> = HashMap::new();
    for n in &graph.nodes {
        if reg.get(&n.type_id).is_none() {
            return Err(GraphError::UnknownNodeType(n.type_id.clone()));
        }
        node_type.insert(n.id, &n.type_id);
    }

    // Pin checks + type compatibility
    for e in &graph.edges {
        let src_def = reg.get(node_type.get(&e.src).ok_or_else(|| GraphError::UnknownPin("src node missing".into()))?) .unwrap();
        let dst_def = reg.get(node_type.get(&e.dst).ok_or_else(|| GraphError::UnknownPin("dst node missing".into()))?) .unwrap();

        let src_pin = src_def.outputs.iter().find(|p| p.id == e.src_pin).ok_or_else(|| GraphError::UnknownPin(format!("src pin {}", e.src_pin)))?;
        let dst_pin = dst_def.inputs.iter().find(|p| p.id == e.dst_pin).ok_or_else(|| GraphError::UnknownPin(format!("dst pin {}", e.dst_pin)))?;

        if !types_compatible(&src_pin.ty, &dst_pin.ty) {
            return Err(GraphError::TypeMismatch(format!("{:?} -> {:?}", src_pin.ty, dst_pin.ty)));
        }
    }

    // Cycle detection on dependency graph: src -> dst edges
    let mut indeg: HashMap<uuid::Uuid, usize> = HashMap::new();
    let mut adj: HashMap<uuid::Uuid, Vec<uuid::Uuid>> = HashMap::new();
    for n in &graph.nodes {
        indeg.insert(n.id, 0);
        adj.insert(n.id, vec![]);
    }
    for e in &graph.edges {
        adj.get_mut(&e.src).map(|v| v.push(e.dst));
        *indeg.entry(e.dst).or_insert(0) += 1;
    }

    let mut q = VecDeque::new();
    for (&id, &d) in &indeg {
        if d == 0 { q.push_back(id); }
    }
    let mut seen = 0usize;
    while let Some(n) = q.pop_front() {
        seen += 1;
        for &m in adj.get(&n).unwrap_or(&vec![]) {
            if let Some(d) = indeg.get_mut(&m) {
                *d -= 1;
                if *d == 0 { q.push_back(m); }
            }
        }
    }
    if seen != graph.nodes.len() {
        return Err(GraphError::Cycle);
    }

    Ok(())
}

fn types_compatible(a: &ValueType, b: &ValueType) -> bool {
    if a == b { return true; }
    matches!((a,b),
        (ValueType::Any, _) | (_, ValueType::Any) |
        (ValueType::Int, ValueType::Float)
    )
}
