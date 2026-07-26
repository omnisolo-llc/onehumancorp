use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub tenant_id: String,
    pub id: Uuid,
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
            tenant_id,
            id: Uuid::new_v4(),
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
    fn test_contact_creation() {
        let tenant_id = "tenant-123".to_string();
        let name = "Maya Baker".to_string();
        let email = Some("maya@example.com".to_string());
        let phone = Some("+1234567890".to_string());

        let contact = Contact::new(tenant_id.clone(), name.clone(), email.clone(), phone.clone());

        assert_eq!(contact.tenant_id, tenant_id);
        assert_eq!(contact.name, name);
        assert_eq!(contact.email, email);
        assert_eq!(contact.phone, phone);
        assert!(!contact.id.is_nil());
        assert!(contact.created_at <= Utc::now());
        assert_eq!(contact.created_at, contact.updated_at);
    }
}
