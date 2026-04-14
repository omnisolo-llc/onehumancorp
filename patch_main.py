with open("srcs/server/main.go", "r") as f:
    content = f.read()

content = content.replace(
"""			cloudEndpoint := os.Getenv("OHC_CLOUD_TELEMETRY_ENDPOINT")
			if cloudEndpoint != "" && envBoolDefault("OHC_TELEMETRY_ENABLED", false) {
				telemetry.StartSyncDaemon(ctx, sipdb.SyncBufferedMetrics, cloudEndpoint, 5*time.Minute)
			mcpSyncWorker := telemetry.NewMcpSyncWorker(dbProvider)
			go mcpSyncWorker.Start(ctx)
			}""",
"""			cloudEndpoint := os.Getenv("OHC_CLOUD_TELEMETRY_ENDPOINT")
			if cloudEndpoint != "" && envBoolDefault("OHC_TELEMETRY_ENABLED", false) {
				telemetry.StartSyncDaemon(ctx, sipdb.SyncBufferedMetrics, cloudEndpoint, 5*time.Minute)
				mcpSyncWorker := telemetry.NewMcpSyncWorker(pool) // Use pool as dbProvider
				go mcpSyncWorker.Start(ctx)
			}""")

with open("srcs/server/main.go", "w") as f:
    f.write(content)
