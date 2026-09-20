# Native CI quality cleanup — verification checkpoint

Date: 2026-09-19. Project: OneHumanCorp. Branch: `fix/bazel-modernization-and-cleanup`; HEAD `c3716d0875df6403322af4fb47d9f56f9042af3c` plus the preserved, substantial dirty worktree. HEAD alone does not identify the tested source. This continues the owner-authorized native Rust/Cargo + Tauri + Node migration; it does not authorize publication, live provider work or customer charges.

## Acceptance decision

**The entire repository is not yet verified clean, and the full CI time target is not achieved evidence.** Strict headless Rust Clippy is now passing, including all test targets, with warnings denied. Actual regression execution confirms several previously reproduced correctness defects are fixed. Repository-wide Node lint remains failing, and the last full regression reruns have not supplied terminal results.

X was not supplied numerically. Preserve the already implemented **30-minute complete Linux required-CI budget**, the documented **15-minute warm-cache stretch target**, and the **10-minute core build diagnostic**. These are different metrics. Do not replace the full gate with the core build, increase timeouts to hide failures, or report a local cached compile as hosted CI certification.

The native CI graph, dependency-only cache namespace, cache-disabled manual option and timestamp-based performance gate are described in [Native development](../development/native-build.md). The gate includes waits between jobs, but excludes queue time before the first job and its own final reporting/upload step. No hosted CI run was dispatched for this dirty tree.

## Reproduced correctness fixes

### Concurrent in-memory queue creation

`src/server/queue.rs` used a get-then-insert sequence to create topic channels. Concurrent first callers could receive different receiver instances, with one channel subsequently overwritten in the map. The new regression synchronizes eight threads over 32 newly named topics and checks receiver identity. It failed against the old implementation. Creation now uses the atomic DashMap entry operation; the regression passed after the fix. Topic discovery, capacity and queue semantics are preserved.

### Malformed sync payloads falsely acknowledged

`src/server/services/sync/service.rs` parsed a mutation payload with `unwrap_or_default`, turning invalid JSON or a non-array value into an empty batch followed by successful acknowledgment. The new test checks malformed JSON, null and object inputs without a running database. It failed before the change and passed after strict parsing was introduced. These inputs now produce `InvalidArgument`; the payload is not acknowledged as accepted work.

### Tenant cost summaries

New dashboard regressions use two tenants sharing an agent ID plus a private agent in the second tenant. They assert tenant-only amounts, token counts, agent visibility, redaction and rejection of blank tenant identity. Both passed against the refreshed implementation. The production tenant-scoping change advanced concurrently during this work, so do not attribute it solely to this session or infer that every historical reporting path is covered.

### Memory and orchestration tests now exercise behavior

Replaced constant-true or construction-only tests with actual isolated behavior:

- SQLite session memory: failed external-boundary embedding leaves source data pending and creates no consolidated memory; successful boundary summary/embedding persists exactly one tenant-scoped memory, preserves customer metadata, removes source only after persistence, and repeated sweeps do not duplicate the result.
- Department registration: an initially empty orchestrator stores the department and event subscription, without ambient database/provider access.
- Competitor worker initialization: preserves configured database ownership without starting an opt-in network worker.
- Unconfigured telemetry: returns without touching a closed database or making network calls, in sequential and parallel coordinator modes.
- Later strengthened tests cover paused queue dispatch, cross-tenant job completion rejection, failure isolation, and a persisted paused-parent DAG transition with replay rejection. These later tests compile under strict Clippy; their final full-suite execution is still outstanding.

### UI error handling

A shared `src/ui/next/src/lib/errors.ts` helper safely narrows unknown thrown values. It preserves valid messages, handles strings and null/non-error values, protects against a throwing message getter, and does not stringify arbitrary objects. Caller-specific fallback messages remain. It is not a credential redactor; provider boundary code must still create non-sensitive errors.

The helper replaces direct `catch (error: any).message` use across affected proposal, quote, assistant and workflow pages. Workflow-save regression tests prove malformed failures keep the owner's draft intact and re-enable the submit control. Nine helper/workflow tests passed.

### Customer history falsely implied facts and executable actions

The memory-history page accepted unvalidated arbitrary JSON as arrays/React children, rendered a hardcoded High Intent label, and displayed reply/refund buttons without handlers. New tests reproduced malformed-data failures before the change.

`memorySummary.ts` now validates response field types, bounded integer interaction counts and interaction arrays. The page resets state when the customer changes, encodes the customer ID and cancels stale requests. It does not invent the High Intent label. Reply/refund controls are explicitly disabled with an explanation because the page has no corresponding execution path. A zero interaction count is preserved rather than replaced by a truthiness fallback. The eight page cases passed after the fix.

### POS client fabricated a manager after failed authentication

The terminal previously created an Offline Manager identity when disconnected, and an Offline Manager (Fallback) after a failed authentication request. Either branch unlocked the terminal without a verified response.

The client now remains locked for offline, network-error, unsuccessful HTTP and malformed identity responses. It requires a confirmed staff ID, tenant ID, name and role, and prevents concurrent repeated PIN submissions. Seven behavioral tests passed, including rejection of a 401 response with a success-looking body and preservation of an actual STAFF role without elevation. A subsequently discovered test-only unsupported Testing Library option was removed; the final typecheck rerun is pending confirmation.

**Remaining security distinction:** the server POS endpoint still derives identity from its authenticated session and does not verify the entered PIN as an independent credential. This client fix is not a claim of a working second authentication factor. The separately implemented staff-mesh PIN flow must be evaluated for deliberate integration; do not substitute a hash check without reviewing its tenant and session contract. Offline card/payment completion behavior also remains a separate unresolved audit area.

### Other truthful-state corrections

Removed another fabricated Stripe-looking deposit link from onboarding product metadata. Deposit configuration now records `not_configured` payment state rather than claiming a provider session exists. The change compiles; a live Stripe session was not created or tested.

A queue-depth query failure no longer appears as zero jobs and HEALTHY; it produces unknown depth and UNKNOWN status. This removes a false health claim rather than merely silencing a lint warning.

## Structural cleanup without weakening gates

Strict Rust lint findings were reduced from 87 unique baseline findings to 48, then zero. Fixes include compact internal RPC error storage while preserving tonic status code/details/metadata, an actual tonic interceptor implementation, typed memory/task/state-transition records, reuse of existing integration request types, duplicate retry/SQL branches, clearer filesystem-provider constructors and preserved module aliases for the queue contract.

The new RPC error tests check protocol round trips and error representation size. They compile in the final strict lint run; full execution remains pending. Private helper argument changes do not alter protobuf wire schemas. Existing business modules, test discovery, signing requirements and lint rules remain enabled.

Node cleanup removes misleading `any` from selected production and browser test paths, uses existing request/function types where intentionally unused values were previously bound, preserves Unicode code-point bounds and typed mock-call signatures, and retains explicit 501/405 responses for unimplemented routes. A previously vacuous cart-recovery toggle test now verifies its explicitly unavailable state. Tests for asynchronous billing-plan updates now wait for updates rather than redefining the browser location object.

The last completed full ESLint snapshot was **723 findings across 294 files**, down from this session's baseline **807 across 330 files**. The 723 count predates additional confirmed Node edits, so it is not a final exact debt count. Most remaining diagnostics are explicit `any`, unused bindings, CommonJS imports in TypeScript and empty blocks. No rule or source-test coverage was disabled to make the number smaller.

## Executed validation evidence

| Command / scope | Observed result | Qualification |
|---|---|---|
| `cargo fmt --all` followed by `cargo clippy --offline --locked --workspace --exclude app --all-targets --keep-going --message-format=short -- -D warnings` | Exit 0 | Complete headless workspace including test targets; Tauri is a separate required lane. `target/validation/round3-clippy-final.log`. |
| Queue and malformed-sync regressions before production changes | Two failures | Demonstrated the old behavior rather than assuming the defect. |
| Focused queue/sync, dashboard, SQLite memory, orchestrator initialization, competitor initialization and telemetry regressions | Eight passed | `target/validation/round3-regressions-green.log`; predates later interface/test-strengthening changes. |
| `make test-node` earlier in this continuation | Exit 0; 1,516 web tests / 340 files, 59 CLI tests / 13 files, 15 desktop-UI tests / 4 files | Also ran root script tests. Web execution 78.55 seconds, CLI 8.02 seconds, desktop UI 2.31 seconds; these are local test execution timings, not cold install/build/hosted CI. Predates later customer-history/POS/API edits. |
| Error helper and workflow draft-preservation tests | Nine passed | Includes malformed save failures and preservation of the unsaved workflow. |
| Customer-history, cart-recovery, seasonal-promotion and interactive-demo tests | Thirteen passed / four files | Customer-history tests first failed against old behavior, then passed after validation and truthful UI fixes. |
| POS identity regression suite | Seven passed | Client/UI boundary fixtures, no live provider or payment operations. |
| Latest strict Node lint snapshot | Failed, 723 findings / 294 files | Subsequent cleanup is not claimed to have made the full lint gate green. |

The last frontend typecheck first found a test-only unsupported `exact` option; that option was removed using a confirmed guarded edit. An attempted browser typecheck referenced a nonexistent `tsconfig.playwright.json` and failed before checking code. There is no root `tsconfig.json` either. Neither attempt is a passing browser-typecheck result. Use real repository commands/configuration; Playwright enumeration alone is also not full static type checking.

## Interrupted final verification and precise recovery

The last whole-headless-workspace test attempt used an empty environment, no inherited provider credentials, offline Cargo and an isolated network namespace. Its log progressed through real compilation, including the root server, worker and authentication crates, but no terminal test result was received. During the final full Node and type/contract reruns, the host became unresponsive even to small read-only file reads. The last measurable host state included the Rust compiler around 5 GiB resident memory and an unrelated macOS process around 4.2 GiB; this is evidence of shared-host load, not proof of a specific OOM cause. Other projects/processes were not killed or altered.

Owned final verification jobs:

- `dae1fa00-3fad-498a-9a21-45568bfd6464`: full headless Rust suite, `target/validation/round3-full-backend.log` and `.time`; cancellation requested after the runner stopped answering reads. Last observed status `stop_requested`, not confirmed terminated.
- `b338c800-dbf1-493f-8d75-55dbb0655583`: final `make test-node`, `target/validation/round3-node-final.log` and `.time`; cancellation requested, not yet confirmed terminated.
- `e14f3b8f-d330-41c4-b909-2466554fae87`: final web typecheck and `make test-contracts`; last observed running, no terminal result received. Logs are `round3-web-types-final.log` and `round3-contracts-final.log`.

Do not restart overlapping compilers or delete active target directories. First observe these exact jobs and inspect logs/current source. Complete final-source typecheck, all Node/Rust tests, strict Node lint, Tauri lint/tests/build, fresh source-bound web build, full real-stack browser execution and deployment/security lanes before declaring the repository clean. Publish exact failure names and source conditions rather than converting unknown, cancelled or zero-test runs into success.

The remaining product audit items—including provider invoice reconciliation, complete connector OAuth lifecycles, real external payment/delivery evidence and measured owner/customer economics—remain in [the main remediation ledger](native_migration_and_remediation.md). A clean compiler is not proof that these business outcomes have been delivered.
