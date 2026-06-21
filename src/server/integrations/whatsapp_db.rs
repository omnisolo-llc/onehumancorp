use sqlx::{Pool, Postgres};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct WhatsappConnection {
    pub id: String,
    pub tenant_id: String,
    pub phone_number_id: String,
    pub waba_id: String,
    pub access_token: String,
    pub status: String,
}

pub async fn get_whatsapp_connection_by_phone(
    pool: &Pool<Postgres>,
    phone_number_id: &str,
) -> Result<Option<WhatsappConnection>, sqlx::Error> {
    sqlx::query_as::<_, WhatsappConnection>(
        "SELECT id, tenant_id, phone_number_id, waba_id, access_token, status FROM whatsapp_connections WHERE phone_number_id = $1"
    )
    .bind(phone_number_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_whatsapp_connection_by_tenant(
    pool: &Pool<Postgres>,
    tenant_id: &str,
) -> Result<Option<WhatsappConnection>, sqlx::Error> {
    sqlx::query_as::<_, WhatsappConnection>(
        "SELECT id, tenant_id, phone_number_id, waba_id, access_token, status FROM whatsapp_connections WHERE tenant_id = $1 LIMIT 1"
    )
    .bind(tenant_id)
    .fetch_optional(pool)
    .await
}