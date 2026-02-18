use crate::app::EditorApp;

pub fn draw_blueprint_editor(ui: &mut egui::Ui, app: &mut EditorApp) {
    // Header row like UE blueprint editor: Graph / Functions / Macros / Variables
    ui.horizontal(|ui| {
        ui.add(egui::Button::new("Graph"));
        ui.add(egui::Button::new("Functions"));
        ui.add(egui::Button::new("Macros"));
        ui.add(egui::Button::new("Variables"));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("✖").clicked() {
                app.ui_state.show_blueprint = false;
                app.project.log("[UI] Blueprint window closed");
            }
        });
    });
    ui.separator();
    // Main blueprint area: left list + graph canvas
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.set_min_width(200.0);
            ui.label(egui::RichText::new("My Blueprint").strong());
            ui.separator();
            selectable(ui, "Graph", true);
            selectable(ui, "Functions", false);
            selectable(ui, "Macros", false);
            selectable(ui, "Variables", false);
            ui.add_space(10.0);
            ui.label(egui::RichText::new("Details").strong());
            ui.separator();
            ui.label(egui::RichText::new("Selected: Event Graph (mock)").weak());
        });
        ui.separator();
        // Graph canvas
        {
            let (rect, resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
            let painter = ui.painter();
            // Dark grid canvas
            painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(18, 19, 22));
            let grid = egui::Color32::from_rgb(40, 42, 46);
            let step = 28.0;
            let mut x = rect.left();
            while x < rect.right() {
                painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], egui::Stroke::new(1.0, grid));
                x += step;
            }
            let mut y = rect.top();
            while y < rect.bottom() {
                painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], egui::Stroke::new(1.0, grid));
                y += step;
            }
            // Event node mock
            let node_size = egui::vec2(160.0, 44.0);
            let node_pos = rect.left_top() + egui::vec2(120.0, 120.0);
            let node_rect = egui::Rect::from_min_size(node_pos, node_size);
            painter.rect_filled(node_rect, 8.0, egui::Color32::from_rgb(120, 65, 55));
            painter.rect_stroke(node_rect, 8.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 140, 120)));
            painter.text(
                node_rect.center(),
                egui::Align2::CENTER_CENTER,
                "Event Graph",
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::from_rgb(240, 240, 240),
            );
            if resp.double_clicked() {
                app.project.log("[Blueprint] Canvas double-click (stub)");
            }
        }
    });
}

fn selectable(ui: &mut egui::Ui, label: &str, selected: bool) {
    let _ = ui.selectable_label(selected, label);
}
