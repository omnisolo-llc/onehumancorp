use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(tenant_id: String, name: String, email: Option<String>, phone: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            email,
            phone,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_new() {
        let tenant_id = "tenant-123".to_string();
        let name = "John Doe".to_string();
        let email = Some("john@example.com".to_string());

        let contact = Contact::new(tenant_id.clone(), name.clone(), email.clone(), None);

        assert_eq!(contact.tenant_id, tenant_id);
        assert_eq!(contact.name, name);
        assert_eq!(contact.email, email);
        assert_eq!(contact.phone, None);
        assert!(!contact.id.is_nil());
        assert!(contact.created_at <= Utc::now());
        assert_eq!(contact.created_at, contact.updated_at);
    }

    #[test]
    fn test_contact_serialization() {
        let contact = Contact::new(
            "tenant-123".to_string(),
            "Jane Doe".to_string(),
            None,
            Some("+1234567890".to_string())
        );
        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(contact, deserialized);
    }
}
