//! Engine and editor version information


pub const ENGINE_NAME: &str = "NeoForge Engine";
pub const ENGINE_CODENAME: &str = "Project Arkadia";
pub const ENGINE_VERSION: &str = "0.1.0";
pub const EDITOR_VERSION: &str = "0.1.0";

pub fn print_versions() {
    println!("Engine: {} (Codename: {})", ENGINE_NAME, ENGINE_CODENAME);
    println!("Engine Version: {}", ENGINE_VERSION);
    println!("Editor Version: {}", EDITOR_VERSION);
}
