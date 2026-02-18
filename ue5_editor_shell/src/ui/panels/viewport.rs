use crate::state::ProjectState;
use crate::ui::UiState;

pub fn draw(ui: &mut egui::Ui, project: &mut ProjectState, _ui_state: &mut UiState) {
    // Viewport header controls (Perspective / Lit)
    ui.horizontal(|ui| {
        ui.add(egui::Button::new("Perspective ▾"));
        ui.add(egui::Button::new("Lit ▾"));
        ui.separator();
        ui.label(egui::RichText::new(format!("Level: {}", project.project_name)).weak());
    });
    ui.add_space(6.0);
    // Fake 3D viewport area
    let (rect, _resp) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
    let painter = ui.painter();
    // Background grid
    painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(24, 26, 29));
    let grid_color = egui::Color32::from_rgb(45, 48, 52);
    let step = 32.0;
    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], egui::Stroke::new(1.0, grid_color));
        x += step;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], egui::Stroke::new(1.0, grid_color));
        y += step;
    }
    // “Horizon” hint
    painter.text(
        rect.center_top() + egui::vec2(0.0, 24.0),
        egui::Align2::CENTER_TOP,
        "Viewport (mock)",
        egui::TextStyle::Body.resolve(ui.style()),
        egui::Color32::from_rgb(210, 214, 220),
    );
}
