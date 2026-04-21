1. **Understand the problem**:
   - The issue asks to "Implement health-check probes for hybrid-mode switching".
   - Telemetry for local-to-cloud mission sync must be added.
   - Probes must correctly categorize "Cloud Inframode" vs "Local Workmode" health.

2. **Analysis of existing code**:
   - `srcs/server/dashboard/server.go` contains `handleHybridHealthCheck` which checks `isStandalone`.
   - Wait, `mode` is currently "standalone" or "cloud" or "local".
   - Wait, the issue says: "Ensure probes correctly categorize 'Cloud Inframode' vs 'Local Workmode' health."
   - So we should probably update the `mode` return values.
   - For standalone (i.e. `isStandalone == true`), the mode should be "Local Workmode".
   - For cloud (i.e. `!isStandalone`), the mode should be "Cloud Inframode".

   - Let's check `handleHybridHealthCheck` again:
     ```go
		mode := "Local Workmode"
		isStandalone := true
		if os.Getenv("DATABASE_URL") != "" {
			mode = "Cloud Inframode"
			isStandalone = false
		}
		if os.Getenv("OHC_STANDALONE") == "true" {
			isStandalone = true
			mode = "Local Workmode"
		}
     ```

   - Additionally, the issue mentions: "Add telemetry for local-to-cloud mission sync."
     - Let's look at `telemetry.go`. I should add a telemetry call in `handleHybridHealthCheck` to record that the probe was accessed, or maybe it means telemetry for "local-to-cloud mission sync" in general?
     - Actually, "health-check probes ... specifically for hybrid-mode switching and local-to-cloud mission sync".
     - We already have `RecordSyncEscalation`, `RecordSyncLatency`, `RecordSyncPayloadSize` etc.
     - Is there a telemetry call for `HybridHealthCheck`?
     - Let's create `RecordHybridHealthCheck(ctx context.Context, mode string, status string)` in `telemetry.go` and call it from `handleHybridHealthCheck`.

   - Wait, `telemetry.go` has `AgentExecutionTracesTotal` etc.
     - Let's create `HybridHealthCheckTotal` in `telemetry.go`.
     - In `Init()`:
       ```go
       HybridHealthCheckTotal, _ = meter.Int64Counter("ohc_hybrid_health_check_total",
           metric.WithDescription("Total number of hybrid health checks"),
       )
       ```
     - Wait, let's just create a simple wrapper or look at how other telemetry works.

3. **Plan**:
   - Edit `srcs/server/dashboard/server.go`: Update `mode` string to "Cloud Inframode" and "Local Workmode" in `handleHybridHealthCheck`.
   - Also emit telemetry for this probe (e.g. `telemetry.RecordHybridHealthCheck(ctx, mode, status)`).
   - Edit `srcs/server/telemetry/telemetry.go`: Add `RecordHybridHealthCheck` and `HybridHealthCheckTotal`.
   - Update tests `TestHandleHybridHealthCheck_Standalone` and `TestHandleHybridHealthCheck_Cloud` in `srcs/server/dashboard/server_onboarding_test.go` to assert "Local Workmode" and "Cloud Inframode".
   - Wait, are there other places? Let's check.
