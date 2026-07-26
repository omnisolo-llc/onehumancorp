use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl Contact {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, name: impl Into<String>, email: impl Into<String>, phone: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            email: email.into(),
            phone: phone.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_creation() {
        let contact = Contact::new("c1", "t1", "Alice", "alice@example.com", "1234567890");
        assert_eq!(contact.id, "c1");
        assert_eq!(contact.tenant_id, "t1");
        assert_eq!(contact.name, "Alice");
        assert_eq!(contact.email, "alice@example.com");
        assert_eq!(contact.phone, "1234567890");
    }

    #[test]
    fn test_contact_serialization() {
        let contact = Contact::new("c1", "t1", "Alice", "alice@example.com", "1234567890");
        let serialized = serde_json::to_string(&contact).unwrap();
        let deserialized: Contact = serde_json::from_str(&serialized).unwrap();
        assert_eq!(contact, deserialized);
    }
}
