pub mod viewport;
pub mod content_browser;
pub mod output_log;
pub mod world_outliner;
pub mod details;
pub mod modes;
pub mod blueprint_mock;

use crate::app::EditorApp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanelId {
    Modes,
    Viewport,
    OutputLog,
    ContentBrowser,
    WorldOutliner,
    Details,
}

pub struct PanelBehavior<'a> {
    pub app: &'a mut EditorApp,
}

impl egui_tiles::Behavior<PanelId> for PanelBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &PanelId) -> egui::WidgetText {
        match pane {
            PanelId::Modes => "Modes".into(),
            PanelId::Viewport => "Perspective".into(),
            PanelId::OutputLog => "Output Log".into(),
            PanelId::ContentBrowser => "Content Browser".into(),
            PanelId::WorldOutliner => "World Outliner".into(),
            PanelId::Details => "Details".into(),
        }
    }
    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: egui_tiles::TileId, pane: &mut PanelId) -> egui_tiles::UiResponse {
        // Slightly tighter padding like UE
        let old = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing = egui::vec2(6.0, 6.0);
        match pane {
            PanelId::Modes => modes::draw(ui, &mut self.app.project, &mut self.app.ui_state),
            PanelId::Viewport => viewport::draw(ui, &mut self.app.project, &mut self.app.ui_state),
            PanelId::OutputLog => output_log::draw(ui, &mut self.app.project),
            PanelId::ContentBrowser => content_browser::draw(ui, &mut self.app.project),
            PanelId::WorldOutliner => world_outliner::draw(ui, &mut self.app.project),
            PanelId::Details => details::draw(ui, &mut self.app.project),
        }
        ui.spacing_mut().item_spacing = old;
        egui_tiles::UiResponse::None
    }
    fn is_tab_closable(&self, _tiles: &egui_tiles::Tiles<PanelId>, _tile_id: egui_tiles::TileId) -> bool {
        false
    }
}
