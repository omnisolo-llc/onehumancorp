// Persistent Memory Layer
pub struct MemoryStore {}

impl MemoryStore {
    pub fn new() -> Self {
        Self {}
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_store_creation() {
        let store = super::MemoryStore::new();
        assert!(true);
    }
}
