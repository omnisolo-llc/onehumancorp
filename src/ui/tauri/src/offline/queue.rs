use std::collections::VecDeque;

pub struct SyncQueue {
    pub pending: VecDeque<String>,
}

impl Default for SyncQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncQueue {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}
