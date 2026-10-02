use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub mod inbox {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_inboxes")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: String,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::conversation::Entity")]
        Conversation,
    }

    impl Related<super::conversation::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Conversation.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod conversation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_conversations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub contact_id: Uuid,
        pub assignee_id: Option<Uuid>,
        pub status: String,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::inbox::Entity",
            from = "Column::InboxId",
            to = "super::inbox::Column::Id"
        )]
        Inbox,
        #[sea_orm(has_many = "super::message::Entity")]
        Message,
    }

    impl Related<super::inbox::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Inbox.def()
        }
    }

    impl Related<super::message::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Message.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod message {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_messages")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub conversation_id: Uuid,
        pub sender_type: String,
        pub sender_id: Option<Uuid>,
        pub content_type: String,
        pub content: String,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::conversation::Entity",
            from = "Column::ConversationId",
            to = "super::conversation::Column::Id"
        )]
        Conversation,
    }

    impl Related<super::conversation::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Conversation.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod contact {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_contacts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: String,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub channel_identity: String,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
