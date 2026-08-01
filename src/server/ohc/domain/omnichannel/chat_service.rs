use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use server_persistence::entities::{contact, conversation, inbox, message};
use uuid::Uuid;
use chrono::Utc;

pub struct ChatService {
    db: DatabaseConnection,
}

impl ChatService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_inboxes(&self, tenant_id: Uuid) -> Result<Vec<inbox::Model>, sea_orm::DbErr> {
        inbox::Entity::find()
            .filter(inbox::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
        channel_type: String,
    ) -> Result<inbox::Model, sea_orm::DbErr> {
        let active_model = inbox::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            name: Set(name),
            channel_type: Set(channel_type),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        active_model.insert(&self.db).await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: String,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<contact::Model, sea_orm::DbErr> {
        let active_model = contact::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            name: Set(name),
            email: Set(email),
            phone: Set(phone),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        active_model.insert(&self.db).await
    }

    pub async fn list_contacts(&self, tenant_id: Uuid) -> Result<Vec<contact::Model>, sea_orm::DbErr> {
        contact::Entity::find()
            .filter(contact::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn create_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        status: String,
    ) -> Result<conversation::Model, sea_orm::DbErr> {
        let active_model = conversation::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            inbox_id: Set(inbox_id),
            contact_id: Set(contact_id),
            status: Set(status),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        active_model.insert(&self.db).await
    }

    pub async fn list_conversations(&self, tenant_id: Uuid) -> Result<Vec<conversation::Model>, sea_orm::DbErr> {
        conversation::Entity::find()
            .filter(conversation::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await
    }

    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
        message_type: String,
    ) -> Result<message::Model, sea_orm::DbErr> {
        let active_model = message::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            conversation_id: Set(conversation_id),
            content: Set(content),
            message_type: Set(message_type),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        };
        active_model.insert(&self.db).await
    }

    pub async fn list_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<message::Model>, sea_orm::DbErr> {
        message::Entity::find()
            .filter(message::Column::TenantId.eq(tenant_id))
            .filter(message::Column::ConversationId.eq(conversation_id))
            .all(&self.db)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Database, DbBackend, DatabaseConnection, Schema};
    use sea_orm::Statement;

    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();

        let schema = Schema::new(DbBackend::Sqlite);

        // Setup inbox table
        let mut inbox_table = schema.create_table_from_entity(inbox::Entity);
        db.execute(DbBackend::Sqlite.build(&inbox_table)).await.unwrap();

        // Setup contact table
        let mut contact_table = schema.create_table_from_entity(contact::Entity);
        db.execute(DbBackend::Sqlite.build(&contact_table)).await.unwrap();

        // Setup conversation table
        let mut conversation_table = schema.create_table_from_entity(conversation::Entity);
        db.execute(DbBackend::Sqlite.build(&conversation_table)).await.unwrap();

        // Setup message table
        let mut message_table = schema.create_table_from_entity(message::Entity);
        db.execute(DbBackend::Sqlite.build(&message_table)).await.unwrap();

        db
    }

    #[tokio::test]
    async fn test_tenant_isolation() {
        let db = setup_test_db().await;
        let service = ChatService::new(db);

        let tenant1_id = Uuid::new_v4();
        let tenant2_id = Uuid::new_v4();

        service.create_inbox(tenant1_id, "Maya's IG".to_string(), "instagram".to_string()).await.unwrap();
        service.create_inbox(tenant2_id, "Carlos's WA".to_string(), "whatsapp".to_string()).await.unwrap();

        let tenant1_inboxes = service.list_inboxes(tenant1_id).await.unwrap();
        assert_eq!(tenant1_inboxes.len(), 1);
        assert_eq!(tenant1_inboxes[0].name, "Maya's IG");

        let tenant2_inboxes = service.list_inboxes(tenant2_id).await.unwrap();
        assert_eq!(tenant2_inboxes.len(), 1);
        assert_eq!(tenant2_inboxes[0].name, "Carlos's WA");
    }
}
