with open("srcs/server/telemetry/telemetry.go", "r") as f:
    content = f.read()

if "RagSyncErrorsTotal    metric.Int64Counter" not in content:
    content = content.replace("RagRecordsSyncedTotal metric.Int64Counter", "RagRecordsSyncedTotal metric.Int64Counter\n\tRagSyncErrorsTotal    metric.Int64Counter")

with open("srcs/server/telemetry/telemetry.go", "w") as f:
    f.write(content)
