use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
}

impl Inbox {
    pub fn new(id: String, tenant_id: String, name: String) -> Self {
        Self {
            id,
            tenant_id,
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_creation() {
        let inbox = Inbox::new("inbox-1".to_string(), "tenant-1".to_string(), "Main Inbox".to_string());
        assert_eq!(inbox.id, "inbox-1");
        assert_eq!(inbox.tenant_id, "tenant-1");
        assert_eq!(inbox.name, "Main Inbox");
    }
}
