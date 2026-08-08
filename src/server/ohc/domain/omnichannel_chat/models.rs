use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_inboxes")]
pub struct InboxModel {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum InboxRelation {
    #[sea_orm(has_many = "ChannelEntity")]
    Channels,
    #[sea_orm(has_many = "ConversationEntity")]
    Conversations,
}

impl Related<ChannelEntity> for InboxEntity {
    fn to() -> RelationDef {
        InboxRelation::Channels.def()
    }
}

impl Related<ConversationEntity> for InboxEntity {
    fn to() -> RelationDef {
        InboxRelation::Conversations.def()
    }
}

impl ActiveModelBehavior for InboxActiveModel {}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_channels")]
pub struct ChannelModel {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum ChannelRelation {
    #[sea_orm(
        belongs_to = "InboxEntity",
        from = "ChannelColumn::InboxId",
        to = "InboxColumn::Id"
    )]
    Inbox,
}

impl Related<InboxEntity> for ChannelEntity {
    fn to() -> RelationDef {
        ChannelRelation::Inbox.def()
    }
}

impl ActiveModelBehavior for ChannelActiveModel {}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_contacts")]
pub struct ContactModel {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum ContactRelation {
    #[sea_orm(has_many = "ConversationEntity")]
    Conversations,
}

impl Related<ConversationEntity> for ContactEntity {
    fn to() -> RelationDef {
        ContactRelation::Conversations.def()
    }
}

impl ActiveModelBehavior for ContactActiveModel {}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_conversations")]
pub struct ConversationModel {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum ConversationRelation {
    #[sea_orm(
        belongs_to = "InboxEntity",
        from = "ConversationColumn::InboxId",
        to = "InboxColumn::Id"
    )]
    Inbox,
    #[sea_orm(
        belongs_to = "ContactEntity",
        from = "ConversationColumn::ContactId",
        to = "ContactColumn::Id"
    )]
    Contact,
    #[sea_orm(has_many = "MessageEntity")]
    Messages,
}

impl Related<InboxEntity> for ConversationEntity {
    fn to() -> RelationDef {
        ConversationRelation::Inbox.def()
    }
}

impl Related<ContactEntity> for ConversationEntity {
    fn to() -> RelationDef {
        ConversationRelation::Contact.def()
    }
}

impl Related<MessageEntity> for ConversationEntity {
    fn to() -> RelationDef {
        ConversationRelation::Messages.def()
    }
}

impl ActiveModelBehavior for ConversationActiveModel {}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_messages")]
pub struct MessageModel {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum MessageRelation {
    #[sea_orm(
        belongs_to = "ConversationEntity",
        from = "MessageColumn::ConversationId",
        to = "ConversationColumn::Id"
    )]
    Conversation,
}

impl Related<ConversationEntity> for MessageEntity {
    fn to() -> RelationDef {
        MessageRelation::Conversation.def()
    }
}

impl ActiveModelBehavior for MessageActiveModel {}
