pub struct ActionSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

#[derive(Default)]
pub struct ActionRegistry {
    actions: Vec<ActionSpec>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            actions: vec![
                ActionSpec {
                    id: "save",
                    label: "Save",
                    shortcut: Some("Ctrl+S"),
                },
                ActionSpec {
                    id: "undo",
                    label: "Undo",
                    shortcut: Some("Ctrl+Z"),
                },
                ActionSpec {
                    id: "redo",
                    label: "Redo",
                    shortcut: Some("Ctrl+Y"),
                },
                ActionSpec {
                    id: "play",
                    label: "Play",
                    shortcut: Some("F5"),
                },
            ],
        }
    }

    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    pub fn action_hints(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|a| match a.shortcut {
                Some(s) => format!("{} [{}] ({})", a.label, s, a.id),
                None => format!("{} ({})", a.label, a.id),
            })
            .collect()
    }
}
