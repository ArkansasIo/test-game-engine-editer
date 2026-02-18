pub struct EditorModule {
    pub name: &'static str,
    pub provides_commands: bool,
    pub provides_tabs: bool,
    pub provides_services: bool,
}

pub fn default_module_graph() -> Vec<EditorModule> {
    vec![
        EditorModule {
            name: "AppCore",
            provides_commands: true,
            provides_tabs: false,
            provides_services: true,
        },
        EditorModule {
            name: "CommandSystem",
            provides_commands: true,
            provides_tabs: false,
            provides_services: true,
        },
        EditorModule {
            name: "DockingLayoutSystem",
            provides_commands: false,
            provides_tabs: true,
            provides_services: true,
        },
        EditorModule {
            name: "AssetSystem",
            provides_commands: true,
            provides_tabs: true,
            provides_services: true,
        },
        EditorModule {
            name: "SelectionSystem",
            provides_commands: true,
            provides_tabs: false,
            provides_services: true,
        },
        EditorModule {
            name: "UndoRedoSystem",
            provides_commands: true,
            provides_tabs: false,
            provides_services: true,
        },
        EditorModule {
            name: "ViewportSystem",
            provides_commands: true,
            provides_tabs: true,
            provides_services: true,
        },
        EditorModule {
            name: "BuildSystem",
            provides_commands: true,
            provides_tabs: false,
            provides_services: true,
        },
        EditorModule {
            name: "SourceControl",
            provides_commands: true,
            provides_tabs: false,
            provides_services: true,
        },
        EditorModule {
            name: "LevelEditor",
            provides_commands: true,
            provides_tabs: true,
            provides_services: false,
        },
        EditorModule {
            name: "BlueprintEditor",
            provides_commands: true,
            provides_tabs: true,
            provides_services: false,
        },
        EditorModule {
            name: "MaterialEditor",
            provides_commands: true,
            provides_tabs: true,
            provides_services: false,
        },
        EditorModule {
            name: "NiagaraFXEditor",
            provides_commands: true,
            provides_tabs: true,
            provides_services: false,
        },
        EditorModule {
            name: "Sequencer",
            provides_commands: true,
            provides_tabs: true,
            provides_services: false,
        },
    ]
}
