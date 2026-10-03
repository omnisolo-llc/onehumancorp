use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl Contact {
    pub fn new(id: String, tenant_id: String, name: String, email: Option<String>, phone: Option<String>) -> Self {
        Self {
            id,
            tenant_id,
            name,
            email,
            phone,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_creation() {
        let contact = Contact::new(
            "contact-1".to_string(),
            "tenant-1".to_string(),
            "John Doe".to_string(),
            Some("john@example.com".to_string()),
            Some("555-1234".to_string()),
        );
        assert_eq!(contact.id, "contact-1");
        assert_eq!(contact.tenant_id, "tenant-1");
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.email.unwrap(), "john@example.com");
        assert_eq!(contact.phone.unwrap(), "555-1234");
    }
}
