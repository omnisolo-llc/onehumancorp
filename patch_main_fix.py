import re

with open("srcs/server/main.go", "r") as f:
    content = f.read()

content = content.replace(
    """if err := sipdb.PruneTelemetryBuffer(ctx, 24*time.Hour); err != nil {

						slog.Error("failed to prune stale agent missions", "error", err)
					} else {
						slog.Debug("successfully pruned stale agent missions")
					}""",
    """if err := sipdb.PruneTelemetryBuffer(ctx, 24*time.Hour); err != nil {
						slog.Error("failed to prune stale telemetry buffer", "error", err)
					} else {
						slog.Debug("successfully pruned stale agent missions and telemetry buffer")
					}"""
)

with open("srcs/server/main.go", "w") as f:
    f.write(content)
