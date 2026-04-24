1. **Analyze PII Leak Risk issue**
   - The test `TestBufferMetricFuncRedactionLinter` in `src/server/telemetry/buffer_pii_linter_test.go` checks that any function calling `BufferMetricFunc` with `json.Marshal` also calls `RedactInterfacePII` or `RedactPII`.
   - The test currently has a specific exclusion allowing `RecordLocalToCloudMissionSync` to fail because it wasn't correctly implemented: `if fn.Name.Name == "RecordLocalToCloudMissionSync"`.
   - We need to refactor `RecordLocalToCloudMissionSync` in `src/server/telemetry/telemetry.go` to correctly call `RedactInterfacePII` so it passes the linter, and remove the exemption in `buffer_pii_linter_test.go`.

2. **Modify `telemetry.go`**
   - Update `RecordLocalToCloudMissionSync` in `src/server/telemetry/telemetry.go` to explicitly call `RedactInterfacePII` *before* or *during* `json.Marshal`, e.g., `payloadBytes, _ := json.Marshal(RedactInterfacePII(payloadMap))`.
   - The function already attempts `payloadMap["missionID"] = RedactInterfacePII(missionID)`, but the linter looks for `RedactInterfacePII` applied to the map that gets passed to `json.Marshal` or explicitly called as an argument to `json.Marshal`. We will update it to match the standard pattern: `payloadBytes, _ := json.Marshal(RedactInterfacePII(payloadMap))`.

3. **Modify `buffer_pii_linter_test.go`**
   - Remove the hardcoded exclusion `if fn.Name.Name == "RecordLocalToCloudMissionSync" { t.Errorf(...) }`. We want the linter to uniformly enforce the rule for all functions, including this one.
   - We'll change the linter to simply `t.Errorf(...)` for *any* function that violates the rule.

4. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
   - Call `pre_commit_instructions`.
   - Ensure all `telemetry` tests pass, including the updated linter test.
