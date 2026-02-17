use std::collections::HashMap;

use neoforge_graph::{model::Graph, registry::NodeRegistry, validate};
use neoforge_plugins::{HostContext, NodeExecutor};

#[derive(thiserror::Error, Debug)]
pub enum VmError {
    #[error(transparent)]
    Graph(#[from] validate::GraphError),
    #[error("missing executor for node type: {0}")]
    MissingExecutor(String),
    #[error("execution error: {0}")]
    Exec(String),
}

#[derive(Default)]
pub struct Vm {
    pub registry: NodeRegistry,
    executors: HashMap<String, Box<dyn NodeExecutor>>,
}

impl Vm {
    pub fn with_builtin() -> Self {
        let mut vm = Vm { registry: NodeRegistry::with_builtin(), executors: HashMap::new() };
        vm.register_executor("core.print", Box::new(builtin::PrintExecutor));
        vm.register_executor("core.const_string", Box::new(builtin::ConstStringExecutor));
        vm.register_executor("town.generate", Box::new(builtin::TownGenerateExecutor));
        vm
    }

    pub fn register_executor(&mut self, type_id: &str, exec: Box<dyn NodeExecutor>) {
        self.executors.insert(type_id.to_string(), exec);
    }

    /// Executes a *dataflow* graph: evaluates nodes in topological order.
    /// MVP: assumes each node outputs are computed from connected inputs; no branching/loops.
    pub fn run(&self, graph: &Graph, ctx: &mut HostContext) -> Result<HashMap<(uuid::Uuid, String), serde_json::Value>, VmError> {
        validate::validate(graph, &self.registry)?;

        // Kahn topological sort using edges src->dst
        let mut indeg: HashMap<uuid::Uuid, usize> = HashMap::new();
        let mut adj: HashMap<uuid::Uuid, Vec<uuid::Uuid>> = HashMap::new();
        for n in &graph.nodes { indeg.insert(n.id, 0); adj.insert(n.id, vec![]); }
        for e in &graph.edges {
            adj.get_mut(&e.src).map(|v| v.push(e.dst));
            *indeg.entry(e.dst).or_insert(0) += 1;
        }
        let mut q: std::collections::VecDeque<uuid::Uuid> = indeg.iter().filter(|(_,d)| **d==0).map(|(k,_)| *k).collect();
        let mut order = Vec::new();
        while let Some(n) = q.pop_front() {
            order.push(n);
            for &m in adj.get(&n).unwrap_or(&vec![]) {
                if let Some(d) = indeg.get_mut(&m) {
                    *d -= 1;
                    if *d == 0 { q.push_back(m); }
                }
            }
        }

        // Build lookup for edges incoming per dst
        let mut incoming: HashMap<uuid::Uuid, Vec<&neoforge_graph::model::Edge>> = HashMap::new();
        for e in &graph.edges {
            incoming.entry(e.dst).or_default().push(e);
        }

        // Values keyed by (node_id, output_pin_id)
        let mut values: HashMap<(uuid::Uuid, String), serde_json::Value> = HashMap::new();

        for node_id in order {
            let node = graph.nodes.iter().find(|n| n.id == node_id).unwrap();
            let def = self.registry.get(&node.type_id).unwrap();
            let exec = self.executors.get(&node.type_id).ok_or_else(|| VmError::MissingExecutor(node.type_id.clone()))?;

            // Gather inputs from connected edges
            let mut inputs: HashMap<String, serde_json::Value> = HashMap::new();
            if let Some(inc) = incoming.get(&node_id) {
                for e in inc {
                    let k = (e.src, e.src_pin.clone());
                    if let Some(v) = values.get(&k) {
                        inputs.insert(e.dst_pin.clone(), v.clone());
                    }
                }
            }
            // Default missing inputs
            for pin in &def.inputs {
                inputs.entry(pin.id.clone()).or_insert(serde_json::Value::Null);
            }

            let outputs = exec.evaluate(ctx, &node.data, &inputs)
                .map_err(|e| VmError::Exec(e))?;

            for (pin, v) in outputs {
                values.insert((node_id, pin), v);
            }
        }

        Ok(values)
    }
}

pub mod builtin {
    use std::collections::HashMap;
    use neoforge_plugins::{HostContext, NodeExecutor};

    pub struct PrintExecutor;
    impl NodeExecutor for PrintExecutor {
        fn evaluate(&self, ctx: &mut HostContext, _node_data: &serde_json::Value, inputs: &HashMap<String, serde_json::Value>)
            -> Result<HashMap<String, serde_json::Value>, String> {
            let msg = inputs.get("msg").and_then(|v| v.as_str()).unwrap_or("(null)");
            ctx.log.push(format!("[Print] {}", msg));
            Ok(HashMap::new())
        }
    }

    pub struct ConstStringExecutor;
    impl NodeExecutor for ConstStringExecutor {
        fn evaluate(&self, _ctx: &mut HostContext, node_data: &serde_json::Value, _inputs: &HashMap<String, serde_json::Value>)
            -> Result<HashMap<String, serde_json::Value>, String> {
            let s = node_data.get("value").and_then(|v| v.as_str()).unwrap_or("Hello");
            Ok(HashMap::from([("value".into(), serde_json::Value::String(s.to_string()))]))
        }
    }

    pub struct TownGenerateExecutor;
    impl NodeExecutor for TownGenerateExecutor {
        fn evaluate(&self, ctx: &mut HostContext, _node_data: &serde_json::Value, inputs: &HashMap<String, serde_json::Value>)
            -> Result<HashMap<String, serde_json::Value>, String> {
            let seed = inputs.get("seed").and_then(|v| v.as_i64()).unwrap_or(1);
            let mode = inputs.get("mode").and_then(|v| v.as_str()).unwrap_or("2d");
            // Placeholder output; real TownGeneratorOS bridge belongs in towngen_adapter crate.
            let town = serde_json::json!({
                "seed": seed,
                "mode": mode,
                "roads": [{"x1":0,"y1":0,"x2":10,"y2":0}],
                "lots": [{"x":2,"y":1,"w":3,"h":2}]
            });
            ctx.log.push(format!("[Town] Generated stub town (mode={}, seed={})", mode, seed));
            Ok(HashMap::from([("town".into(), town)]))
        }
    }
}
