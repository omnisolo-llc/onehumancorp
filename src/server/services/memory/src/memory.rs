use redis::{Client, Commands, RedisResult};
use std::time::Duration;

pub struct AgentMemoryService {
    client: Client,
}

impl AgentMemoryService {
    pub fn new(client: Client) -> Self {
        AgentMemoryService { client }
    }

    fn key(tenant_id: &str, session_id: &str) -> String {
        format!("ohc:mem:{}:{}", tenant_id, session_id)
    }

    pub fn save_episodic_memory(
        &self,
        tenant_id: &str,
        session_id: &str,
        context: &str,
    ) -> RedisResult<()> {
        let mut con = self.client.get_connection()?;
        let key = Self::key(tenant_id, session_id);

        let _: () = redis::cmd("SETEX")
            .arg(key)
            .arg(7 * 24 * 60 * 60) // 7 days in seconds
            .arg(context)
            .query(&mut con)?;

        Ok(())
    }

    pub fn retrieve_recent_memory(
        &self,
        tenant_id: &str,
        session_id: &str,
    ) -> RedisResult<Option<String>> {
        let mut con = self.client.get_connection()?;
        let key = Self::key(tenant_id, session_id);

        let val: Option<String> = con.get(key)?;
        Ok(val)
    }
}
