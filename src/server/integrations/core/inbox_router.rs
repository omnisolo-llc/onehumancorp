use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionStream {
    pub id: String,
    pub tenant_id: String,
    pub spiffe_identity: String,
    pub customer_profile: String,
    pub channel: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiedThread {
    pub thread_id: String,
    pub customer_profile: String,
    pub requires_human_escalation: bool,
}

pub struct InboxRouter {
    pub tenant_id: String,
    pub spiffe_identity: String,
    pub pool: PgPool,
}

impl InboxRouter {
    pub fn new(tenant_id: String, spiffe_identity: String, pool: PgPool) -> Self {
        Self { tenant_id, spiffe_identity, pool }
    }

    pub async fn ingest(&self, source: String, content: String) -> Result<InteractionStream, String> {
        let stream = InteractionStream {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: self.tenant_id.clone(),
            spiffe_identity: self.spiffe_identity.clone(),
            customer_profile: "anonymous".to_string(),
            channel: source.clone(),
            status: "pending".to_string(),
        };

        let query = format!(
            "INSERT INTO interactions (id, tenant_id, customer_id, channel, content) VALUES ('{}', '{}', '{}', '{}', '{}')",
            stream.id, stream.tenant_id, stream.customer_profile, stream.channel, content
        );

        match sqlx::query(&query).execute(&self.pool).await {
            Ok(_) => Ok(stream),
            Err(e) => Err(format!("Failed to persist interaction: {}", e)),
        }
    }
}

pub struct AmbassadorAgent {
    pub id: String,
    pub active: bool,
    pub pool: PgPool,
}

impl AmbassadorAgent {
    pub fn new(id: String, pool: PgPool) -> Self {
        Self { id, active: true, pool }
    }

    pub async fn process_stream(&self, stream: &InteractionStream, content: String) -> Result<UnifiedThread, String> {
        let draft_reply = format!("✨ Ambassador Draft: Thank you for your {} message! We will get back to you shortly.", stream.channel);

        let query = format!(
            "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
            uuid::Uuid::new_v4().to_string(), stream.tenant_id, stream.channel, content, draft_reply, "pending"
        );

        match sqlx::query(&query).execute(&self.pool).await {
            Ok(_) => Ok(UnifiedThread {
                thread_id: format!("thread-{}", stream.id),
                customer_profile: stream.customer_profile.clone(),
                requires_human_escalation: false,
            }),
            Err(e) => Err(format!("Failed to append thread to inbox: {}", e)),
        }
    }
}
