use sqlx::{Pool, Postgres, Sqlite, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ToolIntegration {
    pub id: String, // Uuid as string to support both easily
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub api_url: Option<String>,
    pub integration_code: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub enum ScoutDb {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}

impl ScoutDb {
    pub fn new_pg(pool: Pool<Postgres>) -> Self {
        Self::Postgres(pool)
    }

    pub fn new_sqlite(pool: Pool<Sqlite>) -> Self {
        Self::Sqlite(pool)
    }

    pub async fn save_integration(&self, integration: &ToolIntegration) -> Result<(), sqlx::Error> {
        match self {
            Self::Postgres(pool) => {
                let id = Uuid::parse_str(&integration.id).unwrap_or_else(|_| Uuid::new_v4());
                let query = r#"
                    INSERT INTO tool_integrations (id, tenant_id, name, description, api_url, integration_code, status, created_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#;
                let mut tx = pool.begin().await?;
                sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                    .bind(&integration.tenant_id)
                    .execute(&mut *tx)
                    .await?;
                sqlx::query(query)
                    .bind(id)
                    .bind(&integration.tenant_id)
                    .bind(&integration.name)
                    .bind(&integration.description)
                    .bind(&integration.api_url)
                    .bind(&integration.integration_code)
                    .bind(&integration.status)
                    .bind(integration.created_at)
                    .execute(&mut *tx)
                    .await?;
                tx.commit().await?;
            }
            Self::Sqlite(pool) => {
                let query = r#"
                    INSERT INTO tool_integrations (id, tenant_id, name, description, api_url, integration_code, status, created_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#;
                // sqlite uses string for uuid normally
                sqlx::query(query)
                    .bind(&integration.id)
                    .bind(&integration.tenant_id)
                    .bind(&integration.name)
                    .bind(&integration.description)
                    .bind(&integration.api_url)
                    .bind(&integration.integration_code)
                    .bind(&integration.status)
                    .bind(integration.created_at)
                    .execute(pool)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn get_integration(&self, id: &str, tenant_id: Option<&str>) -> Result<Option<ToolIntegration>, sqlx::Error> {
        match self {
            Self::Postgres(pool) => {
                let uuid = match Uuid::parse_str(id) {
                    Ok(u) => u,
                    Err(_) => return Ok(None),
                };
                let mut tx = pool.begin().await?;
                if let Some(tenant) = tenant_id {
                    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                        .bind(tenant)
                        .execute(&mut *tx)
                        .await?;
                }
                let query = "SELECT id, tenant_id, name, description, api_url, integration_code, status, created_at FROM tool_integrations WHERE id = $1";
                let row = match sqlx::query(query).bind(uuid).fetch_optional(&mut *tx).await? {
                    Some(r) => r,
                    None => {
                        tx.rollback().await?;
                        return Ok(None);
                    }
                };
                tx.commit().await?;
                let id_uuid: Uuid = row.try_get("id")?;
                Ok(Some(ToolIntegration {
                    id: id_uuid.to_string(),
                    tenant_id: row.try_get("tenant_id")?,
                    name: row.try_get("name")?,
                    description: row.try_get("description")?,
                    api_url: row.try_get("api_url")?,
                    integration_code: row.try_get("integration_code")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                }))
            }
            Self::Sqlite(pool) => {
                let row = if let Some(tenant) = tenant_id {
                    let query = "SELECT id, tenant_id, name, description, api_url, integration_code, status, created_at FROM tool_integrations WHERE id = ? AND tenant_id = ?";
                    match sqlx::query(query).bind(id).bind(tenant).fetch_optional(pool).await? {
                        Some(r) => r,
                        None => return Ok(None),
                    }
                } else {
                    let query = "SELECT id, tenant_id, name, description, api_url, integration_code, status, created_at FROM tool_integrations WHERE id = ?";
                    match sqlx::query(query).bind(id).fetch_optional(pool).await? {
                        Some(r) => r,
                        None => return Ok(None),
                    }
                };
                let id_str: String = row.try_get("id")?;
                Ok(Some(ToolIntegration {
                    id: id_str,
                    tenant_id: row.try_get("tenant_id")?,
                    name: row.try_get("name")?,
                    description: row.try_get("description")?,
                    api_url: row.try_get("api_url")?,
                    integration_code: row.try_get("integration_code")?,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                }))
            }
        }
    }
}
