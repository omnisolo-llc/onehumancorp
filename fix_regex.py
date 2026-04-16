import re

with open("srcs/server/telemetry/telemetry_patched.go", "r") as f:
    content = f.read()

# Fix the broken substitutions
content = content.replace("latency.Seconds(, metric.WithAttributes", "latency.Seconds()), metric.WithAttributes")
content = content.replace("latencyMS, metric.WithAttributes", "latencyMS), metric.WithAttributes")

with open("srcs/server/telemetry/telemetry_patched.go", "w") as f:
    f.write(content)
