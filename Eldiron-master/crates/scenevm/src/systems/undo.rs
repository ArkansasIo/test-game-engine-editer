#[derive(Debug, Clone)]
pub struct Transaction<T> {
    pub label: String,
    pub before: T,
    pub after: T,
}

#[derive(Debug, Clone)]
pub struct UndoService {
    capacity: usize,
    cursor: usize,
    history: Vec<Transaction<String>>,
}

impl Default for UndoService {
    fn default() -> Self {
        Self {
            capacity: 128,
            cursor: 0,
            history: Vec::new(),
        }
    }
}

impl UndoService {
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity.max(1);
        if self.history.len() > self.capacity {
            let drop_count = self.history.len() - self.capacity;
            self.history.drain(0..drop_count);
            self.cursor = self.cursor.saturating_sub(drop_count);
        }
    }

    /// Record a transaction using serialized snapshots (JSON/string payloads).
    pub fn push(&mut self, label: impl Into<String>, before: String, after: String) {
        if self.cursor < self.history.len() {
            self.history.truncate(self.cursor);
        }
        self.history.push(Transaction {
            label: label.into(),
            before,
            after,
        });
        if self.history.len() > self.capacity {
            self.history.remove(0);
        } else {
            self.cursor += 1;
        }
        if self.cursor > self.history.len() {
            self.cursor = self.history.len();
        }
    }

    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        self.cursor < self.history.len()
    }

    pub fn undo(&mut self) -> Option<&Transaction<String>> {
        if !self.can_undo() {
            return None;
        }
        self.cursor -= 1;
        self.history.get(self.cursor)
    }

    pub fn redo(&mut self) -> Option<&Transaction<String>> {
        if !self.can_redo() {
            return None;
        }
        let tx = self.history.get(self.cursor);
        self.cursor += 1;
        tx
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }
}
