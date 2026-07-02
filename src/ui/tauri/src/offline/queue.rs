use std::collections::VecDeque;

pub struct SyncQueue {
    pub pending: VecDeque<String>,
}

impl SyncQueue {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }
}
