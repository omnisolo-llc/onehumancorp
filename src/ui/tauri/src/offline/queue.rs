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

    pub fn enqueue(&mut self, payload: String) {
        self.pending.push_back(payload);
    }

    pub fn dequeue(&mut self) -> Option<String> {
        self.pending.pop_front()
    }
}
