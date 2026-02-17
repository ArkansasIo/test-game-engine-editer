use eframe::egui;

use crate::app::ViewportMode;

#[derive(Default)]
pub struct ViewportState {
    show_grid: bool,
    show_stats: bool,
}

impl ViewportState {
    pub fn ui(&mut self, ui: &mut egui::Ui, mode: ViewportMode) {
        // Header controls are rendered in app; keep viewport body focused.
        let (w, h) = (ui.available_width(), ui.available_height());

        ui.allocate_ui_with_layout(
            egui::vec2(w, h),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                // a simple placeholder "render target"
                let frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgb(20, 22, 26))
                    .inner_margin(egui::Margin::same(10.0))
                    .rounding(egui::Rounding::same(4.0));

                frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.show_grid, "Grid");
                        ui.checkbox(&mut self.show_stats, "Show Stats");
                    });
                    ui.separator();

                    match mode {
                        ViewportMode::Mode2D => {
                            ui.heading("2D Viewport (MVP placeholder)");
                            ui.label("Tilemap + sprite tools are a next milestone.");
                        }
                        ViewportMode::Mode3D => {
                            ui.heading("3D Viewport (MVP placeholder)");
                            ui.label("wgpu renderer + gizmos are a next milestone.");
                        }
                    }
                });
            },
        );
    }
}
