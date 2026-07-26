use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
}

impl Contact {
    pub fn new(id: String, tenant_id: String, name: String) -> Self {
        Self {
            id,
            tenant_id,
            name,
            email: None,
            phone: None,
            avatar_url: None,
        }
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn with_avatar_url(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_new() {
        let contact = Contact::new(
            "contact_1".to_string(),
            "tenant_1".to_string(),
            "Alice Smith".to_string(),
        );

        assert_eq!(contact.id, "contact_1");
        assert_eq!(contact.tenant_id, "tenant_1");
        assert_eq!(contact.name, "Alice Smith");
        assert_eq!(contact.email, None);
        assert_eq!(contact.phone, None);
        assert_eq!(contact.avatar_url, None);
    }

    #[test]
    fn test_contact_builder() {
        let contact = Contact::new(
            "contact_1".to_string(),
            "tenant_1".to_string(),
            "Alice Smith".to_string(),
        )
        .with_email("alice@example.com".to_string())
        .with_phone("+1234567890".to_string())
        .with_avatar_url("https://example.com/avatar.jpg".to_string());

        assert_eq!(contact.email.unwrap(), "alice@example.com");
        assert_eq!(contact.phone.unwrap(), "+1234567890");
        assert_eq!(contact.avatar_url.unwrap(), "https://example.com/avatar.jpg");
    }

    #[test]
    fn test_contact_serialization() {
        let contact = Contact::new(
            "contact_1".to_string(),
            "tenant_1".to_string(),
            "Alice Smith".to_string(),
        )
        .with_email("alice@example.com".to_string());

        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();

        assert_eq!(contact, deserialized);
    }
}
