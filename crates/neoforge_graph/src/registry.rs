use std::collections::HashMap;

use crate::model::{NodeDef, PinDef, ValueType};

#[derive(Default)]
pub struct NodeRegistry {
    defs: HashMap<String, NodeDef>,
}

impl NodeRegistry {
    pub fn new() -> Self { Self { defs: HashMap::new() } }

    pub fn register(&mut self, def: NodeDef) {
        self.defs.insert(def.type_id.clone(), def);
    }

    pub fn get(&self, type_id: &str) -> Option<&NodeDef> {
        self.defs.get(type_id)
    }

    pub fn all(&self) -> Vec<&NodeDef> {
        let mut v: Vec<_> = self.defs.values().collect();
        v.sort_by(|a,b| a.display_name.cmp(&b.display_name));
        v
    }

    pub fn with_builtin() -> Self {
        let mut r = Self::new();

        r.register(NodeDef {
            type_id: "core.print".into(),
            display_name: "Print".into(),
            inputs: vec![PinDef { id: "msg".into(), name: "Message".into(), ty: ValueType::String }],
            outputs: vec![],
        });

        r.register(NodeDef {
            type_id: "core.const_string".into(),
            display_name: "Const String".into(),
            inputs: vec![],
            outputs: vec![PinDef { id: "value".into(), name: "Value".into(), ty: ValueType::String }],
        });

        r.register(NodeDef {
            type_id: "town.generate".into(),
            display_name: "Generate Town".into(),
            inputs: vec![
                PinDef { id: "seed".into(), name: "Seed".into(), ty: ValueType::Int },
                PinDef { id: "mode".into(), name: "Mode".into(), ty: ValueType::String }, // "2d" / "3d"
            ],
            outputs: vec![PinDef { id: "town".into(), name: "Town".into(), ty: ValueType::Json }],
        });

        r
    }
}
