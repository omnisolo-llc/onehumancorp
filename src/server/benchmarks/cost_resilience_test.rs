use std::time::{Duration, Instant};
use crate::pricing::steering::{CostSteerer, ModelTier};
use crate::pricing::context_manager::{ContextManager, ContextMessage};
use crate::pricing::rate_limit::{RedisRateLimiter, PlanTier};
use crate::services::billing::auditor::{CostAuditor, AuditEvent};
use ::server_pricing::calculator::CostConfig;

#[tokio::test]
async fn test_cost_resilience_under_load() {
    let config = CostConfig {
        cost_per_input_token: 0.0001,
        cost_per_output_token: 0.0002,
        ..Default::default()
    };
    let auditor = CostAuditor::new(config);

    let mut handles = Vec::new();
    for i in 0..1000 {
        let aud = auditor.clone();
        handles.push(tokio::spawn(async move {
            aud.record_event(AuditEvent {
                agent_id: format!("agent-{}", i % 10),
                input_tokens: 100,
                output_tokens: 50,
                cached_input_tokens: 0,
                local_embedding_tokens: 0,
            });
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    assert!((auditor.get_total_cost() - 20.0).abs() < 0.001);
}

#[test]
fn test_context_pruning_resilience() {
    let manager = ContextManager::new(1000, 5);
    let mut messages = Vec::new();
    messages.push(ContextMessage { role: "system".to_string(), content: "System".to_string() });

    for i in 0..100 {
        messages.push(ContextMessage {
            role: if i % 2 == 0 { "user" } else { "assistant" }.to_string(),
            content: format!("Message {}", i)
        });
    }

    let pruned = manager.prune_history(messages);

    assert_eq!(pruned.len(), 6);
    assert_eq!(pruned[0].role, "system");
    assert_eq!(pruned[5].content, "Message 99");
}

#[test]
fn test_steering_efficiency() {
    let tier = CostSteerer::steer("Analyze the whole architectural implementation and suggest a refactoring strategy", 1000);
    assert_eq!(tier, ModelTier::Premium);

    let tier = CostSteerer::steer("Hi", 1000);
    assert_eq!(tier, ModelTier::Economy);

    let tier = CostSteerer::steer("Analyze everything", 10);
    assert_eq!(tier, ModelTier::Economy);
}

#[tokio::test]
async fn test_rate_limit_saturation() {
    if let Ok(redis_url) = std::env::var("REDIS_URL") {
        let client = redis::Client::open(redis_url).unwrap();
        let limiter = RedisRateLimiter::new(client);
        let tid = "saturation-tenant";

        limiter.set_tenant_tier(tid, PlanTier::Free).await.unwrap();

        for _ in 0..100 {
            let res = limiter.record_action(tid, "agent-1").await.unwrap();
            assert!(res.is_allowed);
        }

        let res = limiter.record_action(tid, "agent-1").await.unwrap();
        assert!(res.is_allowed);
        assert!(res.soft_limit_reached);
        assert!(res.user_message.unwrap().contains("Free tier limit"));
    }
}

#[tokio::test]
async fn test_storage_quota_enforcement() {
     if let Ok(redis_url) = std::env::var("REDIS_URL") {
        let client = redis::Client::open(redis_url).unwrap();
        let limiter = RedisRateLimiter::new(client);
        let tid = "storage-test-tenant";

        limiter.set_tenant_tier(tid, PlanTier::Free).await.unwrap();

        // 400MB is OK
        let res = limiter.check_storage_quota(tid, 400 * 1024 * 1024).await.unwrap();
        assert!(!res.soft_limit_reached);

        // Another 200MB puts us at 600MB (> 500MB limit)
        let res = limiter.check_storage_quota(tid, 200 * 1024 * 1024).await.unwrap();
        assert!(res.soft_limit_reached);
        assert!(res.user_message.unwrap().contains("500MB storage"));
    }
}
