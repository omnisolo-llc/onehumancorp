use redis::AsyncCommands;
use std::sync::Arc;

pub struct PresenceManager {
    client: redis::Client,
}

impl PresenceManager {
    pub fn new(client: redis::Client) -> Self {
        PresenceManager { client }
    }

    pub async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = format!("presence:{}", agent_id);
        
        redis::cmd("SET")
            .arg(&key)
            .arg(status)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async::<()>(&mut con)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }

    pub async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        
        let keys: Vec<String> = con.keys("presence:*").await.map_err(|e| e.to_string())?;
        
        let mut result = Vec::new();
        for key in keys {
            let agent_id = key.trim_start_matches("presence:").to_string();
            let status: String = con.get(&key).await.map_err(|e| e.to_string())?;
            result.push((agent_id, status));
        }
        
        Ok(result)
    }
}
