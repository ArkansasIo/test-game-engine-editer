use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueType {
    Bool,
    Int,
    Float,
    String,
    Vec2,
    Vec3,
    Json,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinDef {
    pub id: String,
    pub name: String,
    pub ty: ValueType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDef {
    pub type_id: String,
    pub display_name: String,
    pub inputs: Vec<PinDef>,
    pub outputs: Vec<PinDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInstance {
    pub id: Uuid,
    pub type_id: String,
    pub pos: [f32; 2],
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub src: Uuid,
    pub src_pin: String,
    pub dst: Uuid,
    pub dst_pin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub version: u32,
    pub nodes: Vec<NodeInstance>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn empty() -> Self {
        Self { version: 1, nodes: vec![], edges: vec![] }
    }
}
