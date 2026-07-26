use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub tenant_id: String,
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Inbox {
    pub fn new(tenant_id: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            tenant_id,
            id: Uuid::new_v4(),
            name,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_creation() {
        let tenant_id = "tenant-123".to_string();
        let name = "Support Inbox".to_string();

        let inbox = Inbox::new(tenant_id.clone(), name.clone());

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, name);
        assert!(!inbox.id.is_nil());
        assert!(inbox.created_at <= Utc::now());
        assert_eq!(inbox.created_at, inbox.updated_at);
    }
}
