mod app;
mod graph_ui;
mod viewport;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "NeoForge Studio (Rust)",
        native_options,
        Box::new(|cc| Box::new(app::EditorApp::new(cc))),
    )
}
