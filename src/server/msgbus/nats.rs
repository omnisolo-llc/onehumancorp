use super::{Bus, DistributedLock, Message};
use async_trait::async_trait;

#[allow(dead_code)]
pub struct NatsBus {
    client: async_nats::Client,
    kv: async_nats::jetstream::kv::Store,
}

#[allow(dead_code)]
impl NatsBus {
    pub async fn new(nats_url: &str) -> Result<Self, String> {
        let client = async_nats::connect(nats_url).await.map_err(|e| e.to_string())?;
        let js = async_nats::jetstream::new(client.clone());
        let kv = match js.get_key_value("bus_locks").await {
            Ok(store) => store,
            Err(_) => js.create_key_value(async_nats::jetstream::kv::Config {
                bucket: "bus_locks".to_string(),
                history: 1,
                ..Default::default()
            }).await.map_err(|e| e.to_string())?
        };

        Ok(NatsBus {
            client,
            kv,
        })
    }
}

#[async_trait]
impl Bus for NatsBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();

        let nats_topic = msg.topic.replace(":", ".");

        let mut retries = 0;
        loop {
            match self.client.publish(nats_topic.clone(), buf.clone().into()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if retries >= 3 {
                        return Err(e.to_string());
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * retries)).await;
                }
            }
        }
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use futures_util::StreamExt;

        let subscribe_topic = if topic.ends_with(':') {
            format!("{}>", topic.replace(":", "."))
        } else {
            topic.replace(":", ".")
        };
        let mut subscriber = self.client.subscribe(subscribe_topic).await.map_err(|e| e.to_string())?;

        let worker = tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                use prost::Message as ProstMessage;
                let m = Message::decode(&msg.payload[..]).unwrap_or_else(|_| Message { topic: msg.subject.to_string().replace(".", ":"), payload: vec![] });
                handler(m);
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }
}

#[async_trait]
impl DistributedLock for NatsBus {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let expires_at = chrono::Utc::now().timestamp() + ttl_seconds as i64;
        let payload = format!("{}:{}", owner, expires_at);

        if let Ok(Some(entry)) = self.kv.entry(resource).await {
            let entry_str = String::from_utf8_lossy(&entry.value);
            if let Some((stored_owner, stored_exp)) = entry_str.split_once(':') {
                if let Ok(exp) = stored_exp.parse::<i64>() {
                    if exp <= chrono::Utc::now().timestamp() || stored_owner == owner {
                        match self.kv.update(resource, payload.clone().into_bytes().into(), entry.revision).await {
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
                    // Update with immediately expired lock to allow atomic replacement
                    let payload = format!("{}:0", owner);
                    let _ = self.kv.update(resource, payload.into_bytes().into(), entry.revision).await;
                }
            }
        }
        Ok(())
    }
}
