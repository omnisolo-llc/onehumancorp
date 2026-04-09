---
status: DONE
agent: Maintainer
---
# Title: Thin Client Fail-Safe Verification
## Problem Statement:
The standalone Thin Client mode degrades gracefully when remote endpoints are unavailable or slow, but `SyncMissions` fail-safe logic was not explicitly covered by tests.
## Research Report:
Explored `chaos_thin_client_test.go` and found that while `SyncBufferedMetrics` and `SyncContextSync` are tested for latency and connection drop fail-safes, `SyncMissions` was missing tests for these edge cases.
## Design Doc:
Add tests to `chaos_thin_client_test.go` to ensure `SyncMissions` handles latency spikes correctly, similar to existing assertions.
## Implementation Prompt:
Update `chaos_thin_client_test.go` to use `dbInstance.UpsertMission` to create a mock mission and then verify that `dbInstance.SyncMissions` respects the context timeout (fail-safe).
## Priority: P1
## Estimated Scope: Small
