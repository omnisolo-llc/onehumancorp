use sea_orm::*;
use uuid::Uuid;
use async_trait::async_trait;

use crate::models::{channel, contact, conversation, inbox, message};

#[async_trait]
pub trait ChatRepository {
    async fn create_channel(&self, db: &DatabaseConnection, model: channel::ActiveModel) -> Result<channel::Model, DbErr>;
    async fn get_channel_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<channel::Model>, DbErr>;

    async fn create_inbox(&self, db: &DatabaseConnection, model: inbox::ActiveModel) -> Result<inbox::Model, DbErr>;
    async fn get_inbox_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<inbox::Model>, DbErr>;

    async fn create_contact(&self, db: &DatabaseConnection, model: contact::ActiveModel) -> Result<contact::Model, DbErr>;
    async fn get_contact_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<contact::Model>, DbErr>;

    async fn create_conversation(&self, db: &DatabaseConnection, model: conversation::ActiveModel) -> Result<conversation::Model, DbErr>;
    async fn get_conversation_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<conversation::Model>, DbErr>;

    async fn create_message(&self, db: &DatabaseConnection, model: message::ActiveModel) -> Result<message::Model, DbErr>;
    async fn get_messages_by_conversation(&self, db: &DatabaseConnection, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<message::Model>, DbErr>;
}

pub struct ChatRepositoryImpl;

#[async_trait]
impl ChatRepository for ChatRepositoryImpl {
    async fn create_channel(&self, db: &DatabaseConnection, model: channel::ActiveModel) -> Result<channel::Model, DbErr> {
        model.insert(db).await
    }

    async fn get_channel_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<channel::Model>, DbErr> {
        channel::Entity::find()
            .filter(channel::Column::Id.eq(id))
            .filter(channel::Column::TenantId.eq(tenant_id))
            .one(db)
            .await
    }

    async fn create_inbox(&self, db: &DatabaseConnection, model: inbox::ActiveModel) -> Result<inbox::Model, DbErr> {
        model.insert(db).await
    }

    async fn get_inbox_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<inbox::Model>, DbErr> {
        inbox::Entity::find()
            .filter(inbox::Column::Id.eq(id))
            .filter(inbox::Column::TenantId.eq(tenant_id))
            .one(db)
            .await
    }

    async fn create_contact(&self, db: &DatabaseConnection, model: contact::ActiveModel) -> Result<contact::Model, DbErr> {
        model.insert(db).await
    }

    async fn get_contact_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<contact::Model>, DbErr> {
        contact::Entity::find()
            .filter(contact::Column::Id.eq(id))
            .filter(contact::Column::TenantId.eq(tenant_id))
            .one(db)
            .await
    }

    async fn create_conversation(&self, db: &DatabaseConnection, model: conversation::ActiveModel) -> Result<conversation::Model, DbErr> {
        model.insert(db).await
    }

    async fn get_conversation_by_id(&self, db: &DatabaseConnection, tenant_id: Uuid, id: Uuid) -> Result<Option<conversation::Model>, DbErr> {
        conversation::Entity::find()
            .filter(conversation::Column::Id.eq(id))
            .filter(conversation::Column::TenantId.eq(tenant_id))
            .one(db)
            .await
    }

    async fn create_message(&self, db: &DatabaseConnection, model: message::ActiveModel) -> Result<message::Model, DbErr> {
        model.insert(db).await
    }

    async fn get_messages_by_conversation(&self, db: &DatabaseConnection, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<message::Model>, DbErr> {
        message::Entity::find()
            .filter(message::Column::ConversationId.eq(conversation_id))
            .filter(message::Column::TenantId.eq(tenant_id))
            .order_by_asc(message::Column::CreatedAt)
            .all(db)
            .await
    }
}
pub mod tests;
