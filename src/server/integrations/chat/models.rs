use sea_orm::entity::prelude::*;

pub mod inbox {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "inboxes")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: String,
        pub channel_type: String,
        pub channel_id: Uuid,
        pub is_active: bool,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
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

pub mod contact {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "contacts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: Option<String>,
        pub email: Option<String>,
        pub phone_number: Option<String>,
        pub custom_attributes: Option<Json>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
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

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "conversations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub contact_id: Uuid,
        pub assignee_id: Option<Uuid>,
        pub status: String,
        pub last_activity_at: DateTimeUtc,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::inbox::Entity",
            from = "Column::InboxId",
            to = "super::inbox::Column::Id"
        )]
        Inbox,
        #[sea_orm(
            belongs_to = "super::contact::Entity",
            from = "Column::ContactId",
            to = "super::contact::Column::Id"
        )]
        Contact,
        #[sea_orm(has_many = "super::message::Entity")]
        Message,
    }

    impl Related<super::inbox::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Inbox.def()
        }
    }

    impl Related<super::contact::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Contact.def()
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

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "messages")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub conversation_id: Uuid,
        pub sender_id: Option<Uuid>,
        pub sender_type: String,
        pub content: String,
        pub metadata: Option<Json>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
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
