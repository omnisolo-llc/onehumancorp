Severity: Low
Vulnerability: Contextual telemetry for sandbox boundary failures was missing.
Impact: Execution tracking via OpenTelemetry could not distinguish sandbox restriction drops ("Operation not permitted" or "Permission denied") from general execution failures.
Fix: Intercepted "Operation not permitted" and "Permission denied" errors in the bash sandbox (`srcs/server/bash_sandbox/sandbox.go`) and appended explicit `<sandbox_violations>...: sandbox boundary drop</sandbox_violations>` telemetry to stderr drops. Additionally, fixed multiple flaky Go test files utilizing shared in-memory SQLite caches to use unique test database instances.
