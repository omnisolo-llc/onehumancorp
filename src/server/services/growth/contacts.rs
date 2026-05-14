
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use ::server_ohc::orchestration::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub org_id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

pub struct ContactService {
    pool: PgPool,
}

impl ContactService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
