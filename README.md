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
