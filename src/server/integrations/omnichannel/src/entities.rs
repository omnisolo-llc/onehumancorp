use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub mod chat_inbox {
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
        #[sea_orm(has_many = "super::chat_channel::Entity")]
        ChatChannel,
        #[sea_orm(has_many = "super::chat_conversation::Entity")]
        ChatConversation,
    }

    impl Related<super::chat_channel::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatChannel.def()
        }
    }

    impl Related<super::chat_conversation::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatConversation.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_channel {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_channels")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub channel_type: String,
        pub config: Json,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::chat_inbox::Entity",
            from = "Column::InboxId",
            to = "super::chat_inbox::Column::Id"
        )]
        ChatInbox,
    }

    impl Related<super::chat_inbox::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatInbox.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_contact {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_contacts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: Option<String>,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::chat_conversation::Entity")]
        ChatConversation,
    }

    impl Related<super::chat_conversation::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatConversation.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_conversation {
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
            belongs_to = "super::chat_inbox::Entity",
            from = "Column::InboxId",
            to = "super::chat_inbox::Column::Id"
        )]
        ChatInbox,
        #[sea_orm(
            belongs_to = "super::chat_contact::Entity",
            from = "Column::ContactId",
            to = "super::chat_contact::Column::Id"
        )]
        ChatContact,
        #[sea_orm(has_many = "super::chat_message::Entity")]
        ChatMessage,
    }

    impl Related<super::chat_inbox::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatInbox.def()
        }
    }

    impl Related<super::chat_contact::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatContact.def()
        }
    }

    impl Related<super::chat_message::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatMessage.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_message {
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
        pub content: String,
        pub content_type: String,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::chat_conversation::Entity",
            from = "Column::ConversationId",
            to = "super::chat_conversation::Column::Id"
        )]
        ChatConversation,
    }

    impl Related<super::chat_conversation::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatConversation.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
