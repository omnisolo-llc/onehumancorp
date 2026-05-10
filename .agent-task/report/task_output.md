The requested Chaos Engineering experiments, Degradation Validation tests, and ML-Resilience rules (60-second timeouts, retry loops, etc.) are already fully implemented in the codebase.

Evidence:
1. Chaos tests `test_sql_sync_lag_simulation`, `test_exhaust_cpu_memory_and_verify_graceful_degradation`, `test_transport_packet_loss_simulation`, `test_sentry_chaos_network_partition` exist in `src/server/chaos.rs`.
2. Chaos tests `test_agent_lock_race_conditions`, `test_pubsub_message_loss`, and `test_cloud_degradation_fallback` exist in `src/server/orchestration/chaos_test.rs`.
3. ML-Resilience timeouts are verified in `test_ml_resilience_60s_timeout_rule` and `test_ml_resilience_tasks_timeout`.

The requested features are already implemented. We performed the database schema parity tasks for `agent_violations` and `agent_memories`.
