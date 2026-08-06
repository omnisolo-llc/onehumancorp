use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conversations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub conversation_id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub last_activity_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::inbox::Entity",
        from = "Column::InboxId",
        to = "super::inbox::Column::InboxId"
    )]
    Inbox,
    #[sea_orm(
        belongs_to = "super::contact::Entity",
        from = "Column::ContactId",
        to = "super::contact::Column::ContactId"
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
