use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub metadata: Option<Json>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
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
