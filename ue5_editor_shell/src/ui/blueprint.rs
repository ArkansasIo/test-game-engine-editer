use crate::{actions::commands::EditorCommand, app::EditorApp};

pub fn draw(ctx: &egui::Context, app: &mut EditorApp) {
    if !app.ui_state.show_blueprint {
        return;
    }

    egui::Window::new("Blueprint Editor")
        .default_pos(egui::pos2(460.0, 240.0))
        .default_size(egui::vec2(800.0, 540.0))
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                for t in ["Blueprint", "Editor", "Sequencer", "Material", "FPX"] {
                    ui.add_sized([92.0, 24.0], egui::Button::new(t));
                }
                ui.separator();
                if ui.button("Compile").clicked() {
                    app.ui_state.enqueue(EditorCommand::CompileBlueprints);
                }
            });

            ui.separator();
            ui.columns(3, |columns| {
                let left = &mut columns[0];
                left.label(egui::RichText::new("My Blueprint").strong());
                left.separator();
                for item in ["Graph", "Functions", "Macros", "Variables"] {
                    left.label(item);
                }
                if left.button("+ Function").clicked() {
                    app.ui_state
                        .enqueue(EditorCommand::AddLog("[LogBlueprint] New function created.".to_owned()));
                }
                if left.button("+ Variable").clicked() {
                    app.ui_state
                        .enqueue(EditorCommand::AddLog("[LogBlueprint] New variable created.".to_owned()));
                }

                let graph = &mut columns[1];
                graph.label(egui::RichText::new("Event Graph").strong());
                graph.separator();
                let graph_rect = graph.available_rect_before_wrap();
                let painter = graph.painter();
                painter.rect_filled(graph_rect, 2.0, egui::Color32::from_rgb(26, 28, 31));
                draw_graph_grid(painter, graph_rect, 22.0);

                let node_rect = egui::Rect::from_min_size(
                    graph_rect.center() - egui::vec2(120.0, 46.0),
                    egui::vec2(180.0, 76.0),
                );
                painter.rect_filled(node_rect, 6.0, egui::Color32::from_rgb(169, 85, 42));
                painter.text(
                    node_rect.left_top() + egui::vec2(10.0, 8.0),
                    egui::Align2::LEFT_TOP,
                    "Event BeginPlay",
                    egui::TextStyle::Button.resolve(graph.style()),
                    egui::Color32::WHITE,
                );
                painter.text(
                    node_rect.left_bottom() - egui::vec2(-10.0, 10.0),
                    egui::Align2::LEFT_BOTTOM,
                    "Exec ->",
                    egui::TextStyle::Small.resolve(graph.style()),
                    egui::Color32::WHITE,
                );
                graph.allocate_rect(graph_rect, egui::Sense::hover());

                let right = &mut columns[2];
                right.label(egui::RichText::new("Blueprint Details").strong());
                right.separator();
                right.label("Class Settings");
                right.label("Parent Class: Actor");
                right.separator();
                right.label("Debug");
                right.label("Breakpoints: 0");
                right.label("Warnings: 0");
            });
        });
}

fn draw_graph_grid(painter: &egui::Painter, rect: egui::Rect, spacing: f32) {
    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(42, 44, 48)),
        );
        x += spacing;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(42, 44, 48)),
        );
        y += spacing;
    }
}
