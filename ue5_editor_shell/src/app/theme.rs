/// Theme: dark UI with UE-like accents.
///
/// Where colors live:
/// - BG: primary surfaces
/// - Panel: panels, docks
/// - Stroke: borders
/// - Accent: selections, highlights
///
/// If you want the exact look, tweak these values first.
pub fn apply_editor_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    // --- Typography ---
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(18.0)),
        (egui::TextStyle::Name("TopBar".into()), egui::FontId::proportional(14.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(13.0)),
        (egui::TextStyle::Monospace, egui::FontId::monospace(12.0)),
        (egui::TextStyle::Button, egui::FontId::proportional(13.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(11.0)),
    ].into();
    // --- Spacing / rounding ---
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(8.0);
    style.visuals.window_rounding = egui::Rounding::same(6.0);
    style.visuals.menu_rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);
    // --- Colors (UE-ish dark) ---
    let bg0 = egui::Color32::from_rgb(28, 30, 33); // main background
    let bg1 = egui::Color32::from_rgb(36, 38, 42); // panels
    let bg2 = egui::Color32::from_rgb(46, 49, 54); // raised/toolbar
    let stroke = egui::Color32::from_rgb(70, 74, 80);
    let stroke_soft = egui::Color32::from_rgb(55, 58, 63);
    let accent = egui::Color32::from_rgb(64, 158, 255); // selection blue
    let accent2 = egui::Color32::from_rgb(255, 165, 70); // warning/orange
    let text = egui::Color32::from_rgb(225, 228, 232);
    style.visuals = egui::Visuals::dark();
    style.visuals.override_text_color = Some(text);
    style.visuals.window_fill = bg1;
    style.visuals.panel_fill = bg0;
    style.visuals.extreme_bg_color = bg2;
    style.visuals.widgets.noninteractive.bg_fill = bg1;
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, stroke_soft);
    style.visuals.widgets.inactive.bg_fill = bg1;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, stroke_soft);
    style.visuals.widgets.hovered.bg_fill = bg2;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, stroke);
    style.visuals.widgets.active.bg_fill = bg2;
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, accent);
    style.visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 70);
    style.visuals.selection.stroke = egui::Stroke::new(1.0, accent);
    // Drag/drop / link lines (used by blueprint mock)
    style.visuals.hyperlink_color = accent;
    // Store theme constants in ctx for easy future retrieval (optional)
    ctx.set_style(style);
    // Optional: set the visuals for error/warning highlighting in widgets
    let _ = accent2;
}
