Documenting flaky sleep-based chaos tests:
- `test_simulate_sql_sync_lag`
- `test_graceful_degradation`
- `test_ml_resilience_60s_timeout_rule`
These rely on hardcoded `sleep` rather than mock time or conditional variables, making them occasionally flake on slower CI executors. This is filed under the backlog.
