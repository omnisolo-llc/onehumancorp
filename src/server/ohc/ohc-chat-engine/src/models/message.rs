use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub message_id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub sender_type: String,
    pub content: String,
    pub message_type: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::conversation::Entity",
        from = "Column::ConversationId",
        to = "super::conversation::Column::ConversationId"
    )]
    Conversation,
    #[sea_orm(has_many = "super::attachment::Entity")]
    Attachment,
}

impl Related<super::conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversation.def()
    }
}

impl Related<super::attachment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attachment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
