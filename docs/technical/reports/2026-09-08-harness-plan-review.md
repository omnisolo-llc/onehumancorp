# Planned implementation and verification — 2026-09-08

## Status

Implementation is committed and pushed on `fix/harness-plan-completion`.
All eight native harnesses have passed individual provider-backed service
writer/fresh-reader checks. The final ordered twelve-row matrix is running; acceptance remains pending
until it passes. The final Bazel verification passed.

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

The [deterministic verification record](2026-09-08-deterministic-verification.json)
contains counts, coverage totals and hashes of the test logs.

| Check | Result |
| --- | --- |
| Harness and worker deterministic suites, latest coverage run | 469 passed across 26 suites; 0 failed; 3 opt-in live tests ignored |
| Broader Rust regression run, before final native follow-up fixes | 1,029 passed across 31 suites; 0 failed; 3 live tests ignored |
| UI unit/component suite | 1,422 passed; 0 failed or skipped |
| Production Next build and TypeScript check | Passed |
| Agent communication API | 24 passed |
| Authentication | 97 passed |
| Worker library | 28 passed |
| Selected backend adapters | 8 passed |
| Kimi/OpenHarness bridges and live runner Python tests | 22 passed |
| DeepSeek scoped subprocess environment contract | Passed |
| Deployment contract | Passed |
| Compose/Helm service isolation rendering | 2 passed |
| PostgreSQL/MySQL migration parity | 2 passed |
| Workspace formatting and strict harness/worker library Clippy | Passed |
| Final Bazel harness targets | Passed; one test target |
| Ordered native-first twelve-harness live matrix | Running |

LLVM coverage measured **91.522390% lines**, **90.932459% regions** and
**86.878216% functions** for the harness and worker packages. This is not branch
coverage. External backend and native process paths are additionally exercised
by container probes; those subprocess executions are not included in the LLVM
coverage counters. No skipped, interrupted or fixture-only run is counted as
live acceptance.

## Native integration corrections

Real pinned CLI execution exposed and verified fixes for:

- Codex's unused diagnostic queue blocking JSON-RPC dispatch after 256 messages;
  diagnostics now use a nonblocking broadcast with explicit lag reporting.
- Codex container sandbox selection and native workers ignoring the configured
  request timeout. Trusted worker configuration controls both behaviors.
- OpenCode treating intermediate tool-call completion as terminal.
- Kimi and DeepSeek emitting valid empty streaming text chunks.
- DeepSeek returning tool results through `source.callId` and stripping ambient
  credential-shaped environment names. Its subprocess extension explicitly
  passes only the issued local-service URL and token through the SDK spawn API.
- Kimi's boolean reasoning interface and missing ACP usage. The bridge pins the
  exact provider effort and returns actual accumulated provider usage through
  ACP's supported prompt response.
- OpenHands registry tool names and initially zero cumulative usage. The adapter
  refreshes the SDK's counters on every poll.
- OpenHarness ending a message before its provider's trailing usage chunk. The
  bridge drains the native stream and preserves the measured final usage.

Each correction has a passing focused regression or pinned-runtime contract
check and a successful native provider-backed writer/fresh-reader run. The live
fixture answers native tool permissions through the authenticated controller
exchange, checks attempt/session/task fences, and requests sequential service
operations using the interpreter installed in each pinned image. Live requests
have a ten-minute limit and writer/reader verification a thirty-minute limit;
production timeout defaults are unchanged.

## Acceptance receipts and reasoning

Individual native receipts preserve model binding, actual usage, service-side
operation receipts, withheld-value verification, credential checks and cleanup:
[OmniSolo](2026-09-08-native-omnisolo-acceptance.json),
[Codex](2026-09-08-native-codex-acceptance.json),
[OpenCode](2026-09-08-native-opencode-acceptance.json),
[DeepSeek](2026-09-08-native-deepseek-acceptance.json),
[Pi](2026-09-08-native-pi-acceptance.json),
[Kimi](2026-09-08-native-kimi-acceptance.json),
[OpenHands](2026-09-08-native-openhands-acceptance.json), and
[OpenHarness](2026-09-08-native-openharness-acceptance.json).
DeepSeek also passed its row in the previous ordered run. That run failed Kimi's
old five-minute limit and does not count as final matrix acceptance.

The selected upstream model is `gpt-5.6-luna` with `max` reasoning. Kimi reports
an explicit native translation to boolean `thinking`; the bridge preserves
`max` on the actual provider request. A pinned CLI wire probe confirmed that
selection and returned 14,269 input and 12 output tokens. OpenHarness also
reports its narrower native reasoning capability explicitly. Native capability
translation and upstream provider selection are distinct evidence fields.

## Completion ledger

| Written plan | Tasks and evidence |
| --- | --- |
| Universal harness services, tasks 1–3 | Pinned integration modes, issued namespaces, portable reference validation and trusted rebinding: local-service and capsule suites. |
| Universal harness services, tasks 4–5 | Per-attempt facade, provider translation and lifecycle revocation: facade, worker gRPC and worker E2E suites. |
| Universal harness services, task 6 | Four real pinned CLI shims: installed CLI probes, Python shim tests and Rust lifecycle tests. |
| Universal harness services, task 7 | Twelve worker images, Compose/Helm inventory, daemon isolation and native Plandex deployment: image builds, deployment contracts and rendering tests. |
| Universal harness services, task 8 | Deterministic shared-service conformance passes; final native-first twelve-row real-provider gate pending. |
| Cross-harness model routing, tasks 1–12 | Portable model selection, configuration, native codecs, provider execution and images: deterministic suites and pinned image builds. |
| Cross-harness model routing, tasks 13–14 | Migration parity, strict Clippy and coverage pass; final Bazel passes; full live matrix pending. |
| Communication channels, tasks 9–10 and streaming follow-up | Registered parallel agent results, observed metrics and authenticated streaming: API/UI tests, production build and TypeScript checks. |
