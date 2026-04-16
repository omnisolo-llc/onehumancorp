import re

with open("srcs/server/main.go", "r") as f:
    content = f.read()

content = content.replace(
    "if err := sipdb.PruneStaleMissions(ctx, 7*24*time.Hour); err != nil {",
    "if err := sipdb.PruneStaleMissions(ctx, 7*24*time.Hour); err != nil {\n						slog.Error(\"failed to prune stale missions\", \"error\", err)\n					}\n					// Hygiene: Prune old telemetry buffer entries to prevent unbounded local growth\n					if err := sipdb.PruneTelemetryBuffer(ctx, 24*time.Hour); err != nil {\n"
)

with open("srcs/server/main.go", "w") as f:
    f.write(content)
