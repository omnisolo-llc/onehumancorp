use crate::billing::Tracker;
use crate::pricing::calculator;
use std::sync::Arc;

pub async fn record_llm_usage(
    tracker: Arc<Tracker>,
    tenant_id: &str,
    agent_id: &str,
    model: &str,
    prompt_tokens: i64,
    completion_tokens: i64,
) {
    let cost = calculator::calculate_cost(model, prompt_tokens, completion_tokens, 0);
    if let Err(e) = tracker.record_token_usage(tenant_id, agent_id, model, prompt_tokens, completion_tokens, cost).await {
        println!("Failed to record token usage: {}", e);
    }
}
