use crate::types::Message;
use std::collections::VecDeque;

/// Master Catalog B.3. Memory
/// Short-term: Conversation history in the active session.
///
/// Encapsulates the active session's conversation history, ensuring
/// strict ordering and allowing for bounded capacity limits to prevent context overflow.
#[derive(Debug, Clone)]
pub struct ShortTermMemory {
    messages: VecDeque<Message>,
    capacity: usize,
}

impl ShortTermMemory {
    /// Create a new ShortTermMemory with a specified capacity (e.g. 100 messages)
    pub fn new(capacity: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Add a message to the conversation history, dropping the oldest if capacity is reached
    pub fn add(&mut self, message: Message) {
        if self.messages.len() >= self.capacity {
            self.messages.pop_front();
        }
        self.messages.push_back(message);
    }

    /// Retrieve all messages
    pub fn get_all(&self) -> Vec<Message> {
        self.messages.iter().cloned().collect()
    }

    /// Clear the history
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_term_memory_capacity() {
        let mut memory = ShortTermMemory::new(3);
        memory.add(Message::user("1".to_string()));
        memory.add(Message::user("2".to_string()));
        memory.add(Message::user("3".to_string()));

        assert_eq!(memory.get_all().len(), 3);

        memory.add(Message::user("4".to_string()));
        let all = memory.get_all();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].content, "2");
        assert_eq!(all[2].content, "4");
    }
}
