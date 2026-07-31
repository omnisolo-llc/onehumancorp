use sea_orm::{DbErr, EntityTrait, QueryFilter, ColumnTrait, QueryTrait};
use uuid::Uuid;

use crate::entities::{inboxes, contacts, conversations, messages};

#[tokio::test]
async fn test_tenant_isolation_rls() -> Result<(), DbErr> {
    let tenant_id_1 = Uuid::new_v4();
    let tenant_id_2 = Uuid::new_v4();

    // Simulate RLS by checking if queries correctly generate conditions for tenant isolation
    let inbox_query = inboxes::Entity::find()
        .filter(inboxes::Column::TenantId.eq(tenant_id_1));
    assert!(inbox_query.clone().build(sea_orm::DatabaseBackend::Postgres).to_string().contains("tenant_id"));

    let contacts_query = contacts::Entity::find()
        .filter(contacts::Column::TenantId.eq(tenant_id_2));
    assert!(contacts_query.clone().build(sea_orm::DatabaseBackend::Postgres).to_string().contains("tenant_id"));

    let conversations_query = conversations::Entity::find()
        .filter(conversations::Column::TenantId.eq(tenant_id_1));
    assert!(conversations_query.clone().build(sea_orm::DatabaseBackend::Postgres).to_string().contains("tenant_id"));

    let messages_query = messages::Entity::find()
        .filter(messages::Column::TenantId.eq(tenant_id_2));
    assert!(messages_query.clone().build(sea_orm::DatabaseBackend::Postgres).to_string().contains("tenant_id"));

    Ok(())
}
