use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "inboxes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "conversation::Entity")]
    Conversation,
    #[sea_orm(has_many = "channel_web_widget::Entity")]
    ChannelWebWidget,
    #[sea_orm(has_many = "channel_email::Entity")]
    ChannelEmail,
}

impl Related<conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversation.def()
    }
}
impl Related<channel_web_widget::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelWebWidget.def()
    }
}
impl Related<channel_email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelEmail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub mod conversation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "conversations")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub contact_id: Uuid,
        pub assignee_id: Option<Uuid>,
        pub status: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::Entity",
            from = "Column::InboxId",
            to = "super::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Inbox,
        #[sea_orm(
            belongs_to = "super::contact::Entity",
            from = "Column::ContactId",
            to = "super::contact::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Contact,
        #[sea_orm(has_many = "super::message::Entity")]
        Message,
    }

    impl Related<super::Entity> for Entity {
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

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "messages")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub conversation_id: Uuid,
        pub sender_id: Option<Uuid>,
        pub content: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::conversation::Entity",
            from = "Column::ConversationId",
            to = "super::conversation::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
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

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "contacts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: String,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
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

pub mod channel_web_widget {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "channel_web_widgets")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub website_url: String,
        pub widget_color: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::Entity",
            from = "Column::InboxId",
            to = "super::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Inbox,
    }

    impl Related<super::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Inbox.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod channel_email {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "channel_emails")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub inbox_id: Uuid,
        pub email: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::Entity",
            from = "Column::InboxId",
            to = "super::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Inbox,
    }

    impl Related<super::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Inbox.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
