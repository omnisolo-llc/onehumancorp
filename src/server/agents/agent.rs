// Trivial change to force rebuild
use async_trait::async_trait;
use std::sync::Arc;
use tokio_stream::StreamExt;
use redis::AsyncCommands;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Status {
    IDLE,
    ACTIVE,
    #[serde(rename = "IN_MEETING")]
    InMeeting,
    BLOCKED,
    #[serde(rename = "WAITING_FOR_TOOLS")]
    WaitingForTools,
}

#[allow(dead_code)]

pub const EVENT_TASK: &str = "task";
#[allow(dead_code)]
pub const EVENT_STATUS: &str = "status";
#[allow(dead_code)]
pub const EVENT_HANDOFF: &str = "handoff";
#[allow(dead_code)]
pub const EVENT_CODE_REVIEWED: &str = "CodeReviewed";
#[allow(dead_code)]
pub const EVENT_TESTS_PASSED: &str = "TestsPassed";
#[allow(dead_code)]
pub const EVENT_SPEC_APPROVED: &str = "SpecApproved";
#[allow(dead_code)]
pub const EVENT_BLOCKER_RAISED: &str = "BlockerRaised";
#[allow(dead_code)]
pub const EVENT_BLOCKER_CLEARED: &str = "BlockerCleared";
#[allow(dead_code)]
pub const EVENT_PR_CREATED: &str = "PRCreated";
#[allow(dead_code)]
pub const EVENT_PR_MERGED: &str = "PRMerged";
#[allow(dead_code)]
pub const EVENT_DESIGN_REVIEWED: &str = "DesignReviewed";
#[allow(dead_code)]
pub const EVENT_APPROVAL_NEEDED: &str = "ApprovalNeeded";

#[allow(dead_code)]

pub const AVAILABLE_MCP_BUNDLES: &[&str] = &[
    "github",
];

#[async_trait]
#[allow(dead_code)]
pub trait Transport: Send + Sync {
    async fn send(&self, message: &[u8]) -> Result<(), String>;
    async fn receive(&self) -> Result<Vec<u8>, String>;
    async fn close(&self) -> Result<(), String>;
}

#[allow(dead_code)]

pub struct InProcessTransport<R, W> {
    reader: tokio::sync::Mutex<tokio::io::BufReader<R>>,
    writer: tokio::sync::Mutex<W>,
}

#[allow(dead_code)]

impl<R, W> InProcessTransport<R, W>
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
    W: tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    pub fn new(reader: R, writer: W) -> Self {
        InProcessTransport {
            reader: tokio::sync::Mutex::new(tokio::io::BufReader::new(reader)),
            writer: tokio::sync::Mutex::new(writer),
        }
    }
}

#[async_trait]
impl<R, W> Transport for InProcessTransport<R, W>
where
    R: tokio::io::AsyncRead + Unpin + Send + Sync + 'static,
    W: tokio::io::AsyncWrite + Unpin + Send + Sync + 'static,
{
    async fn send(&self, message: &[u8]) -> Result<(), String> {
        use tokio::io::AsyncWriteExt;
        let mut writer = self.writer.lock().await;
        writer.write_all(message).await.map_err(|e| e.to_string())?;
        writer.write_all(b"\n").await.map_err(|e| e.to_string())?;
        writer.flush().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn receive(&self) -> Result<Vec<u8>, String> {
        use tokio::io::AsyncBufReadExt;
        let mut reader = self.reader.lock().await;
        let mut line = String::new();
        reader.read_line(&mut line).await.map_err(|e| e.to_string())?;
        if line.ends_with('\n') {
            line.pop();
        }
        if line.ends_with('\r') {
            line.pop();
        }
        Ok(line.into_bytes())
    }

    async fn close(&self) -> Result<(), String> {
        Ok(())
    }
}

#[allow(dead_code)]

pub struct RedisPubSubTransport {
    client: redis::Client,
    publish_chan: String,
    pubsub: tokio::sync::Mutex<redis::aio::PubSub>,
}

#[allow(dead_code)]

impl RedisPubSubTransport {
    pub async fn new(client: redis::Client, publish_chan: &str, subscribe_chan: &str) -> Result<Self, String> {
        let mut pubsub = client.get_async_pubsub().await.map_err(|e| e.to_string())?;
        pubsub.subscribe(subscribe_chan).await.map_err(|e| e.to_string())?;
        
        Ok(RedisPubSubTransport {
            client,
            publish_chan: publish_chan.to_string(),
            pubsub: tokio::sync::Mutex::new(pubsub),
        })
    }
}

#[async_trait]
impl Transport for RedisPubSubTransport {
    async fn send(&self, message: &[u8]) -> Result<(), String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        con.publish::<_, _, ()>(&self.publish_chan, message).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn receive(&self) -> Result<Vec<u8>, String> {
        let mut pubsub = self.pubsub.lock().await;
        let mut stream = pubsub.on_message();
        if let Some(msg) = stream.next().await {
            let payload: Vec<u8> = msg.get_payload().map_err(|e| e.to_string())?;
            Ok(payload)
        } else {
            Err("stream closed".to_string())
        }
    }

    async fn close(&self) -> Result<(), String> {
        Ok(())
    }
}

#[allow(dead_code)]

pub trait AgentExt {
    fn base_system_prompt(&self) -> String;
}

impl AgentExt for crate::ohc::orchestration::Agent {
    fn base_system_prompt(&self) -> String {
        let mut prompt = format!(
            "You are an autonomous AI agent representing One Human Corp (OHC). You operate within the bounds of your Role: {}.\n",
            self.role
        );
        if std::env::var("OHC_STANDALONE").unwrap_or_default() == "true" {
            prompt += "\n# Memory Fallback (Standalone Mode)\n";
            prompt += "The directories .ohc/memory/auto/ and .ohc/memory/team/ already exist. Write state to them directly.\n";
        }
        prompt
    }
}
