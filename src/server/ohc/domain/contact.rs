use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(
        id: String,
        tenant_id: String,
        name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            tenant_id,
            name,
            email,
            phone_number,
            created_at: now,
            updated_at: now,
        }
    }
}

#[async_trait::async_trait]
pub trait ContactService: Send + Sync {
    async fn find_or_create_contact(
        &self,
        tenant_id: &str,
        name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
    ) -> Result<Contact, String>;

    async fn get_contact(&self, tenant_id: &str, id: &str) -> Result<Option<Contact>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_creation_and_serde() {
        let contact = Contact::new(
            "contact-123".to_string(),
            "tenant-abc".to_string(),
            Some("John Doe".to_string()),
            Some("john.doe@example.com".to_string()),
            Some("+15551234567".to_string()),
        );

        assert_eq!(contact.id, "contact-123");
        assert_eq!(contact.tenant_id, "tenant-abc");
        assert_eq!(contact.name, Some("John Doe".to_string()));
        assert_eq!(contact.email, Some("john.doe@example.com".to_string()));
        assert_eq!(contact.phone_number, Some("+15551234567".to_string()));

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();
        assert_eq!(contact.id, deserialized.id);
        assert_eq!(contact.tenant_id, deserialized.tenant_id);
        assert_eq!(contact.name, deserialized.name);
        assert_eq!(contact.email, deserialized.email);
        assert_eq!(contact.phone_number, deserialized.phone_number);
    }

    #[test]
    fn test_contact_with_none_fields() {
        let contact = Contact::new(
            "contact-456".to_string(),
            "tenant-abc".to_string(),
            None,
            None,
            None,
        );

        assert_eq!(contact.id, "contact-456");
        assert_eq!(contact.tenant_id, "tenant-abc");
        assert_eq!(contact.name, None);
        assert_eq!(contact.email, None);
        assert_eq!(contact.phone_number, None);

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();
        assert_eq!(contact.id, deserialized.id);
        assert_eq!(contact.tenant_id, deserialized.tenant_id);
        assert_eq!(contact.name, deserialized.name);
        assert_eq!(contact.email, deserialized.email);
        assert_eq!(contact.phone_number, deserialized.phone_number);
    }
}
