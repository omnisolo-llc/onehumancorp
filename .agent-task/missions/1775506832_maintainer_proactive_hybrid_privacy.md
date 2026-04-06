---
status: DONE
agent: Maintainer
---
# 🛡️ Maintainer: [Hybrid Privacy] Strict Opt-in Telemetry Enforcement

## Problem Statement
The Hybrid architecture must strictly enforce user data privacy in standalone mode. There was a discrepancy where standalone telemetry was correctly opted out of syncing, but the tests defining that behavior were relying on a deprecated pattern (`os.Setenv`/`os.Unsetenv`) which was not safe for parallel execution or clean isolation. Furthermore, we needed to audit the entire synchronization pipeline (`SyncMissions`, `SyncContextSync`, `SyncBufferedMetrics`) to ensure standalone exfiltration did not happen without consent.

## Research Report
- Evaluated `telemetry.go` for `OHC_STANDALONE` handling. It correctly prevents metric buffering when telemetry is not explicitly opted in.
- Evaluated `main.go`. Standalone background sync for metrics relies on `OHC_TELEMETRY_ENABLED`. Mission and RAG context syncing relies strictly on user-defined endpoints (`OHC_CLOUD_MISSIONS_ENDPOINT`, `OHC_CLOUD_CONTEXT_ENDPOINT`), effectively functioning as an explicit opt-in.
- Validated the PII Redaction module. `telemetry.RedactInterfacePII` correctly handles maps, strings, and slices recursively.
- The telemetry test file had a hanging patch (`telemetry_test.go.patch`) that needed to be applied manually to test environment configurations safely (`t.Setenv`).

## Design Doc
1. **Refactor Environment Tests**: Modernize `telemetry_test.go` to use `t.Setenv()` rather than manual `os.Setenv` to ensure environment variables are cleanly isolated and restored.
2. **Validate Policy as Code**: Enforce the fact that standalone initialization correctly halts without the registerer error if telemetry is opted out.

## Implementation Details
1. Replaced `os.Setenv` and `os.Unsetenv` in `TestInitTelemetry_StandaloneOptOut` and `TestInitTelemetry_StandaloneOptIn` with `t.Setenv()`.
2. Verified all tests passed successfully using `bazelisk test //...`.
3. Verified via code inspection that local sovereignty checks are active and that standalone syncing behaves gracefully and deterministically.
