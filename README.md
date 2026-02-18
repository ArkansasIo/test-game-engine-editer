# NeoForge Studio (Rust)

An original, Unreal-inspired editor (dockable panels + node-graph scripting) implemented in Rust.

## Features (MVP)
- Dock layout: Viewport (2D/3D placeholder), Outliner, Details, Content, Graph, Console
- Project open/create (local folder)
- Graph editor (original implementation) + save/load JSON
- Graph runtime interpreter (in-editor) + exportable runtime crate
- Plugin traits + TownGen adapter stub

## Build & run
```bash
rustup update
cargo run -p neoforge_editor
```

## Project layout (created by the editor)
```
<MyProject>/
  Project.neoforge.json
  Content/
  Maps/
  Graphs/
  Plugins/
  Saved/
```

## Notes
- Viewport rendering is stubbed but wired through wgpu via eframe; extend in `apps/neoforge_editor/src/viewport.rs`.
- The node graph UI is implemented in egui; extend in `apps/neoforge_editor/src/graph_ui.rs`.

## UE5 Editor Shell (Generated Systems)
- Crate: `ue5_editor_shell`
- Run: `cargo run -p ue5_editor_shell`
- Command system: queue + command application + undo/redo snapshots.
- Core logic: project lifecycle, play/stop, build, blueprint compile, actor/content CRUD.
- Settings/options: grid, snap, snap value, autosave, realtime viewport, panel visibility toggles.
- UI systems: menubar commands, toolbar actions, mode switching, searchable outliner, editable details, filtered content browser, output log actions, blueprint window actions.
- Hotkeys: `Ctrl+S`, `Ctrl+Z`, `Ctrl+Y`, `Ctrl+B`, `F5`, `Delete`.
- App architecture modules: `editor_api/{types,services,app_core,module_graph}.rs`.
- Services scaffolded: `IEditorApp`, `ILayoutService`, `IBuildService`, `ISourceControlService`, `IViewportService`.
- Layout presets: `Default`, `Animation`, `Modeling`, `Debug`.
- View modes: `Lit`, `Unlit`, `Wireframe` (menu + toolbar wired).
- Build routing: lighting, geometry/paths, navigation, build all.
- Source control routing: connect/disconnect + status bar state.
- Status bar: source control, shader jobs, FPS, layout preset, viewport mode, build state, status text.
- Menu coverage: File, Edit, Window, Tools, Build, Select, Actor, Components, Level, Blueprints, Materials/FX, Cinematics, Play, Help.
- Submenu options system: each top menu now has an `Options` submenu wired to typed state + commands.
- Option state model: `MenuOptionKey` + `MenuOptions` in `ue5_editor_shell/src/state/mod.rs`.
- Option command routing: `SetMenuOption` and `SetPlayClientCount` in `ue5_editor_shell/src/actions/commands.rs`.
- Unified settings panel for all menu-feature options in `ue5_editor_shell/src/ui/dock.rs`.

## 2D/3D Scene, Level, and Resource Systems
- View systems:
  - Active dimension switching (`2D` / `3D`) via toolbar, menus, and modes panel.
  - View state stored in `ProjectState.active_dimension`.
- Scene systems:
  - Create/select scenes with per-scene dimension metadata.
  - State and helpers in `ue5_editor_shell/src/state/mod.rs`.
- Level systems:
  - Create/select levels bound to scenes and dimensions.
  - Active level updates current map context.
- Resource systems:
  - Create/remove engine resources (Texture, Mesh, Material, Blueprint, Audio, Script, LevelAsset).
  - Managed through command routing + settings panel resource manager.
- Command coverage:
  - `SetActiveDimension`, `CreateScene`, `SelectScene`, `CreateLevel`, `SelectLevel`, `CreateResource`, `RemoveResource` in `ue5_editor_shell/src/actions/commands.rs`.

## External Source Integration
- Integrated source roots:
  - `Eldiron-master`
  - `TownGeneratorOS-master`
- `towngen_adapter` now indexes both folders in code:
  - `crates/towngen_adapter/src/sources.rs`
  - `crates/towngen_adapter/src/lib.rs`
- `engine_core` exposes manifests for both folders:
  - `engine_core/src/source_integrations.rs`
  - `engine_core/src/lib.rs`
