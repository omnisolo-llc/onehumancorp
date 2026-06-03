use std::sync::Arc;
use sqlx::PgPool;

pub struct LocalizationHelper {
    pool: Arc<PgPool>,
}

impl LocalizationHelper {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Fetches a localized string for an AI agent's response.
    pub async fn get_localized_string(&self, tenant_id: &str, locale: &str, key: &str) -> Result<String, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let row = sqlx::query(
            "SELECT value FROM ohc_i18n_strings
             WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2 AND key = $3"
        )
        .bind(tenant_id)
        .bind(locale)
        .bind(key)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        match row {
            Some(r) => {
                use sqlx::Row;
                Ok(r.get("value"))
            }
            None => Err(format!("Key {} not found for locale {}", key, locale)),
        }
    }

    /// Fetches preferred currency for a tenant.
    pub async fn get_tenant_currency(&self, tenant_id: &str) -> Result<String, String> {
        // This assumes a tenant settings table exists or we use i18n_strings for it
        self.get_localized_string(tenant_id, "SYSTEM", "base_currency").await
            .or(Ok("USD".to_string()))
    }
}
