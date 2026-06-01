use sqlx::{PgPool, Error};
use super::models::{LocaleConfig, LocalizedContent, ConversationMessage};

pub struct LocalizationRepo {
    pool: PgPool,
}

impl LocalizationRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_locale_config(&self, tenant_id: &str) -> Result<Option<LocaleConfig>, Error> {
        let config = sqlx::query_as::<_, LocaleConfig>(
            r#"
            SELECT tenant_id, primary_locale, supported_locales, auto_translate, created_at, updated_at
            FROM locale_configs
            WHERE tenant_id = $1
            "#,
        )
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(config)
    }

    pub async fn upsert_locale_config(&self, config: &LocaleConfig) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO locale_configs (tenant_id, primary_locale, supported_locales, auto_translate)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (tenant_id) DO UPDATE SET
                primary_locale = EXCLUDED.primary_locale,
                supported_locales = EXCLUDED.supported_locales,
                auto_translate = EXCLUDED.auto_translate,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(&config.tenant_id)
        .bind(&config.primary_locale)
        .bind(&config.supported_locales)
        .bind(config.auto_translate)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_localized_content(&self, tenant_id: &str, entity_id: &str, entity_type: &str, locale: &str) -> Result<Option<LocalizedContent>, Error> {
        let content = sqlx::query_as::<_, LocalizedContent>(
            r#"
            SELECT id, tenant_id, entity_id, entity_type, locale, localized_name, localized_desc, created_at, updated_at
            FROM localized_contents
            WHERE tenant_id = $1 AND entity_id = $2 AND entity_type = $3 AND locale = $4
            "#,
        )
        .bind(tenant_id)
        .bind(entity_id)
        .bind(entity_type)
        .bind(locale)
        .fetch_optional(&self.pool)
        .await?;

        Ok(content)
    }

    pub async fn upsert_localized_content(&self, content: &LocalizedContent) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO localized_contents (tenant_id, entity_id, entity_type, locale, localized_name, localized_desc)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tenant_id, entity_id, entity_type, locale) DO UPDATE SET
                localized_name = EXCLUDED.localized_name,
                localized_desc = EXCLUDED.localized_desc,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(&content.tenant_id)
        .bind(&content.entity_id)
        .bind(&content.entity_type)
        .bind(&content.locale)
        .bind(&content.localized_name)
        .bind(&content.localized_desc)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn insert_conversation_message(&self, message: &ConversationMessage) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO conversation_messages (id, tenant_id, conversation_id, original_text, original_locale, translated_text, target_locale, sender_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&message.id)
        .bind(&message.tenant_id)
        .bind(&message.conversation_id)
        .bind(&message.original_text)
        .bind(&message.original_locale)
        .bind(&message.translated_text)
        .bind(&message.target_locale)
        .bind(&message.sender_type)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
