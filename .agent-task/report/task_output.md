issue_title: "[ML Resilience] Verify DB retry attempts is set to 3"
issue_description: |
  # Background
  The ML-Resilience 60-Second Timeout Rule specifies that AI agent jobs must have a maximum of 3 automatic retries instead of the previous limit of 10. This is to ensure mode parity across Cloud and Standalone environments and strictly enforce the timeout constraints.

  # Findings
  - In `src/server/db.rs`, the `execute_with_retry` method's `max_attempts` has already been set to `3`.
  - In `src/server/orchestration/state/parity_test.rs`, assertions are already checking for 3 attempts (e.g., in `test_execute_with_retry_chaos_exhaustion`).

  Therefore, the codebase is already compliant with the ML-Resilience rule and no code changes are necessary for this specific issue.

  # Trade-offs
  None. This is an explicit requirement of the ML-Resilience specifications to ensure that hanging jobs or locked databases fail fast instead of compounding system delays.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
