use sea_orm::entity::prelude::*;
use sea_orm::{Set, QueryOrder, ActiveModelTrait, ConnectionTrait};
use serde::{Deserialize, Serialize};

pub mod tenant {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_tenants")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub name: String,
        pub settings: Option<serde_json::Value>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "crate::db::inbox::Entity")]
        Inbox,
        #[sea_orm(has_many = "crate::db::contact::Entity")]
        Contact,
    }

    impl Related<crate::db::inbox::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Inbox.def()
        }
    }

    impl Related<crate::db::contact::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Contact.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod inbox {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "chat_inboxes")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub tenant_id: Uuid,
        pub name: String,
        pub channel_type: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "crate::db::tenant::Entity",
            from = "Column::TenantId",
            to = "crate::db::tenant::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Tenant,
        #[sea_orm(has_many = "crate::db::conversation::Entity")]
        Conversation,
    }

    impl Related<crate::db::tenant::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Tenant.def()
        }
    }

    impl Related<crate::db::conversation::Entity> for Entity {
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
        pub phone_number: Option<String>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "crate::db::tenant::Entity",
            from = "Column::TenantId",
            to = "crate::db::tenant::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Tenant,
        #[sea_orm(has_many = "crate::db::conversation::Entity")]
        Conversation,
    }

    impl Related<crate::db::tenant::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Tenant.def()
        }
    }

    impl Related<crate::db::conversation::Entity> for Entity {
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
        pub status: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "crate::db::inbox::Entity",
            from = "Column::InboxId",
            to = "crate::db::inbox::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Inbox,
        #[sea_orm(
            belongs_to = "crate::db::contact::Entity",
            from = "Column::ContactId",
            to = "crate::db::contact::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Contact,
        #[sea_orm(
            belongs_to = "crate::db::tenant::Entity",
            from = "Column::TenantId",
            to = "crate::db::tenant::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Tenant,
        #[sea_orm(has_many = "crate::db::message::Entity")]
        Message,
    }

    impl Related<crate::db::inbox::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Inbox.def()
        }
    }

    impl Related<crate::db::contact::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Contact.def()
        }
    }

    impl Related<crate::db::tenant::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Tenant.def()
        }
    }

    impl Related<crate::db::message::Entity> for Entity {
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
        pub content: String,
        pub message_type: String, // e.g., "incoming", "outgoing"
        pub created_at: chrono::DateTime<chrono::Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "crate::db::conversation::Entity",
            from = "Column::ConversationId",
            to = "crate::db::conversation::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Conversation,
        #[sea_orm(
            belongs_to = "crate::db::tenant::Entity",
            from = "Column::TenantId",
            to = "crate::db::tenant::Column::Id",
            on_update = "NoAction",
            on_delete = "Cascade"
        )]
        Tenant,
    }

    impl Related<crate::db::conversation::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Conversation.def()
        }
    }

    impl Related<crate::db::tenant::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Tenant.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// Database Helper functions ensuring `tenant_id` filtering
pub async fn create_tenant<C: ConnectionTrait>(
    db: &C,
    id: Uuid,
    name: String,
) -> Result<tenant::Model, sea_orm::DbErr> {
    let new_tenant = tenant::ActiveModel {
        id: Set(id),
        name: Set(name),
        settings: Set(None),
    };
    new_tenant.insert(db).await
}

pub async fn create_inbox<C: ConnectionTrait>(
    db: &C,
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    channel_type: String,
) -> Result<inbox::Model, sea_orm::DbErr> {
    let new_inbox = inbox::ActiveModel {
        id: Set(id),
        tenant_id: Set(tenant_id),
        name: Set(name),
        channel_type: Set(channel_type),
    };
    new_inbox.insert(db).await
}

pub async fn list_inboxes<C: ConnectionTrait>(
    db: &C,
    tenant_id: Uuid,
) -> Result<Vec<inbox::Model>, sea_orm::DbErr> {
    inbox::Entity::find()
        .filter(inbox::Column::TenantId.eq(tenant_id))
        .all(db)
        .await
}

pub async fn create_contact<C: ConnectionTrait>(
    db: &C,
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    email: Option<String>,
    phone_number: Option<String>,
) -> Result<contact::Model, sea_orm::DbErr> {
    let new_contact = contact::ActiveModel {
        id: Set(id),
        tenant_id: Set(tenant_id),
        name: Set(name),
        email: Set(email),
        phone_number: Set(phone_number),
    };
    new_contact.insert(db).await
}

pub async fn create_conversation<C: ConnectionTrait>(
    db: &C,
    id: Uuid,
    tenant_id: Uuid,
    inbox_id: Uuid,
    contact_id: Uuid,
    status: String,
) -> Result<conversation::Model, sea_orm::DbErr> {
    let new_conversation = conversation::ActiveModel {
        id: Set(id),
        tenant_id: Set(tenant_id),
        inbox_id: Set(inbox_id),
        contact_id: Set(contact_id),
        status: Set(status),
    };
    new_conversation.insert(db).await
}

pub async fn list_conversations<C: ConnectionTrait>(
    db: &C,
    tenant_id: Uuid,
) -> Result<Vec<conversation::Model>, sea_orm::DbErr> {
    conversation::Entity::find()
        .filter(conversation::Column::TenantId.eq(tenant_id))
        .all(db)
        .await
}

pub async fn create_message<C: ConnectionTrait>(
    db: &C,
    id: Uuid,
    tenant_id: Uuid,
    conversation_id: Uuid,
    content: String,
    message_type: String,
) -> Result<message::Model, sea_orm::DbErr> {
    let new_message = message::ActiveModel {
        id: Set(id),
        tenant_id: Set(tenant_id),
        conversation_id: Set(conversation_id),
        content: Set(content),
        message_type: Set(message_type),
        created_at: Set(chrono::Utc::now()),
    };
    new_message.insert(db).await
}

pub async fn get_conversation_messages<C: ConnectionTrait>(
    db: &C,
    tenant_id: Uuid,
    conversation_id: Uuid,
) -> Result<Vec<message::Model>, sea_orm::DbErr> {
    message::Entity::find()
        .filter(message::Column::TenantId.eq(tenant_id))
        .filter(message::Column::ConversationId.eq(conversation_id))
        .order_by_asc(message::Column::CreatedAt)
        .all(db)
        .await
}
