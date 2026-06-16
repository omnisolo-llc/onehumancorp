use sqlx::{PgPool, Error as SqlxError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Escalation {
    pub id: Uuid,
    pub tenant_id: String,
    pub task_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub summary: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct LocationRepository {
    pool: PgPool,
}

impl LocationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_location(&self, tenant_id: &str, name: &str) -> Result<Location, SqlxError> {
        let location = sqlx::query_as!(
            Location,
            r#"
            INSERT INTO locations (tenant_id, name)
            VALUES ($1, $2)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#,
            tenant_id,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(location)
    }

    pub async fn get_locations_by_tenant(&self, tenant_id: &str) -> Result<Vec<Location>, SqlxError> {
        let locations = sqlx::query_as!(
            Location,
            r#"
            SELECT id, tenant_id, name, created_at, updated_at
            FROM locations
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(locations)
    }

    pub async fn create_escalation(&self, tenant_id: &str, task_id: Option<Uuid>, location_id: Option<Uuid>, summary: &str) -> Result<Escalation, SqlxError> {
        let escalation = sqlx::query_as!(
            Escalation,
            r#"
            INSERT INTO escalations (tenant_id, task_id, location_id, summary, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, tenant_id, task_id, location_id, summary, status, created_at, updated_at
            "#,
            tenant_id,
            task_id,
            location_id,
            summary
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(escalation)
    }

    pub async fn get_escalations_by_tenant(&self, tenant_id: &str) -> Result<Vec<Escalation>, SqlxError> {
        let escalations = sqlx::query_as!(
            Escalation,
            r#"
            SELECT id, tenant_id, task_id, location_id, summary, status, created_at, updated_at
            FROM escalations
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(escalations)
    }

    pub async fn update_escalation_status(&self, tenant_id: &str, escalation_id: Uuid, status: &str) -> Result<Escalation, SqlxError> {
        let escalation = sqlx::query_as!(
            Escalation,
            r#"
            UPDATE escalations
            SET status = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, task_id, location_id, summary, status, created_at, updated_at
            "#,
            status,
            escalation_id,
            tenant_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(escalation)
    }
}
