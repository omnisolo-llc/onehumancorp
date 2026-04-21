1. **Fix duplicate functions in `srcs/server/orchestration/sip.go`**
   - The file contains both `PruneBufferedMetrics` and `PruneTelemetryBuffer` which do exactly the same thing. I will use a Python script via `run_in_bash_session` to remove `PruneTelemetryBuffer` from `srcs/server/orchestration/sip.go` and replace its usage in `srcs/server/main.go` and tests with `PruneBufferedMetrics`.
   - Dedicated verification step: I will run `git diff srcs/server/orchestration/sip.go srcs/server/main.go` to confirm the removal and replacement are correct.

2. **Fix health check stuck mission count**
   - The method `CheckHealth` in `srcs/server/orchestration/health.go` currently considers `FAILED` missions as `STUCK`. It does `SELECT COUNT(*) FROM agent_missions WHERE status = 'STUCK' OR status = 'FAILED'`. `FAILED` missions are expected terminal states and shouldn't continuously degrade the dashboard's health status.
   - I will use a Python script via `run_in_bash_session` to change the query in `srcs/server/orchestration/health.go` to only count `'STUCK'` missions (e.g. `SELECT COUNT(*) FROM agent_missions WHERE status = 'STUCK'`).
   - Dedicated verification step: I will run `git diff srcs/server/orchestration/health.go` to verify the query change.

3. **Fix stagnant mission sanitization**
   - In `srcs/server/orchestration/sip.go`, `PruneStaleMissions` updates `status = 'FAILED'` but it leaves them. However, since we fixed the health check to ignore `FAILED` this fixes the noisy health degradation. But wait, `PruneStaleMissions` is only called with `7*24*time.Hour` threshold in `main.go`, meaning a mission might be stuck for 7 days before being marked as `FAILED`.
   - The instructions state: "Sanitize and prioritize the `agent_missions` queue, ensuring no 'stuck' missions persist in either mode."
   - I will use a Python script via `run_in_bash_session` to update `srcs/server/main.go` to invoke a dedicated cleanup logic for STUCK missions or update the `PruneStaleMissions` threshold to actively prune STUCK ones much faster (e.g., mark PENDING/STUCK missions older than 1 hour as FAILED). Or better, I will modify `main.go`'s periodic ticker to explicitly call `PruneStaleMissions(ctx, 1*time.Hour)` for stuck/pending timeout, and another for full deletion. Actually, just adding `sipdb.SanitizeStuckMissions(ctx, 1*time.Hour)` could be a cleaner approach, or updating `PruneStaleMissions` to handle STUCK/BURSTING/PENDING missions differently from COMPLETED ones.
   - Let's update `srcs/server/orchestration/sip.go` to have `SanitizeStuckMissions(ctx context.Context, timeout time.Duration)` which updates `STUCK` and `PENDING` missions older than `timeout` to `FAILED`. Then call this in `main.go` every hour.
   - Dedicated verification step: I will run `git diff srcs/server/orchestration/sip.go srcs/server/main.go` to confirm the logic.

4. **Run Tests**
   - Execute `./bazelisk test //srcs/server/...` to ensure there are no compilation errors or failed tests.

5. **Pre-commit checks**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit PR**
   - Use the `submit` tool to create the PR.
   - Title: `🧹 Maintainer: [Hygiene] Fix stuck mission sanitization and resolve duplicate methods`
   - Description: Ensure STUCK missions are correctly swept up, health check only considers STUCK (not FAILED) as degraded, and fix duplicate `PruneTelemetryBuffer` method definitions.
