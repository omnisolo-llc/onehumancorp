use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Inbox {
    pub fn new(tenant_id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_new() {
        let tenant_id = "tenant-123";
        let name = "Support Inbox";
        let inbox = Inbox::new(tenant_id, name);

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, name);
    }

    #[test]
    fn test_inbox_serialization() {
        let inbox = Inbox::new("tenant-123", "Support Inbox");
        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();

        assert_eq!(inbox, deserialized);
    }
}
