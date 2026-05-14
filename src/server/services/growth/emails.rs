
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use ::server_ohc::orchestration::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub subject: String,
    pub html_content: String,
    pub plain_text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct EmailMarketingService {
    pool: PgPool,
}

impl EmailMarketingService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
