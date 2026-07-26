use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub identifier: String,
}

impl Contact {
    pub fn new(id: String, tenant_id: String, name: String, identifier: String) -> Self {
        Self {
            id,
            tenant_id,
            name,
            identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_creation() {
        let contact = Contact::new(
            "contact_1".to_string(),
            "tenant_1".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
        );

        assert_eq!(contact.id, "contact_1");
        assert_eq!(contact.tenant_id, "tenant_1");
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.identifier, "john@example.com");
    }
}
