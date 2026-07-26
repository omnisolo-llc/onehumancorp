use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(tenant_id: String, name: String, email: Option<String>, phone_number: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            name,
            email,
            phone_number,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_creation() {
        let tenant_id = "tenant-456".to_string();
        let name = "Maya Baker".to_string();
        let email = Some("maya@example.com".to_string());

        let contact = Contact::new(tenant_id.clone(), name.clone(), email.clone(), None);

        assert!(!contact.id.is_empty());
        assert_eq!(contact.tenant_id, tenant_id);
        assert_eq!(contact.name, name);
        assert_eq!(contact.email, email);
        assert_eq!(contact.phone_number, None);
        assert_eq!(contact.avatar_url, None);
        assert!(contact.created_at <= Utc::now());
        assert_eq!(contact.created_at, contact.updated_at);
    }
}
