import re

with open("srcs/server/telemetry/telemetry_patched.go", "r") as f:
    content = f.read()

# Let's fix naked .Record() calls
# The known ones are:
replacements = [
    ("latencyHistogram.Record(r.Context(), duration, attributes)", "latencyHistogram.Record(r.Context(), duration, attributes)"),
    ("TaskProcessingLatency.Record(ctx, latency.Seconds())", 'TaskProcessingLatency.Record(ctx, latency.Seconds(), metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("SIPSyncLatencyRecorder.Record(ctx, latency.Seconds())", 'SIPSyncLatencyRecorder.Record(ctx, latency.Seconds(), metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("SIPSyncPayloadSizeRecorder.Record(ctx, int64(bytes))", 'SIPSyncPayloadSizeRecorder.Record(ctx, int64(bytes), metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("SubAgentExecutionDuration.Record(ctx, duration)", 'SubAgentExecutionDuration.Record(ctx, duration, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("SubAgentQueueDelayHistogram.Record(ctx, delay)", 'SubAgentQueueDelayHistogram.Record(ctx, delay, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("syncLatency.Record(ctx, latency.Seconds())", 'syncLatency.Record(ctx, latency.Seconds(), metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("syncPayloadSize.Record(ctx, int64(size))", 'syncPayloadSize.Record(ctx, int64(size), metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("syncDaemonBatchSize.Record(ctx, size)", 'syncDaemonBatchSize.Record(ctx, size, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
    ("swarmTaskProcessingLatency.Record(ctx, latencyMS)", 'swarmTaskProcessingLatency.Record(ctx, latencyMS, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))'),
]

for old, new in replacements:
    content = content.replace(old, new)

with open("srcs/server/telemetry/telemetry_patched.go", "w") as f:
    f.write(content)
