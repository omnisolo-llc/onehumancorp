use sqlx::{Pool, Postgres, Sqlite};
use serde_json::Value;

pub enum PoolWrapper {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}

pub struct OnboardingRepository {
    pool: PoolWrapper,
}

impl OnboardingRepository {
    pub fn new_postgres(pool: Pool<Postgres>) -> Self {
        OnboardingRepository { pool: PoolWrapper::Postgres(pool) }
    }

    pub fn new_sqlite(pool: Pool<Sqlite>) -> Self {
        OnboardingRepository { pool: PoolWrapper::Sqlite(pool) }
    }

    pub async fn save_state(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
        step: i32,
        state: Value
    ) -> Result<(), String> {
        match &self.pool {
            PoolWrapper::Postgres(p) => {
                sqlx::query(
                    "INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json)                      VALUES ($1, $2, $3, $4, $5)                      ON CONFLICT (tenant_id, organization_id) DO UPDATE                      SET state_json = EXCLUDED.state_json,                          current_step = EXCLUDED.current_step,                          updated_at = CURRENT_TIMESTAMP"
                )
                .bind(tenant_id)
                .bind(organization_id)
                .bind(user_id)
                .bind(step)
                .bind(state)
                .execute(p)
                .await
                .map_err(|e| e.to_string())?;
            }
            PoolWrapper::Sqlite(p) => {
                // SQLite schema might slightly differ in Standalone mode as seen in lib.rs
                sqlx::query(
                    "INSERT INTO onboarding_state (tenant_id, user_id, current_step, state_json)                      VALUES ($1, $2, $3, $4)                      ON CONFLICT (tenant_id, user_id) DO UPDATE                      SET state_json = EXCLUDED.state_json,                          current_step = EXCLUDED.current_step,                          updated_at = CURRENT_TIMESTAMP"
                )
                .bind(tenant_id)
                .bind(user_id)
                .bind(step)
                .bind(state.to_string())
                .execute(p)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_state(&self, tenant_id: &str) -> Result<Option<Value>, String> {
        match &self.pool {
            PoolWrapper::Postgres(p) => {
                let row = sqlx::query("SELECT state_json FROM onboarding_state WHERE tenant_id = $1")
                    .bind(tenant_id)
                    .fetch_optional(p)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(row.map(|r| {
                    use sqlx::Row;
                    r.get("state_json")
                }))
            }
            PoolWrapper::Sqlite(p) => {
                let row = sqlx::query("SELECT state_json FROM onboarding_state WHERE tenant_id = $1")
                    .bind(tenant_id)
                    .fetch_optional(p)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(row.and_then(|r| {
                    use sqlx::Row;
                    let s: String = r.get("state_json");
                    serde_json::from_str(&s).ok()
                }))
            }
        }
    }

    pub async fn delete_state(&self, tenant_id: &str) -> Result<(), String> {
        match &self.pool {
            PoolWrapper::Postgres(p) => {
                sqlx::query("DELETE FROM onboarding_state WHERE tenant_id = $1")
                    .bind(tenant_id)
                    .execute(p)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            PoolWrapper::Sqlite(p) => {
                sqlx::query("DELETE FROM onboarding_state WHERE tenant_id = $1")
                    .bind(tenant_id)
                    .execute(p)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}
