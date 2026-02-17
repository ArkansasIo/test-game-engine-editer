use eframe::egui;
use neoforge_core::{assets, log::LogBuffer, project::{Project, ProjectError}};
use neoforge_graph::model::Graph;
use neoforge_runtime::vm::Vm;

use crate::{graph_ui::GraphUiState, viewport::ViewportState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolMode { Select, Move, Rotate, Scale }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewportMode { Mode2D, Mode3D }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CenterTab { Viewport, Graph }

pub struct EditorApp {
    tool: ToolMode,
    viewport_mode: ViewportMode,
    center_tab: CenterTab,

    // data
    log: LogBuffer,
    project: Option<Project>,
    assets: Vec<assets::AssetEntry>,
    graph: Graph,

    // ui state
    graph_ui: GraphUiState,
    viewport: ViewportState,

    // execution
    vm: Vm,

    // left panel state
    place_search: String,
    place_category: PlaceCategory,

    // extra asset directories
    extra_asset_dirs: Vec<std::path::PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlaceCategory { RecentlyPlaced, Basic, Lights, Cinematic, Vfx, Geometry, Volumes }

impl PlaceCategory {
    fn label(self) -> &'static str {
        match self {
            PlaceCategory::RecentlyPlaced => "Recently Placed",
            PlaceCategory::Basic => "Basic",
            PlaceCategory::Lights => "Lights",
            PlaceCategory::Cinematic => "Cinematic",
            PlaceCategory::Vfx => "Visual Effects",
            PlaceCategory::Geometry => "Geometry",
            PlaceCategory::Volumes => "Volumes",
        }
    }
}

impl EditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.0);

        let log = LogBuffer::default();
        log.push("NeoForge Studio started.");

        Self {
            tool: ToolMode::Select,
            viewport_mode: ViewportMode::Mode3D,
            center_tab: CenterTab::Viewport,
            log,
            project: None,
            assets: vec![],
            graph: Graph::empty(),
            graph_ui: GraphUiState::default(),
            viewport: ViewportState::default(),
            vm: Vm::with_builtin(),
            place_search: String::new(),
            place_category: PlaceCategory::RecentlyPlaced,
            extra_asset_dirs: vec![
                std::path::PathBuf::from("third_party/TownGeneratorOS"),
                std::path::PathBuf::from("tiny-creatures/tiny-creatures"),
                std::path::PathBuf::from("LiberatedPixelCup-master/Kyrises_16x16_RPG_Icon_Pack_V1.3/Kyrise's 16x16 RPG Icon Pack - V1.3"),
                std::path::PathBuf::from("LiberatedPixelCup-master/LiberatedPixelCup-master"),
            ],
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_menu_bar(ctx);
        self.top_tool_bar(ctx);

        self.left_place_panel(ctx);
        self.right_outliner_details_panel(ctx);
        self.bottom_content_browser(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.center_header(ui);

            ui.separator();

            match self.center_tab {
                CenterTab::Viewport => self.viewport.ui(ui, self.viewport_mode),
                CenterTab::Graph => self.graph_ui.ui(ui, &self.vm.registry, &mut self.graph, &self.log),
            }
        });
    }

    fn top_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menubar").exact_height(26.0).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project...").clicked() { ui.close_menu(); self.action_new_project(); }
                    if ui.button("Open Project...").clicked() { ui.close_menu(); self.action_open_project(); }
                    ui.separator();
                    if ui.button("Save Graph").clicked() { ui.close_menu(); self.action_save_graph(); }
                    if ui.button("Load Graph").clicked() { ui.close_menu(); self.action_load_graph(); }
                    ui.separator();
                    if ui.button("Export Runtime Package").clicked() { ui.close_menu(); self.action_export_runtime(); }
                    ui.separator();
                    if ui.button("Quit").clicked() { ui.close_menu(); ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                });

                ui.menu_button("Edit", |ui| {
                    ui.label("Undo/Redo (MVP placeholder)");
                });

                ui.menu_button("Window", |ui| {
                    if ui.button("Reset Layout").clicked() { ui.close_menu(); /* panels are fixed in MVP */ }
                    if ui.button("Toggle Console").clicked() { ui.close_menu(); /* always visible in MVP */ }
                });

                ui.menu_button("Tools", |ui| {
                    ui.label("Tool settings (MVP placeholder)");
                });

                ui.menu_button("Build", |ui| {
                    ui.label("Build pipeline (MVP placeholder)");
                });

                ui.menu_button("Help", |ui| {
                    ui.label("Docs/About (MVP placeholder)");
                });
            });
        });
    }

    fn top_tool_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").exact_height(44.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                // left cluster
                if ui.button("💾 Save").clicked() { self.action_save_graph(); }
                ui.separator();

                ui.menu_button("Source Control ▾", |ui| { ui.label("Not configured"); });
                ui.menu_button("Content ▾", |ui| { ui.label("Content actions"); });
                ui.menu_button("Marketplace ▾", |ui| { ui.label("MVP placeholder"); });

                ui.separator();

                ui.menu_button("Settings ▾", |ui| { ui.label("Preferences"); });

                // center cluster (mode)
                ui.separator();
                ui.selectable_value(&mut self.center_tab, CenterTab::Viewport, "Viewport");
                ui.selectable_value(&mut self.center_tab, CenterTab::Graph, "Blueprints");

                ui.separator();

                // right cluster (play/build)
                if ui.button("▶ Run").clicked() { self.action_run_graph_in_editor(); }
                ui.menu_button("Build ▾", |ui| { ui.label("Build options"); });
                if ui.button("🚀 Launch").clicked() { self.log.push("Launch (placeholder)"); }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("NeoForge Studio");
                });
            });
        });
    }

    fn left_place_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("place_actors").default_width(250.0).show(ctx, |ui| {
            ui.heading("Place Actors");
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label("🔎");
                ui.text_edit_singleline(&mut self.place_search);
            });
            ui.add_space(6.0);

            ui.group(|ui| {
                ui.selectable_value(&mut self.place_category, PlaceCategory::RecentlyPlaced, "Recently Placed");
                ui.separator();
                ui.selectable_value(&mut self.place_category, PlaceCategory::Basic, "Basic");
                ui.selectable_value(&mut self.place_category, PlaceCategory::Lights, "Lights");
                ui.selectable_value(&mut self.place_category, PlaceCategory::Cinematic, "Cinematic");
                ui.selectable_value(&mut self.place_category, PlaceCategory::Vfx, "Visual Effects");
                ui.selectable_value(&mut self.place_category, PlaceCategory::Geometry, "Geometry");
                ui.selectable_value(&mut self.place_category, PlaceCategory::Volumes, "Volumes");
            });

            ui.add_space(10.0);
            ui.separator();
            ui.label(format!("Category: {}", self.place_category.label()));
            ui.label("Drag/drop spawning is a later milestone.");
        });
    }

    fn right_outliner_details_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_stack").default_width(320.0).show(ctx, |ui| {
            // Outliner
            ui.heading("Outliner");
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("🔎");
                let mut s = String::new();
                ui.add_enabled(false, egui::TextEdit::singleline(&mut s).hint_text("Search (MVP)"));
            });
            ui.separator();
            egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                ui.label("Temple_Overview");
                ui.indent("o1", |ui| {
                    ui.label("SkyLight");
                    ui.label("DirectionalLight");
                    ui.label("SkySphere");
                    ui.label("StoneColumn1");
                    ui.label("AncientStatue");
                    ui.label("PlayerStart");
                });
            });

            ui.add_space(10.0);
            ui.separator();

            // Details
            ui.heading("Details");
            ui.add_space(6.0);
            ui.group(|ui| {
                ui.label("Transform");
                ui.separator();
                ui.horizontal(|ui| { ui.label("Location"); ui.add_enabled(false, egui::DragValue::new(&mut 0.0)); ui.add_enabled(false, egui::DragValue::new(&mut 0.0)); ui.add_enabled(false, egui::DragValue::new(&mut 0.0)); });
                ui.horizontal(|ui| { ui.label("Rotation"); ui.add_enabled(false, egui::DragValue::new(&mut 0.0)); ui.add_enabled(false, egui::DragValue::new(&mut 0.0)); ui.add_enabled(false, egui::DragValue::new(&mut 0.0)); });
                ui.horizontal(|ui| { ui.label("Scale"); ui.add_enabled(false, egui::DragValue::new(&mut 1.0)); ui.add_enabled(false, egui::DragValue::new(&mut 1.0)); ui.add_enabled(false, egui::DragValue::new(&mut 1.0)); });
            });

            ui.add_space(10.0);
            ui.separator();

            // Console (always visible in MVP)
            ui.heading("Console");
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() { self.log.clear(); }
            });
            ui.separator();
            egui::ScrollArea::vertical().stick_to_bottom(true).max_height(140.0).show(ui, |ui| {
                for line in self.log.snapshot() {
                    ui.label(line);
                }
            });
        });
    }

    fn bottom_content_browser(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("content_browser").exact_height(210.0).show(ctx, |ui| {
            ui.heading("Content Browser");
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                if ui.button("All").clicked() {}
                if ui.button("Maps").clicked() {}
                if ui.button("Materials").clicked() {}
                if ui.button("Meshes").clicked() {}
                ui.separator();
                if ui.button("Refresh").clicked() { self.refresh_assets(); }
                if let Some(p) = &self.project {
                    ui.label(format!("{}", p.content_dir().display()));
                }
                for dir in &self.extra_asset_dirs {
                    ui.label(format!("Extra: {}", dir.display()));
                }
                if self.project.is_none() && self.extra_asset_dirs.is_empty() {
                    ui.label("Open a project to browse Content/");
                }
            });

            ui.separator();

            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for a in &self.assets {
                        ui.group(|ui| {
                            ui.label(&a.rel_path);
                            ui.label(format!("[{}]", a.kind));
                        });
                        ui.add_space(6.0);
                    }
                    if self.assets.is_empty() {
                        ui.label("No assets indexed (MVP). Put files in Content/ and press Refresh.");
                    }
                });
            });
        });
    }

    fn center_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Viewport header similar feel (original implementation)
            ui.label(match self.center_tab { CenterTab::Viewport => "Viewport", CenterTab::Graph => "Graph" });
            ui.separator();
            ui.menu_button("Perspective ▾", |ui| {
                ui.label("Camera modes (MVP)");
            });
            ui.menu_button("Lit ▾", |ui| { ui.label("View modes (MVP)"); });
            ui.menu_button("Show ▾", |ui| { ui.label("Visibility toggles (MVP)"); });

            ui.separator();
            ui.selectable_value(&mut self.viewport_mode, ViewportMode::Mode3D, "3D");
            ui.selectable_value(&mut self.viewport_mode, ViewportMode::Mode2D, "2D");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Export Runtime").clicked() { self.action_export_runtime(); }
            });
        });
    }

    fn refresh_assets(&mut self) {
        let mut all_assets = Vec::new();
        if let Some(p) = &self.project {
            all_assets.extend(assets::index_content_dir(p.content_dir()));
        }
        for dir in &self.extra_asset_dirs {
            if dir.exists() {
                all_assets.extend(assets::index_content_dir(dir));
            }
        }
        self.assets = all_assets;
    }

    fn action_new_project(&mut self) {
        let Some(dir) = rfd::FileDialog::new().pick_folder() else { return; };
        let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("Project");
        match Project::create(&dir, name, &self.log) {
            Ok(p) => {
                self.project = Some(p);
                self.log.push("Created project.");
                self.refresh_assets();
            }
            Err(e) => self.log.push(format!("Create project failed: {e:?}")),
        }
    }

    fn action_open_project(&mut self) {
        let Some(dir) = rfd::FileDialog::new().pick_folder() else { return; };
        match Project::open(&dir, &self.log) {
            Ok(p) => {
                self.project = Some(p);
                self.log.push("Opened project.");
                self.refresh_assets();
            }
            Err(e) => self.log.push(format!("Open project failed: {e:?}")),
        }
    }

    fn action_save_graph(&mut self) {
        let Some(p) = &self.project else {
            self.log.push("Open a project first.");
            return;
        };
        let path = p.graphs_dir().join("Main.graph.json");
        match std::fs::write(&path, serde_json::to_vec_pretty(&self.graph).unwrap()) {
            Ok(_) => self.log.push(format!("Saved graph to {}", path.display())),
            Err(e) => self.log.push(format!("Save graph failed: {e:?}")),
        }
    }

    fn action_load_graph(&mut self) {
        let Some(p) = &self.project else {
            self.log.push("Open a project first.");
            return;
        };
        let path = p.graphs_dir().join("Main.graph.json");
        match std::fs::read(&path).ok().and_then(|bytes| serde_json::from_slice(&bytes).ok()) {
            Some(g) => {
                self.graph = g;
                self.log.push(format!("Loaded graph from {}", path.display()));
            }
            None => self.log.push(format!("Load graph failed: could not read or parse file")),
        }
    }

    fn action_run_graph_in_editor(&mut self) {
        let mut ctx = neoforge_plugins::HostContext::default();
        match self.vm.run(&self.graph, &mut ctx) {
            Ok(out) => self.log.push(format!("Graph run OK: {out:?}")),
            Err(e) => self.log.push(format!("Graph run error: {e:?}")),
        }
    }

    fn action_export_runtime(&mut self) {
        let Some(p) = &self.project else {
            self.log.push("Open a project first.");
            return;
        };
        let out_dir = p.root.join("Saved").join("RuntimeExport");
        let _ = std::fs::create_dir_all(&out_dir);
        let path = out_dir.join("Main.graph.json");
        match std::fs::write(&path, serde_json::to_vec_pretty(&self.graph).unwrap()) {
            Ok(_) => self.log.push(format!("Exported runtime graph to {}", path.display())),
            Err(e) => self.log.push(format!("Export runtime failed: {e:?}")),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update(ctx, frame);
    }
}
