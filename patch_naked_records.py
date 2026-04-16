import re

with open("srcs/server/telemetry/telemetry_patched.go", "r") as f:
    content = f.read()

# Replace naked .Add(ctx, X) with .Add(ctx, X, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))
content = re.sub(r'(\.Add\(ctx,\s*[^,)]+)\)', r'\1, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))', content)

# Replace naked .Record(ctx, X) with .Record(ctx, X, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))
content = re.sub(r'(\.Record\(ctx,\s*[^,)]+)\)', r'\1, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))', content)

# There is also one edge case: .Record(ctx, duration, attributes) in Middleware, which got caught by the WithAttributes regex earlier.
# Wait, attributes is defined explicitly inside Middleware.
# The regex `metric.WithAttributes\(` already modified `attributes := metric.WithAttributes(`

with open("srcs/server/telemetry/telemetry_patched.go", "w") as f:
    f.write(content)
