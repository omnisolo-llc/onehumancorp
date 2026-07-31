use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub last_activity_at: Option<DateTimeWithTimeZone>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
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
    ChatMessages,
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
        Relation::ChatMessages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
