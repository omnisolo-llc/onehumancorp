# Production agent optimization and security review

Date: 2026-07-12  
Reviewed branch: `main`  
Audit head: `64332c4aa`  
Priority: correctness and security first, then performance and token efficiency

## Executive summary

The production agent path now avoids duplicate tool schemas, no longer caches responses by truncated prompt prefixes, confines outbound HTTP and file access, fails closed on built-in-agent authentication, and isolates circuit breakers per provider client. Focused Cargo and Bazel tests pass for those changes.

The end-to-end audit also found unresolved production boundary defects. Most importantly, the server's general gRPC SPIFFE interceptor trusts an unverified request header, agent-manager mutations do not consistently enforce organization ownership, model-callable business tools accept tenant IDs from model output, and closing an agent result stream does not stop paid producer work. These findings need focused remediation before the cloud path should be considered tenant-safe.

Remediation update: F-01 through F-08 have now been addressed in focused follow-up commits. The original finding text below is retained as the audit snapshot; each resolved finding carries a dated status and verification evidence. Explicit Postgres CI coverage remains open.

## Completed optimization work

| Area | Change | Evidence |
|---|---|---|
| Quality baseline | Added deterministic production-path and parser regressions | Focused Cargo and Bazel tests passed before optimization |
| Token efficiency | Removed serialized native tool definitions from the system prompt; native schemas remain in the provider `tools` field | Request-profile regression verifies one schema representation |
| Response correctness | Removed the application cache keyed by truncated prompt prefixes | Distinct long prompts with common prefixes produce distinct provider calls |
| Outbound security | Added scheme, credential, DNS/IP, redirect, proxy, and response-size policy | 144 tool tests and Bazel tools build passed |
| File security | Canonical workspace confinement, symlink escape rejection, bounded reads/writes, atomic replacement | 154 tool tests and Bazel tools build passed |
| Authentication | Explicit dev/test-only disablement, minimum keyed token configuration, constant-time verification, startup failure propagation, unverifiable SPIFFE rejection | Core/agent/root auth suites and three Bazel library targets passed |
| Resilience | Replaced four provider-global breakers with per-client state and transient-failure classification | 11 Cargo LLM tests and Bazel LLM test passed |
| Tenant-safe agent tools | Bound each agent process to an immutable tenant capability, removed tenant selection from seven model-facing schemas, reused the lazy database pool, set transaction tenant context, and added explicit reschedule predicates | 157 tools tests and parent-crate Cargo check passed |
| Tenant-safe agent memory | Captured the process tenant once at startup and used it for semantic search and completion records, with fail-closed cloud configuration | 517 agent tests and the Bazel agent library test passed |
| Stream cancellation | Replaced the unbounded query stream with a 64-event buffer and raced query execution, gRPC runs, retry backoff, and completion-memory writes against receiver closure | Two drop-observable LLM regressions, 519 agent tests, and the Bazel agent library test passed |
| Memory worker boundaries | Added injectable, deadline-bound summarization; explicit system authority for cross-tenant acquisition/filesystem ingestion; and organization-scoped failure/final mutations | 5 Cargo worker tests and the Bazel workers target passed; real Postgres/RLS assertions remained skipped because `OHC_DATABASE_URL` was unset |
| Telemetry privacy | Removed raw task and error bodies from LangSmith/Langfuse logs while retaining provider, event, run ID, and coarse error class | Emitted-log capture regression, 2 observability tests, and all 520 agent tests passed |
| JavaScript supply chain | Constrained vulnerable transitive releases in root/UI pnpm and npm graphs and added four production audit gates to CI | Root/UI pnpm and npm production audits report zero vulnerabilities; 817 UI tests, 58 CLI tests, MCP smoke test, and Bazel UI target pass |

### UI-01 — Universal UI shell and rendered consistency

**Status (2026-07-12): Verified in production mode.** The styling outage came from a Tailwind 4 PostCSS pipeline being applied to an application whose configuration and utilities target Tailwind 3. The standalone UI now uses `tailwindcss` 3.4.19 through the standard `tailwindcss` and `autoprefixer` PostCSS plugins; `npm run test:tailwind-config` reports `Tailwind/PostCSS pipeline is coherent.`

Shell ownership is explicit rather than inferred from page markup. `ProductShellGuard` resolves every route to either universal `AppShell` ownership or intentional page ownership, and `AppShell` owns the sole sidebar, compact navigation, topbar, main canvas, status/actions, Help Center entry, and Voice Assistant. The responsive repair constrains sticky/phone canvases to the shell content box, keeps document overflow at zero, preserves horizontal scrolling only inside compact navigation/tab lists, and normalizes shared card/panel/list surfaces to the common 8 px design ceiling while preserving working Tailwind utility generation. Mobile root-layout controls no longer compete with shell actions: the two redundant help triggers hide below `sm`, while the single Voice Assistant renders in normal sticky-topbar flow on mobile and remains viewport-fixed at bottom center on desktop. Stable root, trigger, status-surface, and state markers cover listening, processing, success, and error. Media ownership is unmount-safe: pending acquisition, recorder callbacks, streams, tracks, and reset timers are session-guarded and cleaned up idempotently without sending late audio or logging raw failures. Mouse/touch and non-repeating Enter/Space preserve hold-to-talk, synthetic assistive activation provides a non-pointer toggle, and dynamic pressed/label/live-status semantics announce operation and state. A 320/390 px collision matrix now checks initial and scrolled geometry plus every active state against the viewport, topbar, shell navigation, sibling actions, and exposed product controls.

Inbox hydration and offline behavior are stable. PowerSync opens its local database after client mount, holds the settled local empty/data state through connector or backend sync failure, and does not replace server HTML during hydration. The empty inbox regression also rejects literal `\\n` text artifacts. Production browser coverage waits for `inbox-settled` and recorded zero uncaught page errors or hydration mismatch/replacement messages on desktop and mobile.

Fresh verification at the final UI head produced the following exact evidence:

- `pnpm exec vitest run`: 223 files and 893 tests passed, zero failures. Voice regressions cover normal and unmount cleanup, late and overlapping media acquisition, stale-recorder completion ownership, exact-once track release, callback/send suppression, mouse/touch/cancel and Enter/Space hold behavior, repeat suppression, assistive synthetic-click toggling, dynamic pressed/label state, and polite/assertive live announcements.
- `pnpm exec tsc --noEmit`: exit 0 with no diagnostics. This required loading `vitest/globals` in the TypeScript environment and aligning test route contexts/browser mocks/fixtures with their production contracts; checks were not suppressed.
- `pnpm run build`: Next.js 14.2.35 compiled and type-checked successfully, generated 427/427 static pages, and exited 0. A pre-existing duplicate `simulateInvoiceDraft` handler and control in `/feed` was removed with a source regression that requires one owner.
- Production Playwright, using the config `webServer`, line reporter, one worker, and `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium`: 58/58 passed in 1.3 minutes. This comprises the original 36 app-shell cases, 12 mobile collision cases at 320/390 px, 3 desktop/active-state Voice Assistant geometry cases, and 7 styled-page checks. Every shell case found exactly one `.app-sidebar`, `.app-topbar`, and `.app-main`, no horizontal document overflow, and no shared surface radius above the normalized threshold. The active-state cases exercise mocked microphone capture and deterministic processing/success/error responses after vertical scroll.
- The secured `scripts/visual-audit.mjs` run against `next start`: 36 pages, 0 failures, `coverageComplete: true`, no fatal error, and 36/36 screenshots. The matrix is 18 routes by 2 viewports: dashboard, assistant, orders, inventory, inbox, agents, settings, business analytics, integrations, calendar, diagnostics, agent marketplace, visual workflow, website builder, booking widget, storefront widget, onboarding, and login at 1440x1000 and 390x844.
- Original-resolution inspection confirmed one navigation shell, readable hierarchy, unobscured actions, consistent shared surfaces, and no clipping on desktop/mobile dashboard, agents, integrations, website builder, and login, plus mobile agent marketplace and inbox. The agents/integrations tab rows are valid local horizontal scrollers rather than document overflow. Fresh 320/390 px scrolled captures of listening, processing, success, and error show each status panel and stable trigger fully contained in the sticky topbar with clear separation from integration Connect actions; website-builder and login calls to action, marketplace search/error state, and inbox panels also remain unobscured.
- `bazel test //src/ui/next:next_vitest --test_output=errors`: 1/1 target passed.
- Root/UI `pnpm audit --prod`: both report `No known vulnerabilities found`. Root/UI `npm audit --omit=dev`: both report `found 0 vulnerabilities`.

The production server still logs expected missing-service errors in this isolated environment: proxy/fetch connection refusals for local backends on ports 8080 and 18789, Postgres on 5432, and the backend-served Swagger CSS/bundle. Next also identifies request-header/request-URL API routes as dynamic during static generation. These messages did not cause navigation, hydration, shell, screenshot, test, or build failures.

## Boundary matrix

| Path | Trust boundary and authorization source | Tenant handling | Deadline/cancellation | Telemetry | Conclusion |
|---|---|---|---|---|---|
| HTTP agent chat (`src/server/api/agents/chat.rs`) | Axum `Claims` extension | Rejects missing `organization_id`; passes the claim-derived tenant to routing and orchestration | No explicit route-level deadline found | Prompt is placed in action payload; not directly logged here | Tenant source is sound; deadline remains unverified |
| Built-in agent gRPC (`src/agents/builtin/service.rs`) | Token auth or a non-serializable trusted in-process extension | No request tenant; successful memory writes use process-global `OHC_ORGANIZATION_ID`, defaulting to `system` | Provider clients and each run attempt have deadlines; output channel is bounded, but producer ignores receiver closure | Instrumentation skips request values | Authentication improved; tenant attribution and cancellation fail requirements |
| Agent manager gRPC (`src/server/services/agent/service.rs`) | Server-wide `spiffe_interceptor` | Header-derived org is used for snapshots, but not all mutations/reads are scoped to it | Spawned tasks have no local timeout | No prompt logging in reviewed methods | Authentication and multiple ownership checks fail |
| Session-memory pipeline (`src/server/workers/agent_memory_pipeline.rs`) | Background system worker | Cross-tenant fetch intentionally clears tenant context; final Postgres insert sets row tenant context, while failure updates use the pool without tenant context | Embedding is limited to 60 seconds; summarization calls have no deadline | Provider errors are logged verbatim | Mixed controls; several high-risk gaps |
| Native business tools (`src/agents/builtin/tools`) | Model-generated tool arguments | Quote and booking schemas expose `tenant_id`; executors trust it | Database connect/query paths have no explicit deadline | Database errors are returned to the model | Confirmed tenant-confusion boundary |

## Confirmed findings

### F-01 — Critical — client-spoofable SPIFFE authentication

`src/server/lib.rs:418` reads `x-spiffe-id` directly from request metadata and treats successful string parsing as authentication. The Tonic server is not configured with client-certificate verification on this path. `src/server/auth/mod.rs:768` only checks slash positions; it accepts untrusted domains and empty organization/agent components. A remote client can therefore manufacture the identity used by intercepted gRPC services.

Smallest regression: `spiffe_interceptor_rejects_identity_without_verified_peer_certificate`.

### F-02 — High — cross-tenant agent-manager reads and mutations

`fire_agent` derives an organization but calls `Hub::fire_agent` by caller-supplied agent ID without verifying ownership (`src/server/services/agent/service.rs:153`). `delegate_task` likewise accepts globally registered sender and recipient IDs without checking either organization (`:171`). `get_identities` reads all agents (`:215`), while skills and snapshots are stored in global vectors and returned without organization filtering (`:232`, `:267`).

Smallest regressions: `fire_agent_rejects_other_org_agent`, `delegate_task_rejects_cross_org_agents`, and `snapshots_are_scoped_to_authenticated_org`.

### F-03 — High — model-controlled tenant IDs in production business tools

**Status (2026-07-12): Remediated.** Commits `7ad1f0fc4` and `f6ccafb41` inject an immutable `TenantContext` into all booking and quote executors. Their model-facing schemas no longer contain `tenant_id`; PostgreSQL transactions use the captured context, quote generation reuses the shared lazy pool, quote line items carry the tenant explicitly, and rescheduling reads/updates include tenant predicates. The regression `tenant_aware_tool_schemas_do_not_expose_tenant_selection` passes as part of all 157 tools-crate tests.

The default production tool set registers booking and quote mutations (`src/agents/builtin/tools/mod.rs:137` and `:167`). `generate_quote` requires `tenant_id` in the model-facing schema and binds that value directly into the insert (`src/agents/builtin/tools/quote.rs:8`, `:48`, `:118`) without setting authenticated database tenant context. Booking tools follow the same caller-controlled pattern. A prompt injection or model error can select another tenant when the database role bypasses RLS; with enforced RLS, these paths can fail unpredictably instead.

Smallest regression: `generate_quote_uses_authenticated_tenant_not_tool_arguments`.

### F-04 — High — stream cancellation does not cancel producer work

**Status (2026-07-12): Remediated.** Commits `ef38d1ae9` and `52bc4710c` replace the unbounded `Agent::query` channel with a 64-event buffer and race both query and gRPC producer futures against receiver closure. The service also cancels retry backoff and completion-memory writes after disconnect. Drop-observable LLM regressions prove that both producer layers cancel the actual in-flight provider future; all 519 built-in-agent tests and the Bazel agent library target pass.

`RunTaskStream` uses a bounded channel of 64, but the producer calls `try_send` and discards full/closed errors (`src/agents/builtin/service.rs:941`, `:1043`). It never tests receiver closure before retries, LLM calls, tools, or memory writes. The lower-level `Agent::query` creates an unbounded channel and similarly ignores send failure (`src/agents/builtin/agent.rs:2888`). A disconnected client can leave costly work running to completion, and the unbounded path can grow without backpressure.

Smallest regressions: `run_task_stops_when_receiver_is_dropped` and `query_applies_bounded_backpressure`.

### F-05 — High — agent memory tenant attribution is process-global

**Status (2026-07-12): Remediated.** Commits `7ad1f0fc4` and `aaf947781` require a non-system `OHC_ORGANIZATION_ID` for cloud/cluster agent startup, capture it as an immutable process capability, and use it for both semantic search and completion records. Standalone mode may explicitly use the local `system` tenant. The focused memory regression, all 517 built-in-agent tests, and `//src/agents/builtin:ohc_builtin_agent_lib_unit_test` pass.

After a successful task, the built-in service assigns memory to `OHC_ORGANIZATION_ID`, defaulting to `system`, rather than an authenticated request tenant (`src/agents/builtin/service.rs:1095`). The public task request carries no enforced tenant identity. A shared agent process can therefore misattribute or combine memories across callers.

Smallest regression: `run_task_memory_uses_authenticated_request_tenant`.

### F-06 — High — memory worker has incomplete tenant and timeout controls

**Status (2026-07-12): Remediated in code; real Postgres/RLS execution remains unverified.** Commits `8be295f0c` and `68b3649c2` add a testable summarization boundary with a 60-second deadline, redact provider error bodies, use explicit system authority for cross-tenant acquisition and filesystem ingestion, and run failure/final mutations under the row's organization. Failure resets require both session and agent IDs. Five focused Cargo tests and `//src/server/workers:server_workers_unit_test` pass. Because `OHC_DATABASE_URL` was unset, the Postgres-named Cargo test returned early; F-10 still blocks an end-to-end RLS verification claim.

The Postgres worker explicitly clears tenant context to fetch work across tenants (`src/server/workers/agent_memory_pipeline.rs:154`). Final inserts set tenant context, but failure-status updates execute directly on the pool with only `session_id` (`:226`, `:237`). Summarization provider calls at `:204` are not wrapped in a deadline, although embedding calls are. The filesystem-memory Postgres insert also lacks an explicit tenant transaction. These paths are safe only under undocumented role and globally-unique-ID assumptions.

Smallest regressions: `memory_failure_update_is_tenant_scoped`, `memory_summary_has_deadline`, and `fs_memory_insert_sets_system_tenant_context`.

### F-07 — High — raw tasks and provider errors enter logs

**Status (2026-07-12): Remediated.** The observability providers no longer interpolate task or error bodies. Run/error events retain structured provider, event, run ID, and a coarse non-sensitive error class. The regression `observability_logs_metadata_without_task_or_error_body` captures actual tracing output and proves sentinel task/error secrets are absent; both observability tests and all 520 built-in-agent tests pass.

Both observability implementations log the complete task at run start and raw error strings (`src/agents/builtin/observability.rs:31`, `:57`, `:77`, `:99`). Tasks can contain prompts, customer data, credentials, and tool material; provider/database error bodies can also contain sensitive values. Other reviewed telemetry methods correctly omit request, response, final output, and tool result bodies.

Smallest regression: `observability_logs_metadata_without_task_or_error_body`.

### F-08 — High — production JavaScript dependency advisories

**Status (2026-07-12): Remediated.** Root and standalone UI pnpm policies now constrain patched MCP SDK, DOMPurify, `form-data`, `undici`, `ws`, PostCSS, `js-yaml`, `esbuild`, and `brace-expansion` releases; equivalent npm overrides keep both tracked npm lockfiles safe. All four production audits report zero vulnerabilities. CI now audits both package-manager graphs. Compatibility evidence includes 217 UI test files/817 tests, 13 CLI test files/58 tests, an MCP stdio startup smoke test, and `//src/ui/next:next_vitest`. The deprecated `@modelcontextprotocol/server-github` wrapper remains a maintenance risk, but its transitive SDK is patched. A later UI contract remediation supersedes the original baseline note: the standalone UI TypeScript check now passes without suppressed diagnostics; no claim is made here about the separate CLI TypeScript baseline.

`pnpm audit --prod` reports 14 advisories: 7 high, 4 moderate, and 3 low. High-severity paths include `@modelcontextprotocol/sdk` ReDoS and DNS-rebinding issues, `form-data` CRLF injection, three `undici` proxy/TLS/routing issues, and legacy `ws` memory-exhaustion DoS. Patched versions were identified by the audit and should be applied with targeted compatibility tests.

Smallest regression: lockfile audit in CI with an explicit, expiring advisory allowlist.

### F-09 — Medium — tracked opaque JWT-secret candidate

`src/server/auth/.ohc_jwt_secret` is a tracked 32-byte opaque file. Runtime code writes the active secret under the safe user directory rather than reading this source-tree path, so production use is not proven. Nevertheless, a secret-shaped artifact is present in Git history and should be removed and any potentially related signing material rotated. Its contents were not printed or copied during this review.

### F-10 — Medium — tenant tests silently skip while reporting success

All six `server_auth` multitenancy tests return immediately when `OHC_DATABASE_URL` is unset. In this environment Cargo reported 6 passed in 0.00 seconds even though no Postgres assertions ran. Several Postgres queue and memory tests use similar environment-sensitive setup. CI needs an explicit Postgres lane, and skip conditions must be surfaced as skips or failures in the security lane.

Smallest regression: `multitenancy_suite_requires_postgres_in_ci`.

## Passing and unverified test evidence

| Command | Result | Interpretation |
|---|---|---|
| `cargo test -p server_auth multitenancy_isolation -- --nocapture` | 6 reported passed in 0.00s | **Unverified:** `OHC_DATABASE_URL` was unset and tests returned early |
| `cargo test -p ohc-mono --lib agent_memory_pipeline -- --nocapture` | 3 passed | SQLite and timeout behavior covered; real Postgres isolation remains environment-dependent |
| `cargo test -p ohc-mono --lib services::agent -- --nocapture` | 7 passed | Happy-path tests only; no cross-organization negative cases |
| `cargo test -p ohc-mono --lib orchestration::queue -- --nocapture` | 17 passed | SQLite behavior covered; Postgres/RLS claims require a configured database to be considered verified |
| `cargo test -p ohc_builtin_agent service -- --nocapture` | 7 passed | Auth/configuration and basic service behavior pass; no receiver-cancellation regression exists |
| `cargo test -p ohc_builtin_agent_tools --lib` | 157 passed | Tenant-aware tool schemas no longer expose tenant selection; tools regressions remain green |
| `cargo test -p ohc_builtin_agent --lib` | 520 passed | Process tenant, captured-memory, cancellation, and emitted-log redaction regressions pass with the full agent suite |
| `bazel test //src/agents/builtin:ohc_builtin_agent_lib_unit_test` | 1 target passed | Bazel build/test graph includes and validates the tenant-capability changes |
| `cargo test -p ohc-mono --lib agent_memory_pipeline` | 5 passed | Deterministic summary deadline and scoped SQL shape pass; the Postgres-named test skipped its database body because `OHC_DATABASE_URL` was unset |
| `bazel test //src/server/workers:server_workers_unit_test` | 1 target passed | Worker crate and its full Bazel dependency graph build and test successfully |

## Dependency and secret scanning

- `cargo audit` is not installed, so Rust advisory status is unverified. `cargo tree -d` completed and showed substantial duplicate dependency families, including Axum 0.7/0.8 and Tower 0.4/0.5; this is maintenance and binary-size debt, not itself a vulnerability.
- The audit snapshot initially reported 14 pnpm advisories (7 high, 4 moderate, 3 low). After remediation, root/UI pnpm and root/UI npm production audits each report zero vulnerabilities.
- No private-key PEM blocks or the removed `default_auth_key_change_me` fallback were found.
- The credentialed-Postgres-URL pattern matched 66 files in the scan scope (the `docs/` tree was excluded). Most inspected paths are tests, local defaults, or deployment templates; they need environment-by-environment validation before being classified as real credentials. No values are reproduced here.

## Remediation order

1. Replace header-trusting SPIFFE interception with verified mTLS peer identity and reject empty/untrusted identities.
2. Scope all agent-manager operations, skills, identities, and snapshots to the authenticated organization.
3. Remove tenant IDs from model-facing tool schemas and inject an authenticated tenant capability into executors.
4. Propagate stream cancellation into producer tasks and replace unbounded event channels.
5. Carry authenticated tenant context through built-in task execution and memory writes.
6. Add deadlines and tenant-scoped transactions to every memory-worker provider/database path.
7. Redact task/error bodies from logs and add telemetry field tests.
8. Upgrade vulnerable JavaScript dependency paths and add enforced advisory scanning.
9. Remove the tracked secret candidate and make Postgres security tests explicit in CI.

## Benchmark and final verification

Reproducible before/after benchmark data and the complete final formatting/lint/test matrix will be added in the next phase. UI visual consistency is also a newly requested review track and will be documented separately after rendered desktop/mobile inspection.
