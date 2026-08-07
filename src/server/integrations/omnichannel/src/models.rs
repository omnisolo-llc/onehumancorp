use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "inbox")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String, // e.g., "whatsapp", "instagram", "email"
    pub config: Json,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub mod message {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "message")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub conversation_id: Uuid,
        pub content: String,
        pub content_type: String, // e.g., "text", "image"
        pub sender_type: String,  // e.g., "operator", "contact", "agent"
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod conversation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "conversation")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub contact_id: Uuid,
        pub status: String,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;
    use serde_json::json;

    #[test]
    fn test_inbox_instantiation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let inbox = Model {
            id,
            tenant_id,
            name: "My Whatsapp".to_string(),
            channel_type: "whatsapp".to_string(),
            config: json!({}),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };
        assert_eq!(inbox.name, "My Whatsapp");
        assert_eq!(inbox.channel_type, "whatsapp");
    }

    #[test]
    fn test_conversation_instantiation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let conversation = conversation::Model {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            status: "open".to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };
        assert_eq!(conversation.status, "open");
        assert_eq!(conversation.inbox_id, inbox_id);
    }

    #[test]
    fn test_message_instantiation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let message = message::Model {
            id,
            tenant_id,
            conversation_id,
            content: "Hello!".to_string(),
            content_type: "text".to_string(),
            sender_type: "contact".to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };
        assert_eq!(message.content, "Hello!");
        assert_eq!(message.sender_type, "contact");
    }
}
