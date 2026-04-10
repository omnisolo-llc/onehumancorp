Plan:
1. Update `srcs/server/telemetry/telemetry.go` to handle `map[string]string` in `RedactInterfacePII`.
2. Update `srcs/server/api/sync_escalation_handler.go` to redact PII in `p.Context`.
3. Update `srcs/server/api/sync_handler.go` to fallback to `telemetry.RedactPII` for non-JSON payloads.
4. Run tests and verify the changes locally.
5. Create a `pre_commit_instructions` and submit!
