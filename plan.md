1. **Analyze telemetry compliance requirements**:
   - Instruction: "In telemetry or logging code, always apply `RedactInterfacePII` (or an equivalent redaction function) to payload maps before calling `json.Marshal` to prevent PII leakage in multi-tenant environments."
   - Issue 1: In `srcs/server/telemetry/telemetry.go`, `RecordQueueLength` directly constructs a payload map using `fmt.Sprintf` and doesn't apply `RedactInterfacePII`. Wait, currently it uses `fmt.Sprintf` instead of a JSON payload! However, to be consistent with all other `BufferMetricFunc` calls, and to comply with the PII redaction rule for payload maps before calling `json.Marshal`, I need to rewrite `RecordQueueLength` to use a map, redact it, and marshal it.
   - Issue 2: In `srcs/server/orchestration/event_log.go`, `sanitizeHubEvent` takes a `raw` object, calls `json.Marshal(raw)`, and THEN tries to unmarshal, redact, and re-marshal. If unmarshal fails, the unredacted payload is saved! The rule dictates redacting the object BEFORE the first `json.Marshal`. I will update `sanitizeHubEvent` to redact `raw` immediately using `telemetry.RedactInterfacePII(raw)`.

2. **Execute changes**:
   - `srcs/server/telemetry/telemetry.go`: Refactor `RecordQueueLength`.
   - `srcs/server/orchestration/event_log.go`: Refactor `sanitizeHubEvent`.

3. **Verify changes**:
   - Run `bazelisk test //srcs/server/telemetry/...` and `bazelisk test //srcs/server/orchestration/...` to ensure all tests pass.

4. **Complete pre-commit steps**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

5. **Submit the PR**:
   - Issue ID will be included.
