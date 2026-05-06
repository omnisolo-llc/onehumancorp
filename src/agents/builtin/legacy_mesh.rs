#![allow(dead_code)]

use std::time::Duration;
use uuid::Uuid;
use sqlx::Row;

pub struct DistributedLock {
    client: redis::Client,
    key: String,
    value: String,
}

impl DistributedLock {
    pub fn new(client: redis::Client, key: &str) -> Self {
        DistributedLock {
            client,
            key: format!("lock:{}", key),
            value: Uuid::new_v4().to_string(),
        }
    }

    pub async fn acquire(&self, timeout: Duration, expiration: Duration) -> Result<(), String> {
        let con_future = self.client.get_multiplexed_async_connection();
        let con_result = tokio::time::timeout(std::time::Duration::from_secs(2), con_future).await;
        let mut con = match con_result {
            Ok(Ok(c)) => c,
            Ok(Err(e)) => return Err(e.to_string()),
            Err(_) => return Err("timeout connecting to redis".to_string()),
        };
        let start = std::time::Instant::now();
        
        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let res: Option<String> = redis::cmd("SET")
                .arg(&self.key)
                .arg(&self.value)
                .arg("NX")
                .arg("PX")
                .arg(expiration.as_millis() as u64)
                .query_async(&mut con)
                .await
                .map_err(|e| e.to_string())?;

            if res.is_some() {
                return Ok(());
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn release(&self) -> Result<(), String> {
        let con_future = self.client.get_multiplexed_async_connection();
        let con_result = tokio::time::timeout(std::time::Duration::from_secs(2), con_future).await;
        let mut con = match con_result {
            Ok(Ok(c)) => c,
            Ok(Err(e)) => return Err(e.to_string()),
            Err(_) => return Err("timeout connecting to redis".to_string()),
        };
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let res: i32 = script.key(&self.key).arg(&self.value).invoke_async(&mut con).await.map_err(|e| e.to_string())?;
        
        if res == 1 {
            Ok(())
        } else {
            Err("failed to release lock: not owner or lock expired".to_string())
        }
    }
}

pub struct Mission {
    pub mission_id: String,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub priority: String,
    pub payload: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn claim_mission(pool: &sqlx::PgPool, agent_id: &str) -> Result<Option<Mission>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let query = r#"
        UPDATE ohc_tasks.mission_queue
        SET status = 'IN_PROGRESS',
            assigned_agent = $1,
            updated_at = NOW()
        WHERE mission_id = (
            SELECT mission_id
            FROM ohc_tasks.mission_queue
            WHERE status = 'QUEUED'
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        )
        RETURNING mission_id, title, status, assigned_agent, priority, payload, created_at, updated_at
    "#;

    let row = sqlx::query(query)
        .bind(agent_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(row) = row {
        let payload_str: String = row.get("payload");
        let m = Mission {
            mission_id: row.get("mission_id"),
            title: row.get("title"),
            status: row.get("status"),
            assigned_agent: row.get("assigned_agent"),
            priority: row.get("priority"),
            payload: serde_json::from_str(&payload_str).unwrap_or_default(),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(Some(m))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_claim_mission_no_db() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy").unwrap();
        let res = claim_mission(&pool, "agent-1").await;
        // Should fail because table doesn't exist or connection fails on execution!
        assert!(res.is_err());
    }
}
