use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_inboxes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub channel_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chat_conversation::Entity")]
    ChatConversations,
}

impl Related<super::chat_conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatConversations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
