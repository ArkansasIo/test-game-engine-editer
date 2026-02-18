use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionKind {
    Actor,
    Component,
    Asset,
    Node,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectionId {
    pub kind: SelectionKind,
    pub id: u64,
}

#[derive(Debug, Clone, Default)]
pub struct SelectionService {
    entries: FxHashSet<SelectionId>,
    primary: Option<SelectionId>,
}

impl SelectionService {
    pub fn clear(&mut self) {
        self.entries.clear();
        self.primary = None;
    }

    pub fn set_single(&mut self, entry: SelectionId) {
        self.entries.clear();
        self.entries.insert(entry.clone());
        self.primary = Some(entry);
    }

    pub fn add(&mut self, entry: SelectionId) {
        if self.primary.is_none() {
            self.primary = Some(entry.clone());
        }
        self.entries.insert(entry);
    }

    pub fn remove(&mut self, entry: &SelectionId) {
        self.entries.remove(entry);
        if self.primary.as_ref() == Some(entry) {
            self.primary = self.entries.iter().next().cloned();
        }
    }

    pub fn contains(&self, entry: &SelectionId) -> bool {
        self.entries.contains(entry)
    }

    pub fn primary(&self) -> Option<&SelectionId> {
        self.primary.as_ref()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &SelectionId> {
        self.entries.iter()
    }
}
