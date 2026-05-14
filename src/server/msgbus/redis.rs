use super::{Bus, DistributedLock, Message};
use async_trait::async_trait;

#[allow(dead_code)]
pub struct RedisBus {
    client: redis::Client,
    publish_conn: tokio::sync::Mutex<redis::aio::MultiplexedConnection>,
}

#[allow(dead_code)]
impl RedisBus {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let publish_conn = client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

        Ok(RedisBus {
            client,
            publish_conn: tokio::sync::Mutex::new(publish_conn),
        })
    }
}

#[async_trait]
impl Bus for RedisBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();

        let mut retries = 0;
        loop {
            match redis::cmd("PUBLISH")
                .arg(&msg.topic)
                .arg(&buf)
                .query_async::<()>(&mut *conn)
                .await {
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

        let mut pubsub = self.client.get_async_pubsub().await.map_err(|e| e.to_string())?;
        if topic.ends_with(':') {
            pubsub.psubscribe(format!("{}*", topic)).await.map_err(|e| e.to_string())?;
        } else {
            pubsub.subscribe(&topic).await.map_err(|e| e.to_string())?;
        }
        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(buf) = msg.get_payload::<Vec<u8>>() {
                    use prost::Message as ProstMessage;
                    let topic_name = msg.get_channel_name().to_string();
                    let m = Message::decode(&buf[..]).unwrap_or_else(|_| Message { topic: topic_name, payload: vec![] });
                    handler(m);
                }
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }
}


#[async_trait]
impl DistributedLock for RedisBus {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("lock:{}", resource);
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                redis.call("set", KEYS[1], ARGV[1], "EX", ARGV[2])
                return 1
            elseif redis.call("set", KEYS[1], ARGV[1], "NX", "EX", ARGV[2]) then
                return 1
            else
                return 0
            end
        "#);
        let res: i32 = script.key(&key).arg(owner).arg(ttl_seconds).invoke_async(&mut *conn).await.map_err(|e| e.to_string())?;
        Ok(res == 1)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("lock:{}", resource);
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let _: i32 = script.key(&key).arg(owner).invoke_async(&mut *conn).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
