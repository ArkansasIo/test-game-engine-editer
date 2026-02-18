mod app;
mod actions;
mod state;
mod ui;
use app::EditorApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Unreal Engine 5 Editor")
            .with_inner_size([1400.0, 860.0])
            .with_min_inner_size([1100.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Unreal Engine 5 Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(EditorApp::new(cc)))),
    )
}
