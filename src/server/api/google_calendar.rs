use std::sync::Arc;
use server_integrations_google_calendar::client::{OAuthTokenResponse, TokenStorage};


pub struct GoogleCalendarTokenStorage {
    pub db: Arc<sqlx::PgPool>,
    pub tenant_id: String,
}

impl GoogleCalendarTokenStorage {
    pub fn new(db: Arc<sqlx::PgPool>, tenant_id: uuid::Uuid) -> Self {
        Self {
            db,
            tenant_id: tenant_id.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl TokenStorage for GoogleCalendarTokenStorage {
    async fn refresh_if_needed(
        &self,
        current_refresh_token: &str,
        _force: bool,
    ) -> Result<Option<OAuthTokenResponse>, String> {
        let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();

        let new_token =
            server_integrations_google_calendar::client::RealGoogleCalendarClient::refresh_token(
                &client_id,
                &client_secret,
                current_refresh_token,
            )
            .await?;

        let pool = &*self.db;
        let expires_at = new_token
            .expires_in
            .map(|secs| chrono::Utc::now() + chrono::Duration::seconds(secs as i64));

        sqlx::query(
            r#"
            UPDATE calendar_integrations
            SET access_token = $1,
                expires_at = $2,
                updated_at = NOW()
            WHERE tenant_id = $3 AND provider = 'google_calendar'
            "#,
        )
        .bind(new_token.access_token.clone())
        .bind(expires_at)
        .bind(&self.tenant_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update token: {}", e))?;

        Ok(Some(new_token))
    }

    async fn mark_disconnected(&self) -> Result<(), String> {
        let pool = &*self.db;
        sqlx::query(
            r#"
            UPDATE calendar_integrations
            SET access_token = '',
                refresh_token = NULL,
                sync_token = NULL,
                expires_at = NULL,
                updated_at = NOW()
            WHERE tenant_id = $1 AND provider = 'google_calendar'
            "#,
        )
        .bind(&self.tenant_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to clear token: {}", e))?;

        Ok(())
    }
}
