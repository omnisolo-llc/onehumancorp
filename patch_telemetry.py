import re

with open("srcs/server/telemetry/telemetry.go", "r") as f:
    content = f.read()

# Add getEnvMode()
env_mode_func = """
// getEnvMode returns the current environment mode (cloud vs standalone).
func getEnvMode() string {
	mode := os.Getenv("OHC_ENV_MODE")
	if mode == "" {
		return "standalone"
	}
	return mode
}
"""

if "func getEnvMode()" not in content:
    content = content.replace('func envBoolDefault(', env_mode_func + '\nfunc envBoolDefault(')

# 1. Update payloadMap := map[string]interface{}{ to include EnvMode
content = re.sub(
    r'(payloadMap := map\[string\]interface{}\{)',
    r'\1\n\t\t\t"EnvMode": getEnvMode(),',
    content
)

# 2. Add EnvMode to existing metric.WithAttributes calls
content = re.sub(
    r'metric\.WithAttributes\(',
    r'metric.WithAttributes(\n\t\tattribute.String("EnvMode", getEnvMode()),',
    content
)

# 3. Add WithAttributes to naked .Add() calls
content = re.sub(
    r'(\.Add\(ctx,\s*[^,)]+)\)',
    r'\1, metric.WithAttributes(attribute.String("EnvMode", getEnvMode())))',
    content
)

# 4. Add WithAttributes to naked .Record() calls. Need to be careful about calls like .Record(ctx, latency.Seconds())
# We can just match `.Record(ctx, <balanced expression>)` but python regex doesn't support recursive easily.
# Let's do it manually for the 9 cases we found earlier.

with open("srcs/server/telemetry/telemetry_patched.go", "w") as f:
    f.write(content)
