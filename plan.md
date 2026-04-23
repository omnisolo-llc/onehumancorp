1. **Create Database Migration**:
   - Create `srcs/server/db/migrations/20260429000000_sub_agent_queue_backoff.sql` (both Postgres and SQLite compatible if possible, or use `.go` migration, wait, `goose` supports basic SQL `ALTER TABLE`).
   - Add columns: `attempts INTEGER DEFAULT 0`, `max_attempts INTEGER DEFAULT 3`, `run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP` to `sub_agent_queue`.

2. **Update `SubAgentJob` and `QueueManager.Enqueue`**:
   - Update `SubAgentJob` struct in `srcs/server/orchestration/queue/queue_manager.go` with `Attempts`, `MaxAttempts`, `RunAfter`.
   - In `Enqueue`, initialize `RunAfter = time.Now()` if zero, and `MaxAttempts = 3` if zero. Modify the `INSERT` query to include these fields.

3. **Update `QueueManager.Poll` (Quota and Scheduling)**:
   - Modify the `Poll` method to only select jobs where `run_after <= NOW()`.
   - Add a condition to ensure the organization hasn't exceeded the VRAM/Token quota (simulated as max 10 `RUNNING` jobs).
   - In both PostgreSQL (`FOR UPDATE SKIP LOCKED`) and SQLite lock methods, the subquery must include `(SELECT COUNT(*) FROM sub_agent_queue r WHERE r.organization_id = sub_agent_queue.organization_id AND r.status = 'RUNNING') < 10`.

4. **Update `QueueManager.MarkFailed` (Exponential Backoff)**:
   - Query the current `attempts` and `max_attempts`.
   - If `attempts < max_attempts`, increment `attempts`, calculate `run_after = NOW() + (2^attempts * 1s)`, and `UPDATE` status back to `QUEUED`.
   - If `attempts >= max_attempts`, `UPDATE` status to `FAILED`.

5. **Write Unit Tests**:
   - In `srcs/server/orchestration/queue/queue_manager_loop_test.go`, add `TestQueueManager_QuotaEnforcement` to verify a 11th job is NOT dequeued if 10 are running.
   - Add `TestQueueManager_ExponentialBackoff` to verify `MarkFailed` correctly retries up to `max_attempts`.

6. **Pre-commit and Submit**:
   - Run `pre_commit_instructions` and execute checks (`bazelisk test //...`).
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
   - Run `submit` with issue ID 5049.
