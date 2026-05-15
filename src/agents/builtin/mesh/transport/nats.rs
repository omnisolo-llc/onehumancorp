use super::MeshTransport;
use crate::proto::hub::TeammateMeshEvent as Message;
use async_trait::async_trait;

#[derive(Clone)]
pub struct NatsTransport {
    client: async_nats::Client,
    kv: async_nats::jetstream::kv::Store,
}

impl NatsTransport {
    pub async fn new(url: &str) -> Result<Self, String> {
        let client = async_nats::connect(url).await.map_err(|e| e.to_string())?;
        let js = async_nats::jetstream::new(client.clone());
        let kv = match js.get_key_value("mesh_locks").await {
            Ok(store) => store,
            Err(_) => js
                .create_key_value(async_nats::jetstream::kv::Config {
                    bucket: "mesh_locks".to_string(),
                    history: 1,
                    ..Default::default()
                })
                .await
                .map_err(|e| e.to_string())?,
        };

        Ok(Self { client, kv })
    }
}

#[async_trait]
impl MeshTransport for NatsTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        message.encode(&mut buf).map_err(|e| e.to_string())?;
        self.client
            .publish(topic.to_string(), buf.into())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use futures::StreamExt;
        use prost::Message as ProstMessage;

        let mut subscriber = self
            .client
            .subscribe(topic.to_string())
            .await
            .map_err(|e| e.to_string())?;

        let worker = tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                if let Ok(decoded) = Message::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        Ok(Box::new(move || {
            worker.abort();
        }))
    }

    async fn acquire_lock(
        &self,
        resource: &str,
        owner: &str,
        ttl_seconds: u64,
    ) -> Result<bool, String> {
        let expires_at = chrono::Utc::now().timestamp() + ttl_seconds as i64;
        let payload = format!("{}:{}", owner, expires_at);

        if let Ok(Some(entry)) = self.kv.entry(resource).await {
            let entry_str = String::from_utf8_lossy(&entry.value);
            if let Some((stored_owner, stored_exp)) = entry_str.split_once(':') {
                if let Ok(exp) = stored_exp.parse::<i64>() {
                    if exp <= chrono::Utc::now().timestamp() || stored_owner == owner {
                        match self
                            .kv
                            .update(
                                resource,
                                payload.clone().into_bytes().into(),
                                entry.revision,
                            )
                            .await
                        {
                            Ok(_) => return Ok(true),
                            Err(_) => return Ok(false),
                        }
                    } else {
                        return Ok(false);
                    }
                }
            }
        }

        match self.kv.create(resource, payload.into_bytes().into()).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("wrong last sequence") {
                    Ok(false)
                } else {
                    Err(e.to_string())
                }
            }
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        if let Ok(Some(entry)) = self.kv.entry(resource).await {
            let entry_str = String::from_utf8_lossy(&entry.value);
            if let Some((stored_owner, _)) = entry_str.split_once(':') {
                if stored_owner == owner {
                    let payload = format!("{}:0", owner);
                    let _ = self
                        .kv
                        .update(resource, payload.into_bytes().into(), entry.revision)
                        .await;
                }
            }
        }
        Ok(())
    }

    async fn register_presence(
        &self,
        agent_id: &str,
        status: &str,
        ttl_seconds: u64,
    ) -> Result<(), String> {
        let key = format!("presence_{}", agent_id);
        let expires_at = chrono::Utc::now().timestamp() + ttl_seconds as i64;
        let payload = format!("{}:{}", status, expires_at);
        self.kv
            .put(&key, payload.into_bytes().into())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let mut keys = self.kv.keys().await.map_err(|e| e.to_string())?;
        let mut agents = Vec::new();
        use futures::StreamExt;
        let now = chrono::Utc::now().timestamp();
        while let Some(Ok(key)) = keys.next().await {
            if key.starts_with("presence_") {
                if let Ok(Some(entry)) = self.kv.entry(&key).await {
                    let entry_str = String::from_utf8_lossy(&entry.value);
                    if let Some((status, stored_exp)) = entry_str.split_once(':') {
                        if let Ok(exp) = stored_exp.parse::<i64>() {
                            if exp > now {
                                let agent_id = key.strip_prefix("presence_").unwrap().to_string();
                                agents.push((agent_id, status.to_string()));
                            } else {
                                let _ = self.kv.delete(&key).await;
                            }
                        }
                    }
                }
            }
        }
        Ok(agents)
    }
}
