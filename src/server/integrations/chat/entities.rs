use sea_orm::entity::prelude::*;

pub mod chat_inboxes {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "chat_inboxes")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: String,
        pub created_at: Option<DateTimeWithTimeZone>,
        pub updated_at: Option<DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::chat_channels::Entity")]
        ChatChannels,
        #[sea_orm(has_many = "super::chat_conversations::Entity")]
        ChatConversations,
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_channels {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "chat_channels")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub channel_type: String,
        pub config: Option<Json>,
        pub created_at: Option<DateTimeWithTimeZone>,
        pub updated_at: Option<DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::chat_inboxes::Entity",
            from = "Column::InboxId",
            to = "super::chat_inboxes::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        ChatInboxes,
    }

    impl Related<super::chat_inboxes::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatInboxes.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_contacts {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "chat_contacts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: Option<String>,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub created_at: Option<DateTimeWithTimeZone>,
        pub updated_at: Option<DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::chat_conversations::Entity")]
        ChatConversations,
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_conversations {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "chat_conversations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub contact_id: Uuid,
        pub assignee_id: Option<Uuid>,
        pub status: String,
        pub created_at: Option<DateTimeWithTimeZone>,
        pub updated_at: Option<DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::chat_inboxes::Entity",
            from = "Column::InboxId",
            to = "super::chat_inboxes::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        ChatInboxes,
        #[sea_orm(
            belongs_to = "super::chat_contacts::Entity",
            from = "Column::ContactId",
            to = "super::chat_contacts::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        ChatContacts,
        #[sea_orm(has_many = "super::chat_messages::Entity")]
        ChatMessages,
    }

    impl Related<super::chat_inboxes::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatInboxes.def()
        }
    }

    impl Related<super::chat_contacts::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatContacts.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod chat_messages {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
    #[sea_orm(table_name = "chat_messages")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub conversation_id: Uuid,
        pub sender_type: String,
        pub sender_id: Option<Uuid>,
        pub content: String,
        pub created_at: Option<DateTimeWithTimeZone>,
        pub updated_at: Option<DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::chat_conversations::Entity",
            from = "Column::ConversationId",
            to = "super::chat_conversations::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        ChatConversations,
    }

    impl Related<super::chat_conversations::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::ChatConversations.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
