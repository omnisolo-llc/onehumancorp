use crate::api::twilio_voice::VoiceContextRouter;
use server_domain::routing::CentrifugeNode;
use sqlx::SqlitePool;
use std::sync::Arc;

#[tokio::test]
async fn test_voice_context_router_init() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let _router = VoiceContextRouter::new(
        pool,
        Arc::new(CentrifugeNode::default()),
        "test_config".to_string(),
    );
    assert!(true);
}
