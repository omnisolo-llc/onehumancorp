1. **Initialize `HybridSyncDaemon` in `main.go`:**
   - I will use `run_in_bash_session` to write a Python script that injects `hybridSyncDaemon := hybrid_sync.NewHybridSyncDaemon(pool, 5*time.Second, os.Getenv("OHC_CLOUD_CONTEXT_ENDPOINT"))` and `hybridSyncDaemon.Start(ctx)` into `srcs/server/main.go` under the "Background sync for standalone missions to cloud" block.
   - The python script will also ensure the `github.com/onehumancorp/mono/srcs/server/orchestration/hybrid_sync` package is imported in `main.go`.
2. **Ensure `main.go` builds successfully:**
   - I will run `bazelisk build //srcs/server/...` to verify the modified `main.go` has no compile errors.
3. **Verify the task's requirements:**
   - The service is created, respects `FOR UPDATE SKIP LOCKED`, degrades gracefully (logs errors on cloud connection issues instead of crashing, keeps the data locally).
   - Mock tests exist and use `db.NewSqliteProvider`.
   - The queue relies on `queue.NewPostgresTaskQueue(hub.SIPDB().Provider())` which maps to SubAgentQueue.
4. **Complete pre commit steps:**
   - Run `pre_commit_instructions` tool to perform required tests, verifications, review, and reflection.
   - Run `bazelisk test //... --test_timeout=120` to guarantee all tests pass.
5. **Submit the change:**
   - Submit the branch with a descriptive commit message.
