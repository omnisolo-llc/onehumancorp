use sqlx::{Pool, Postgres, Error};
use super::models::BrandVoiceProfile;

pub struct BrandVoiceRepo {
    pool: Pool<Postgres>,
}

impl BrandVoiceRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_by_tenant_id(&self, tenant_id: &str) -> Result<Option<BrandVoiceProfile>, Error> {
        sqlx::query_as!(
            BrandVoiceProfile,
            r#"
            SELECT id, tenant_id, tone_descriptors, vocabulary_preferences, specific_knowledge_facts, emoji_usage_level, created_at, updated_at
            FROM brand_voice_profiles
            WHERE tenant_id = $1
            "#,
            tenant_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn upsert(&self, profile: &BrandVoiceProfile) -> Result<BrandVoiceProfile, Error> {
        sqlx::query_as!(
            BrandVoiceProfile,
            r#"
            INSERT INTO brand_voice_profiles (tenant_id, tone_descriptors, vocabulary_preferences, specific_knowledge_facts, emoji_usage_level)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id)
            DO UPDATE SET
                tone_descriptors = EXCLUDED.tone_descriptors,
                vocabulary_preferences = EXCLUDED.vocabulary_preferences,
                specific_knowledge_facts = EXCLUDED.specific_knowledge_facts,
                emoji_usage_level = EXCLUDED.emoji_usage_level,
                updated_at = NOW()
            RETURNING id, tenant_id, tone_descriptors, vocabulary_preferences, specific_knowledge_facts, emoji_usage_level, created_at, updated_at
            "#,
            profile.tenant_id,
            profile.tone_descriptors,
            profile.vocabulary_preferences,
            profile.specific_knowledge_facts,
            profile.emoji_usage_level
        )
        .fetch_one(&self.pool)
        .await
    }
}
