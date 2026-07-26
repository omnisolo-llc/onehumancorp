use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(tenant_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name: None,
            email: None,
            phone_number: None,
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
    fn test_new_contact() {
        let tenant_id = "tenant_1".to_string();

        let contact = Contact::new(tenant_id.clone());

        assert_eq!(contact.tenant_id, tenant_id);
        assert!(contact.name.is_none());
        assert!(contact.email.is_none());
        assert!(contact.phone_number.is_none());
        assert!(contact.avatar_url.is_none());
    }
}
