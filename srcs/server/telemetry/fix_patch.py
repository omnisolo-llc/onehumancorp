with open("srcs/server/telemetry/telemetry.go", "r") as f:
    content = f.read()

content = content.replace("RagRecordsSyncedTotal metric.Int64Counter", "RagRecordsSyncedTotal metric.Int64Counter\n\tRagSyncErrorsTotal    metric.Int64Counter")
content = content.replace('RagSyncErrorsTotal, err = m.Int64Counter(\n\t\t"ohc_rag_sync_errors_total",\n\t\tmetric.WithDescription("Total RAG sync errors"),\n\t)\n\tif err != nil {\n\t\terrs = append(errs, err)\n\t}', '')

with open("srcs/server/telemetry/telemetry.go", "w") as f:
    f.write(content)
