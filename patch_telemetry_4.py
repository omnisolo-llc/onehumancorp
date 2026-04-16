import re

with open("srcs/server/telemetry/telemetry_patched.go", "r") as f:
    content = f.read()

# Fix the two int64 casting bugs
content = content.replace("swarmTaskQueueLengthGauge.Add(ctx, int64(delta, metric.WithAttributes(attribute.String(\"EnvMode\", getEnvMode()))))",
                          "swarmTaskQueueLengthGauge.Add(ctx, int64(delta), metric.WithAttributes(attribute.String(\"EnvMode\", getEnvMode())))")

content = content.replace("subAgentQueueLengthGauge.Add(ctx, int64(delta, metric.WithAttributes(attribute.String(\"EnvMode\", getEnvMode()))))",
                          "subAgentQueueLengthGauge.Add(ctx, int64(delta), metric.WithAttributes(attribute.String(\"EnvMode\", getEnvMode())))")

# Also there's 3 undefined initialization errors, but they are defined in other files of the same package:
# telemetry_bridge.go, rag_sync_metrics.go, minimax_metrics.go.
# Our test `go build -o /dev/null srcs/server/telemetry/telemetry_patched.go` failed because it only builds that one file, not the whole package.

with open("srcs/server/telemetry/telemetry_patched.go", "w") as f:
    f.write(content)
