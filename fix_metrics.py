import re

with open('srcs/server/telemetry/telemetry.go', 'r') as f:
    content = f.read()

# I will add `SharedTasksCreatedCounter` metric to telemetry.go

new_vars = """
	SharedTasksCreatedCounter metric.Int64Counter
	MeshMessagesBroadcastCounter metric.Int64Counter
"""

content = content.replace("	SyncEscalationsCount metric.Int64Counter", "	SyncEscalationsCount metric.Int64Counter\n" + new_vars)

new_init = """
	SharedTasksCreatedCounter, err = m.Int64Counter("shared_tasks_created", metric.WithDescription("Number of shared tasks created"))
	if err != nil {
		slog.Warn("Failed to create shared_tasks_created metric", "error", err)
	}
	MeshMessagesBroadcastCounter, err = m.Int64Counter("mesh_messages_broadcast", metric.WithDescription("Number of mesh messages broadcasted"))
	if err != nil {
		slog.Warn("Failed to create mesh_messages_broadcast metric", "error", err)
	}
"""

content = content.replace("	SyncEscalationsCount, err = m.Int64Counter(\"sync_escalations_count\", metric.WithDescription(\"Total number of cross-cluster sync escalations\"))", "	SyncEscalationsCount, err = m.Int64Counter(\"sync_escalations_count\", metric.WithDescription(\"Total number of cross-cluster sync escalations\"))\n" + new_init)

with open('srcs/server/telemetry/telemetry.go', 'w') as f:
    f.write(content)
