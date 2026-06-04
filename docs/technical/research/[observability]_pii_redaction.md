# [observability] Fix PII redaction leaks in telemetry and event logging

## Title
Fix PII redaction leaks before JSON marshaling in telemetry and event logging.

## Problem Statement
The OHC codebase enforces strict PII redaction across the multi-tenant architecture. Specifically, "In telemetry or logging code, always apply `RedactInterfacePII` (or an equivalent redaction function) to payload maps before calling `json.Marshal` to prevent PII leakage in multi-tenant environments."
However, there are multiple code paths violating this rule:
1. `src/server/telemetry/telemetry.go`: The `RecordQueueLength` function builds a payload string manually using `fmt.Sprintf` instead of safely constructing a map, redacting it, and JSON marshaling it.
2. `src/server/orchestration/event_log.go`: The `sanitizeHubEvent` function takes a `raw interface{}`, immediately serializes it via `json.Marshal(raw)`, and only then attempts to unmarshal and redact. If the initial parsing fails, the unredacted payload leaks into the event log and persistent storage.

## Research Report
A deep code audit verified all `BufferMetricFunc` calls and `json.Marshal` usages.
- In `telemetry.go`, an invariant test (`pii_linter_test.go` and `ast_pii_linter_test.go`) enforces the redaction mechanism. However, `RecordQueueLength` bypasses it because it skips JSON marshaling entirely, breaking the structural consistency.
- In `event_log.go`, `sanitizeHubEvent` calls `json.Marshal` prematurely before the redaction step, representing a severe risk for leaking tenant data into orchestrator logs.

## Design Doc
1. **telemetry.go**: Update `RecordQueueLength` to construct a map with the key `"delta"`, apply `RedactInterfacePII(payloadMap)`, and serialize it with `json.Marshal(redactedMap)` to align with all other telemetry metrics.
2. **event_log.go**: Update `sanitizeHubEvent` to redact the `raw` input BEFORE performing the initial `json.Marshal`.

## Implementation Prompt
Hello Implementer agent! Your task is to resolve PII leakage risks in the telemetry and logging systems.

1. In `src/server/telemetry/telemetry.go`, locate `func RecordQueueLength(ctx context.Context, delta int)`. Update the implementation to follow the pattern used by `RecordSwarmTaskQueueLength`:
```go
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"delta": delta,
		}
		redactedMap := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedMap)
		_ = BufferMetricFunc(ctx, "ohc_sub_agent_queue_length", string(payloadBytes))
		return
	}
```

2. In `src/server/orchestration/event_log.go`, locate `func sanitizeHubEvent(raw interface{}) (HubEvent, error)`. Ensure the `raw` payload is redacted BEFORE it is ever converted to JSON. Modify the logic to immediately redact `raw`:
```go
func sanitizeHubEvent(raw interface{}) (HubEvent, error) {
	redactedRaw := telemetry.RedactInterfacePII(raw)
	payload, err := json.Marshal(redactedRaw)
	if err != nil {
		return HubEvent{}, fmt.Errorf("marshal hub event: %w", err)
	}

	// Since we've already redacted, we just parse it back if we need the map for type detection.
	// But actually, we can just detect the type on the redacted raw directly if it's already a map!

	// Ensure the logic is sound and no unredacted data is ever marshaled into `payload`.
```
*(You will need to adjust the unmarshaling type-check logic accordingly, but the core invariant is `telemetry.RedactInterfacePII` must be invoked BEFORE `json.Marshal`)*.

3. Run `bazelisk test //src/server/telemetry/...` and `bazelisk test //src/server/orchestration/...` to verify your changes pass the strict AST PII Linters.

## Priority
P0

## Estimated Scope
Small
