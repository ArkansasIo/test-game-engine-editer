use eframe::egui;
use neoforge_core::log::LogBuffer;
use neoforge_graph::{model::{Graph, NodeInstance, Edge}, registry::NodeRegistry};
use uuid::Uuid;

#[derive(Default)]
pub struct GraphUiState {
    selected_node: Option<Uuid>,
    pending_link_from: Option<(Uuid, String)>, // (node, output pin)
    pan: egui::Vec2,
    zoom: f32,
}

impl GraphUiState {
    pub fn ui(&mut self, ui: &mut egui::Ui, reg: &NodeRegistry, graph: &mut Graph, log: &LogBuffer) {
        ui.horizontal(|ui| {
            if ui.button("+ Node").clicked() {
                self.add_node_popup(ui.ctx(), reg);
            }
            if ui.button("Add: Const String").clicked() {
                let id = Uuid::new_v4();
                graph.nodes.push(NodeInstance{
                    id,
                    type_id: "core.const_string".into(),
                    pos: [120.0, 80.0],
                    data: serde_json::json!({"value":"Hello from Rust"}),
                });
                self.selected_node = Some(id);
            }
            if ui.button("Add: Print").clicked() {
                let id = Uuid::new_v4();
                graph.nodes.push(NodeInstance{
                    id,
                    type_id: "core.print".into(),
                    pos: [420.0, 120.0],
                    data: serde_json::json!({}),
                });
                self.selected_node = Some(id);
            }
            if ui.button("Connect selected (Const->Print)").clicked() {
                // Convenience wiring for demo: connect last const_string to last print
                let s = graph.nodes.iter().rev().find(|n| n.type_id == "core.const_string").map(|n| n.id);
                let p = graph.nodes.iter().rev().find(|n| n.type_id == "core.print").map(|n| n.id);
                if let (Some(s), Some(p)) = (s,p) {
                    graph.edges.push(Edge{ id: Uuid::new_v4(), src: s, src_pin: "value".into(), dst: p, dst_pin: "msg".into()});
                    log.push("Wired Const String -> Print");
                }
            }
        });
        ui.separator();

        // Canvas
        let (rect, resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
        if resp.dragged() {
            self.pan += resp.drag_delta();
        }

        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, ui.visuals().extreme_bg_color);

        // Draw edges first
        for e in &graph.edges {
            let Some(a) = graph.nodes.iter().find(|n| n.id == e.src) else { continue; };
            let Some(b) = graph.nodes.iter().find(|n| n.id == e.dst) else { continue; };
            let ap = to_screen(rect, self.pan, a.pos);
            let bp = to_screen(rect, self.pan, b.pos);
            let a_out = ap + egui::vec2(140.0, 20.0);
            let b_in = bp + egui::vec2(0.0, 20.0);
            // Approximate cubic with polyline for now
            let points = vec![a_out, b_in];
            painter.add(egui::Shape::line(points, egui::Stroke::new(2.0, ui.visuals().widgets.noninteractive.fg_stroke.color)));
        }

        // Draw nodes
        for n in &mut graph.nodes {
            let def = reg.get(&n.type_id);
            let title = def.map(|d| d.display_name.clone()).unwrap_or_else(|| n.type_id.clone());

            let pos = to_screen(rect, self.pan, n.pos);
            let node_rect = egui::Rect::from_min_size(pos, egui::vec2(160.0, 70.0));
            let selected = self.selected_node == Some(n.id);

            let fill = if selected { ui.visuals().selection.bg_fill } else { ui.visuals().widgets.noninteractive.bg_fill };
            painter.rect_filled(node_rect, 8.0, fill);
            painter.rect_stroke(node_rect, 8.0, egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.fg_stroke.color));
            painter.text(node_rect.min + egui::vec2(8.0, 6.0), egui::Align2::LEFT_TOP, title, egui::FontId::proportional(14.0), ui.visuals().text_color());

            // Interaction: click-select + drag
            let id = ui.make_persistent_id(n.id);
            let r = ui.interact(node_rect, id, egui::Sense::click_and_drag());
            if r.clicked() { self.selected_node = Some(n.id); }
            if r.dragged() {
                let d = r.drag_delta();
                n.pos[0] += d.x;
                n.pos[1] += d.y;
            }

            // Mini pin buttons (1 out on right, 1 in on left for MVP)
            if n.type_id == "core.const_string" {
                let pin_rect = egui::Rect::from_center_size(node_rect.right_center() + egui::vec2(-8.0, 0.0), egui::vec2(12.0, 12.0));
                let pr = ui.interact(pin_rect, ui.make_persistent_id((n.id, "out")), egui::Sense::click());
                painter.circle_filled(pin_rect.center(), 5.0, ui.visuals().widgets.noninteractive.fg_stroke.color);
                if pr.clicked() { self.pending_link_from = Some((n.id, "value".into())); }
            }
            if n.type_id == "core.print" {
                let pin_rect = egui::Rect::from_center_size(node_rect.left_center() + egui::vec2(8.0, 0.0), egui::vec2(12.0, 12.0));
                let pr = ui.interact(pin_rect, ui.make_persistent_id((n.id, "in")), egui::Sense::click());
                painter.circle_filled(pin_rect.center(), 5.0, ui.visuals().widgets.noninteractive.fg_stroke.color);
                if pr.clicked() {
                    if let Some((src, src_pin)) = self.pending_link_from.take() {
                        graph.edges.push(Edge{
                            id: Uuid::new_v4(),
                            src,
                            src_pin,
                            dst: n.id,
                            dst_pin: "msg".into(),
                        });
                        log.push("Created link");
                    }
                }
            }
        }
    }

    fn add_node_popup(&self, ctx: &egui::Context, _reg: &NodeRegistry) {
        // MVP placeholder: use quick-add buttons above.
        ctx.debug_painter().text(egui::pos2(8.0, 8.0), egui::Align2::LEFT_TOP, "Use quick-add buttons", egui::FontId::proportional(12.0), egui::Color32::WHITE);
    }
}

fn to_screen(rect: egui::Rect, pan: egui::Vec2, world: [f32;2]) -> egui::Pos2 {
    rect.min + pan + egui::vec2(world[0], world[1])
}
