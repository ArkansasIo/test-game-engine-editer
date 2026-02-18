@echo off
REM 1. Generate the UE5-style editor shell
python gen_ue5_editor.py ue5_editor_shell

REM 2. Enter the generated project directory
cd ue5_editor_shell

REM 3. Run the new editor shell (Rust + egui/eframe)
cargo run

REM 4. Go back to the workspace root
cd ..

REM 5. Build your entire Rust workspace (all crates)
cargo build --workspace

REM 6. Run the full system demo for your engine_core crate
cargo run --example full_system_demo -p engine_core

REM 7. Run the generated UE5 editor shell (again, if needed)
cargo run -p ue5_editor_shell

REM 8. Run your own editor_shell crate (if you have one)
cargo run -p editor_shell
