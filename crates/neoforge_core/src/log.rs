use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct LogBuffer {
    inner: Arc<Mutex<Vec<String>>>,
}

impl LogBuffer {
    pub fn push(&self, msg: impl Into<String>) {
        let mut g = self.inner.lock().unwrap();
        g.push(msg.into());
        let len = g.len();
        if len > 2000 {
            g.drain(0..len.saturating_sub(2000));
        }
    }

    pub fn snapshot(&self) -> Vec<String> {
        self.inner.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.inner.lock().unwrap().clear();
    }
}
