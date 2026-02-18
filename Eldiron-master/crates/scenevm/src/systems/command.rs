use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandContext {
    Global,
    Viewport,
    Outliner,
    ContentBrowser,
    GraphEditor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorAction {
    Save,
    Undo,
    Redo,
    Play,
    Stop,
    BuildLighting,
    BuildAll,
    AddActor,
    DeleteSelected,
    DuplicateSelected,
    ToggleGrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shortcut {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: char,
}

#[derive(Debug, Clone)]
pub struct CommandRouter {
    pub focus: CommandContext,
    bindings: FxHashMap<(CommandContext, Shortcut), EditorAction>,
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self {
            focus: CommandContext::Global,
            bindings: FxHashMap::default(),
        }
    }
}

impl CommandRouter {
    pub fn register(
        &mut self,
        context: CommandContext,
        shortcut: Shortcut,
        action: EditorAction,
    ) {
        self.bindings.insert((context, shortcut), action);
    }

    pub fn set_focus(&mut self, context: CommandContext) {
        self.focus = context;
    }

    pub fn resolve(&self, shortcut: Shortcut) -> Option<EditorAction> {
        self.bindings
            .get(&(self.focus, shortcut))
            .copied()
            .or_else(|| self.bindings.get(&(CommandContext::Global, shortcut)).copied())
    }

    pub fn seed_defaults(&mut self) {
        self.register(
            CommandContext::Global,
            Shortcut {
                ctrl: true,
                shift: false,
                alt: false,
                key: 'S',
            },
            EditorAction::Save,
        );
        self.register(
            CommandContext::Global,
            Shortcut {
                ctrl: true,
                shift: false,
                alt: false,
                key: 'Z',
            },
            EditorAction::Undo,
        );
        self.register(
            CommandContext::Global,
            Shortcut {
                ctrl: true,
                shift: false,
                alt: false,
                key: 'Y',
            },
            EditorAction::Redo,
        );
        self.register(
            CommandContext::Viewport,
            Shortcut {
                ctrl: false,
                shift: false,
                alt: false,
                key: 'P',
            },
            EditorAction::Play,
        );
    }
}
