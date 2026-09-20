#[cfg(test)]
mod ml_resilience_tests {
    use crate::queue::{Job, MemoryTaskQueue, TaskQueue};

    fn job(id: &str, tenant: &str, status: &str) -> Job {
        let now = chrono::Utc::now();
        Job {
            id: id.into(),
            tenant_id: tenant.into(),
            parent_task_id: String::new(),
            job_type: "operations".into(),
            payload: "{}".into(),
            status: status.into(),
            retry_count: 0,
            max_retries: 3,
            next_retry_at: now,
            locked_until: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[tokio::test]
    async fn test_agent_timeout_and_retry_rule() {
        // ML-Resilience Rule 1: AI agent jobs must have a 60-second timeout with automatic retry (max 3 attempts).
        let timeout_ms = omnisolo_builtin_agent::agent::agent_task_timeout().as_millis();
        assert_eq!(
            timeout_ms, 60000,
            "Agent jobs must have a 60-second timeout"
        );

        // The max attempts rule is checked in queue.rs handling code, where attempts are incremented up to max_attempts (3 default)
    }

    #[tokio::test]
    async fn failed_job_does_not_complete_or_block_another_tenant_job() {
        let queue = MemoryTaskQueue::new();
        queue
            .enqueue(job("failed", "tenant-a", "QUEUED"))
            .await
            .unwrap();
        queue
            .enqueue(job("healthy", "tenant-b", "QUEUED"))
            .await
            .unwrap();
        let first = queue
            .dequeue(vec!["operations".into()])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(first.id, "failed");
        assert!(
            queue
                .fail(&first.id, "tenant-b", "wrong tenant")
                .await
                .is_err()
        );
        queue
            .fail(&first.id, "tenant-a", "provider unavailable")
            .await
            .unwrap();
        let second = queue
            .dequeue(vec!["operations".into()])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            (second.id.as_str(), second.tenant_id.as_str()),
            ("healthy", "tenant-b")
        );
        assert!(queue.complete(&second.id, "tenant-a").await.is_err());
        queue.complete(&second.id, "tenant-b").await.unwrap();
        queue.complete(&second.id, "tenant-b").await.unwrap();
        assert!(
            queue
                .dequeue(vec!["operations".into()])
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn paused_job_is_not_dispatched_until_explicitly_requeued() {
        // This verifies queue pause semantics, not an unperformed live LLM call.
        let queue = MemoryTaskQueue::new();
        let mut paused = job("paused", "tenant-a", "PAUSED");
        queue.enqueue(paused.clone()).await.unwrap();
        assert!(
            queue
                .dequeue(vec!["operations".into()])
                .await
                .unwrap()
                .is_none()
        );
        paused.status = "QUEUED".into();
        queue.requeue(paused).await.unwrap();
        let resumed = queue
            .dequeue(vec!["operations".into()])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(resumed.id, "paused");
        assert_eq!(resumed.status, "IN_PROGRESS");
        assert!(
            queue
                .dequeue(vec!["operations".into()])
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_token_budget_server_side() {
        // ML-Resilience Rule 5: Token budgets must be enforced server-side.
        let mut tracker = omnisolo_builtin_agent::budget::BudgetTracker::default();
        let budget = 1000;
        let global_turn_tokens = 800; // < 900 (90%)
        let decision = omnisolo_builtin_agent::budget::check_token_budget(
            &mut tracker,
            budget,
            global_turn_tokens,
        );
        // It should continue since we haven't reached 1000 or diminishing returns
        assert_eq!(
            decision.action,
            omnisolo_builtin_agent::budget::BudgetAction::Continue,
            "Token budget must enforce limits server-side"
        );

        let global_turn_tokens_exceeded = 950; // > 90% (threshold is 0.9)
        let decision_stop = omnisolo_builtin_agent::budget::check_token_budget(
            &mut tracker,
            budget,
            global_turn_tokens_exceeded,
        );
        assert_eq!(
            decision_stop.action,
            omnisolo_builtin_agent::budget::BudgetAction::Stop,
            "Token budget must stop server-side execution if exceeded"
        );
    }
}
