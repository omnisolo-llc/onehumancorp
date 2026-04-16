1. **Modify `srcs/server/telemetry/telemetry.go` to include `EnvMode` tag in all metric recordings:**
   - Use python script `patch_telemetry_5.py` to inject `attribute.String("EnvMode", getEnvMode())` into all metric `.Add` and `.Record` calls, and add `getEnvMode()` helper.
   - Run `bazelisk test //srcs/server/telemetry/...` to verify the module builds and passes. (Already done manually but listing it for tracking).
2. **Create/Verify `srcs/server/telemetry/sync_worker.go`**:
   - The file already exists and looks correct. It syncs the standalone metrics to the cloud API.
3. **Add `local_telemetry_buffer` to SQLite schema**:
   - Run `cat << 'EOF' > srcs/server/db/schema.sql ... EOF` to create `srcs/server/db/schema.sql` containing the `local_telemetry_buffer` table.
   - Run `cat srcs/server/db/schema.sql` to verify creation.
4. **Grafana Dashboard**:
   - Run `mkdir -p monitoring/grafana/dashboards` and `cat << 'EOF' > monitoring/grafana/dashboards/harness_efficiency.json ... EOF` with the Grafana JSON.
   - Run `cat monitoring/grafana/dashboards/harness_efficiency.json` to verify creation.
5. **Run Tests**:
   - Run `bazelisk test //srcs/server/telemetry/... //srcs/server/db/...` to ensure no regressions were introduced.
6. **Pre Commit**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Submit PR**:
   - Explicitly run `git checkout -b fix-harness-efficiency-telemetry`.
   - Run `git add .` and `git commit -m "🧹 Maintainer: [backend] Architect Cloud vs Standalone Efficiency Telemetry for Agent Harness"`.
   - Run `curl -X POST https://api.github.com/repos/onehumancorp/mono/pulls -H "Authorization: Bearer $GITHUB_TOKEN" -d '{"title": "🧹 Maintainer: [backend] Architect Cloud vs Standalone Efficiency Telemetry for Agent Harness", "head": "fix-harness-efficiency-telemetry", "base": "main", "body": "Resolves #5443"}'` to submit the PR.
