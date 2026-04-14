with open("srcs/server/main.go", "r") as f:
    content = f.read()

content = content.replace(
"""				mcpSyncWorker := telemetry.NewMcpSyncWorker(pool) // Use pool as dbProvider
				go mcpSyncWorker.Start(ctx)""",
"""				mcpSyncWorker := telemetry.NewMcpSyncWorker(pool, cloudEndpoint) // Use pool as dbProvider
				go mcpSyncWorker.Start(ctx)""")

with open("srcs/server/main.go", "w") as f:
    f.write(content)
