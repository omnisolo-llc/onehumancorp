use redis::AsyncCommands;

/// Dedicated transport for Agent Harness bidirectional communication
/// Uses separate channels for requests and responses to prevent 'echo chamber' bugs
pub struct CloudTransport {
    client: redis::Client,
    channel_req: String,
    channel_res: String,
}

impl CloudTransport {
    pub fn new(redis_url: &str, base_topic: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(CloudTransport {
            client,
            channel_req: format!("{}_req", base_topic),
            channel_res: format!("{}_res", base_topic),
        })
    }

    pub async fn dispatch_request(&self, payload: Vec<u8>) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let _: () = conn.publish(&self.channel_req, payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn listen_for_responses(&self, handler: impl Fn(Vec<u8>) + Send + Sync + 'static) -> Result<(), String> {
        let mut pubsub = self.client.get_async_pubsub().await.map_err(|e| e.to_string())?;
        pubsub.subscribe(&self.channel_res).await.map_err(|e| e.to_string())?;
        use futures_util::StreamExt;
        let mut stream = pubsub.into_on_message();

        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(buf) = msg.get_payload::<Vec<u8>>() {
                    handler(buf);
                }
            }
        });

        Ok(())
    }
}

// Integration into the executor or wrapper can be modeled here or inside `harness/executor.rs`