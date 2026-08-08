pub mod contacts;
pub mod conversations;
pub mod inboxes;
pub mod messages;
pub mod entities;

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{
        prelude::*, DbBackend, MockDatabase, MockExecResult,
    };
    use uuid::Uuid;
    use serde_json::json;
    use chrono::Utc;

    #[tokio::test]
    async fn test_contact_crud() {
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
            ])
            .append_query_results([
                vec![contacts::Model {
                    id: Uuid::new_v4(),
                    tenant_id: Uuid::new_v4(),
                    name: Some("John Doe".to_string()),
                    email: Some("john@example.com".to_string()),
                    phone: None,
                    custom_attributes: json!({}),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }],
                vec![contacts::Model {
                    id: Uuid::new_v4(),
                    tenant_id: Uuid::new_v4(),
                    name: Some("John Doe Updated".to_string()),
                    email: Some("john@example.com".to_string()),
                    phone: None,
                    custom_attributes: json!({}),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }],
            ])
            .into_connection();

        let tenant_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let mut contact = contacts::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            tenant_id: sea_orm::ActiveValue::Set(tenant_id),
            name: sea_orm::ActiveValue::Set(Some("John Doe".to_string())),
            email: sea_orm::ActiveValue::NotSet,
            phone: sea_orm::ActiveValue::NotSet,
            custom_attributes: sea_orm::ActiveValue::Set(json!({})),
            created_at: sea_orm::ActiveValue::Set(Utc::now()),
            updated_at: sea_orm::ActiveValue::Set(Utc::now()),
        };

        // Create
        let result = contacts::Entity::insert(contact.clone()).exec(&db).await.unwrap();
        assert_eq!(result.last_insert_id, id);

        // Read (and check tenant invariant simulated here)
        let found = contacts::Entity::find()
            .filter(contacts::Column::TenantId.eq(tenant_id))
            .filter(contacts::Column::Id.eq(id))
            .one(&db)
            .await
            .unwrap();
        assert!(found.is_some());

        // Update
        contact.name = sea_orm::ActiveValue::Set(Some("John Doe Updated".to_string()));
        let update_result = contacts::Entity::update(contact).exec(&db).await;
        assert!(update_result.is_ok());

        // Delete
        let delete_result = contacts::Entity::delete_by_id(id).exec(&db).await.unwrap();
        assert_eq!(delete_result.rows_affected, 1);
    }
}
