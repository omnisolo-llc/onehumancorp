# Planned implementation and verification — 2026-09-08

## Status

Implementation and acceptance work is ongoing on `fix/harness-plan-completion`.
Commit `bcc0d038a` was pushed before this completion pass. The results below
replace the earlier review's obsolete implementation-gap descriptions. A live
matrix is not complete until its native service-operation gate and all twelve
rows pass.

## Implemented plan requirements

| Requirement | Implementation and verification |
| --- | --- |
| Immutable per-attempt provider routing | Each attempt owns its facade, native child and service listener. Request selection, output limits, correlation, response redaction, cancellation, deletion and disconnect are enforced. Provider and gRPC lifecycle tests exercise revocation and concurrent attempts. |
| Issued local-service authority | Registry issuance, generations, revocation and terminal fencing back typed operations. Caller-supplied portable references do not grant authority. |
| Trusted namespaces | Worker bootstrap admits sessions from a service-owned configuration. Control HTTP and worker gRPC require a control credential; native children receive a different, scoped credential. Linux workers protect parent credentials from same-UID child process inspection. |
| Existing backend selection | Gateways wrap selected SQLite, JSON, Anthropic, Redis and vector memory implementations, existing blob storage, selected tools and the Screenshot/Playwright executor. No fallback database replaces the selected backend. Configuration digests reflect that selection. |
| Portable handoff | Capsule references retain names and digests; trusted admission issues fresh attempt bindings and fences old authority. Resumed configuration, forgery and capsule-retention regressions pass. |
| Native OmniSolo service execution | A bounded Responses tool loop executes operations through the issued service route, preserves tool results between model calls and accounts for every response's usage. A provider fixture verifies an actual SQLite write before final output. |
| Four real CLI shims | Aider 0.86.0, Goose 1.33.1, Open Interpreter 0.4.2 and Plandex 2.2.1 invoke their pinned native commands. CLI-originated provider traffic, usage, failure, cancellation and cleanup replace synthetic shim responses. All four installed CLI probes passed against a deterministic provider fixture. |
| Service deployment | Compose and Helm offer a separate service container with daemon-only storage/configuration mounts. Plandex has its own pinned native server and service-owned database. Additional backend credentials remain daemon-only. |
| Native-first live acceptance | Eight native harnesses precede four shim rows. Native writers and fresh readers use distinct withheld values for memory, artifacts, workspace and cache. Cross-harness reads and service-side receipts establish actual tool, integration and browser operations. Metadata-only rows fail validation. |
| Parallel agent execution | Authenticated orchestration dispatches to registered agents and streams correlated real progress/results. Tenant boundaries, disconnect cancellation, timeouts and coordinator cleanup are enforced. Simulated hierarchical execution was removed. |
| Agent metrics and SSE | Metrics report observed executions, failures, costs and available memory samples. The UI displays unavailable measurements honestly. SSE wakes on messages, checks tenant ownership, expires sessions and streams incrementally through the authenticated Next transport. |

## Verified evidence

| Check | Result |
| --- | --- |
| UI unit/component suite | 1,422 passed; 0 failed or skipped |
| Production Next build | Passed |
| TypeScript check | Passed |
| Agent API tests | 24 passed |
| Authentication tests | 97 passed |
| Worker library tests | 27 passed, including bootstrap retry, rejection, actual scoped storage and revocation |
| Focused resumed-session/backend/capsule tests | 39 passed; selected backend adapter rerun: 8 passed |
| Live runner and service fixture Python tests | 12 passed |
| Shim, Kimi bridge and Plandex entrypoint Python tests | 17 passed |
| Deployment contract | Passed |
| Compose/Helm service isolation rendering | 2 passed |
| Built service image smoke | Actual SQLite write, selected Read tool execution, integration metadata read and Chromium screenshot/snapshot passed |

The full deterministic run passed 460 tests (three explicitly live tests ignored).
The final timeout and remote-HTTP regressions are being rerun. Strict Clippy
passes for the harness, worker and builtin agent libraries. Bazel passed its
harness test target. Migration parity passed both tests. LLVM coverage measured
91.408957% lines, 90.824504% regions and 86.620630% functions before the final
regressions; this is not branch coverage. The provider-backed matrix remains
pending: the first native probe exposed the native worker ignoring the configured
request timeout, which now has a passing regression test. Earlier concurrent
verification runs exposed a request-shape regression and process-startup timeout
sensitivity; the fixes are included and affected suites are being rerun with
bounded concurrency. No interrupted, skipped or fixture-only run is counted as
live native acceptance.
