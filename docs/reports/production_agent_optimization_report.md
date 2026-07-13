# Production agent optimization and security review

Date: 2026-07-13
Reviewed branch: `main`  
Initial audit head: `64332c4aa`
Priority: correctness and security first, then performance and token efficiency

## Executive summary

The production agent path now avoids duplicate tool schemas, no longer caches responses by truncated prompt prefixes, confines outbound HTTP and file access, fails closed on built-in-agent authentication, and isolates circuit breakers per provider client. Focused Cargo and Bazel tests pass for those changes.

The initial end-to-end audit also found production boundary defects: the general gRPC SPIFFE interceptor trusted an unverified request header, agent-manager mutations lacked consistent organization ownership checks, model-callable business tools accepted tenant IDs from model output, and closing an agent result stream did not stop paid producer work. Those current-tree defects were remediated in the focused commits recorded under F-01 through F-11. Operational items that code changes cannot complete—especially credential rotation, history/log assessment, and remote CI execution—remain explicitly listed.

Remediation update: F-01 through F-11 have now been addressed in focused follow-up commits. The original finding text below is retained as the audit snapshot; each resolved finding carries a dated status and verification evidence.

## Completed optimization work

| Area | Change | Evidence |
|---|---|---|
| Quality baseline | Added deterministic production-path and parser regressions | Focused Cargo and Bazel tests passed before optimization |
| Structured-output quality | Removed fenced-markdown deserialization that bypassed the native `structured_output` recovery contract; strengthened retry and call-count assertions | 3 focused regressions, 77 core tests, doc tests, and the Bazel agent unit target passed |
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
| Build credential hygiene | Removed tracked literal remote-cache/BES API headers and added fail-closed tracked-file enforcement | Disposable Git behavior tests, current-tree hygiene scan, and tracked-rc Bazel announcement scan pass without emitting credential values |

### UI-01 — Universal UI shell and rendered consistency

**Status (2026-07-12): Verified in production mode.** The styling outage came from a Tailwind 4 PostCSS pipeline being applied to an application whose configuration and utilities target Tailwind 3. The standalone UI now uses `tailwindcss` 3.4.19 through the standard `tailwindcss` and `autoprefixer` PostCSS plugins; `npm run test:tailwind-config` reports `Tailwind/PostCSS pipeline is coherent.`

Shell ownership is explicit rather than inferred from page markup. `ProductShellGuard` resolves every route to either universal `AppShell` ownership or intentional page ownership, and `AppShell` owns the sole sidebar, compact navigation, topbar, main canvas, status/actions, Help Center entry, and Voice Assistant. The responsive repair constrains sticky/phone canvases to the shell content box, keeps document overflow at zero, preserves horizontal scrolling only inside compact navigation/tab lists, and normalizes shared card/panel/list surfaces to the common 8 px design ceiling while preserving working Tailwind utility generation. Mobile root-layout controls no longer compete with shell actions: the two redundant help triggers hide below `sm`, while the single Voice Assistant renders in normal sticky-topbar flow on mobile and remains viewport-fixed at bottom center on desktop. Stable root, trigger, status-surface, and state markers cover listening, processing, success, and error. Media ownership is unmount-safe: pending acquisition, recorder callbacks, streams, tracks, and reset timers are session-guarded and cleaned up idempotently without sending late audio or logging raw failures. Starting a new recording cancels prior resets, and every remaining reset checks its owning session before changing status or transcription. Mouse/touch and non-repeating Enter/Space preserve hold-to-talk, synthetic assistive activation provides a non-pointer toggle, and dynamic pressed/label/live-status semantics announce operation and state. A 320/390 px collision matrix now checks initial and scrolled geometry plus every active state against the viewport, topbar, shell navigation, sibling actions, and exposed product controls.

Inbox hydration and offline behavior are stable. PowerSync opens its local database after client mount, holds the settled local empty/data state through connector or backend sync failure, and does not replace server HTML during hydration. The empty inbox regression also rejects literal `\\n` text artifacts. Production browser coverage waits for `inbox-settled` and recorded zero uncaught page errors or hydration mismatch/replacement messages on desktop and mobile.

Fresh verification at the final UI head produced the following exact evidence:

- `pnpm exec vitest run`: 224 files and 902 tests passed, zero failures. Voice regressions cover normal and unmount cleanup, late and overlapping media acquisition, stale-recorder completion ownership, session-owned success/error resets, exact-once track release, callback/send suppression, mouse/touch/cancel and Enter/Space hold behavior, repeat suppression, assistive synthetic-click toggling, dynamic pressed/label state, and polite/assertive live announcements. Pure visual-audit policy regressions prove fail-closed page-error, hydration, unexpected-console, screenshot, and coverage behavior while narrowly classified isolated-service/resource errors remain nonfatal.
- `pnpm exec tsc --noEmit`: exit 0 with no diagnostics. This required loading `vitest/globals` in the TypeScript environment and aligning test route contexts/browser mocks/fixtures with their production contracts; checks were not suppressed.
- `pnpm run build`: Next.js 14.2.35 compiled and type-checked successfully, generated 427/427 static pages, and exited 0. A pre-existing duplicate `simulateInvoiceDraft` handler and control in `/feed` was removed with a source regression that requires one owner.
- Production Playwright, using the config `webServer`, line reporter, one worker, and `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium`: 58/58 passed in 1.3 minutes. This comprises the original 36 app-shell cases, 12 mobile collision cases at 320/390 px, 3 desktop/active-state Voice Assistant geometry cases, and 7 styled-page checks. Every shell case found exactly one `.app-sidebar`, `.app-topbar`, and `.app-main`, no horizontal document overflow, and no shared surface radius above the normalized threshold. The active-state cases exercise mocked microphone capture and deterministic processing/success/error responses after vertical scroll.
- The secured `scripts/visual-audit.mjs` run against `next start`: 36 pages, 0 failures, `coverageComplete: true`, no fatal error, and 36/36 verified screenshot files. It waits for the shell and inbox settled state, observes a bounded post-ready window, and records console, page, hydration, and expected isolated-service/resource failures separately. The final run classified 164 errors by explicit message/location rules and recorded 0 unexpected console errors, 0 uncaught page errors, and 0 hydration errors. Invalid-browser and invalid-output-path probes exited nonzero with incomplete coverage; the output directory remained mode 0700 and report/screenshots mode 0600. The matrix is 18 routes by 2 viewports: dashboard, assistant, orders, inventory, inbox, agents, settings, business analytics, integrations, calendar, diagnostics, agent marketplace, visual workflow, website builder, booking widget, storefront widget, onboarding, and login at 1440x1000 and 390x844.
- Original-resolution inspection confirmed one navigation shell, readable hierarchy, unobscured actions, consistent shared surfaces, and no clipping on desktop/mobile dashboard, agents, integrations, website builder, and login, plus mobile agent marketplace and inbox. The agents/integrations tab rows are valid local horizontal scrollers rather than document overflow. Fresh 320/390 px scrolled captures of listening, processing, success, and error show each status panel and stable trigger fully contained in the sticky topbar with clear separation from integration Connect actions; website-builder and login calls to action, marketplace search/error state, and inbox panels also remain unobscured.
- `bazel test //src/ui/next:next_vitest --test_output=errors`: 1/1 target passed.
- Root/UI `pnpm audit --prod`: both report `No known vulnerabilities found`. Root/UI `npm audit --omit=dev`: both report `found 0 vulnerabilities`.

The production server still logs expected missing-service errors in this isolated environment: proxy/fetch connection refusals for local backends on ports 8080 and 18789, Postgres on 5432, and the backend-served Swagger CSS/bundle. The audit allows only the enumerated message plus URL/port/path combinations; an identical 500 response on an unknown API path is a tested failure. Next also identifies request-header/request-URL API routes as dynamic during static generation. These messages did not cause navigation, hydration, shell, screenshot, test, or build failures.

### CHAT-00 — Chatwoot removal

**Status (2026-07-13): Removed from the active application and deployment graph.** The repository owner confirmed in this thread on 2026-07-13 that there was no real-customer or production Chatwoot deployment or data. That owner statement was not independently verified against an external production system. On that owner-confirmed basis, no Chatwoot data migration was performed. The native OHC omnichannel inbox remains in place; expanding its channel and inbox capabilities belongs to later native-chat projects rather than this removal.

The removal deleted the unused Rust integration crate and its Cargo workspace/dependency, integration re-export, Tauri Bazel-manifest, lockfile, and Bazel graph edges. Deployment removal covered the three Compose services (migration, web, and worker), related backend environment/database initialization, both Prometheus scrape jobs, the dedicated Helm workload and service templates, Helm values and backend environment, HPA, ServiceMonitor, NetworkPolicy and ingress peers, deploy Bazel filegroups, and the Kind override. Active product surfaces now advertise the native inbox and direct Instagram DM, WhatsApp, SMS, and email connectors. Active README, developer, business, customer-success, and C++ evaluation claims were updated; exactly three research reports are retained with explicit superseded-architecture markers. A fail-closed tracked-file residue guard and its CI step enforce the active-source boundary.

Removal-matrix evidence captured at `36d36f4337526841344b98103230fcaa1ac1e9d8`:

- `bash deploy/tests/no_chatwoot_residue_test.sh` exited 0 with no output.
- `cargo check -p ohc-mono --locked` exited 0. Cargo emitted four existing unused-variable warnings for `db` in `src/server/api/staff_mesh.rs`; no warning was suppressed for this work.
- `bazel test --config=local --remote_cache= --remote_executor= --bes_backend= //src/server/integrations:server_integrations_unit_test //src/ui/next:next_vitest //deploy:deploy_artifacts_test --test_output=errors` exited 0 with 3/3 Bazel test targets passing. The integration log reports 6 passed and 0 failed; the Vitest log reports 235 test files and 943 tests passed; the deployment artifact target reports its checks passed. Bazel also warned that some declared test sizes exceed its size guidance.
- The initial separately produced Bazel graph contained 7,283 labels; its producer exited 0 before the negative scan found zero Chatwoot labels.

That initial graph result was then reproduced from the corrected report tree with a private producer file and a separate negative command. The producer must succeed before the residue status is evaluated; status 1 is required to mean no match. The exact block was:

```bash
set -euo pipefail
umask 077
query_dir=$(mktemp -d /tmp/ohc-chatwoot-bazel-query.XXXXXX)
printf 'query_dir=%s\n' "$query_dir"
(
  trap 'rm -rf "$query_dir"' EXIT HUP INT TERM
  test "$(stat -c '%a' "$query_dir")" = 700

  bazel query //... >"$query_dir/labels.txt" 2>"$query_dir/query.err"
  label_count=$(wc -l <"$query_dir/labels.txt")
  test "$label_count" -eq 7283

  set +e
  rg -i 'chatwoot' "$query_dir/labels.txt" >/dev/null
  residue_status=$?
  set -e
  test "$residue_status" -eq 1

  chmod -R go-rwx "$query_dir"
  test "$(stat -c '%a' "$query_dir")" = 700
  if find "$query_dir" -mindepth 1 -perm /077 -print -quit | grep -q .; then
    printf 'non-private query evidence entry found\n' >&2
    exit 1
  fi
  printf 'bazel_query_labels=%s\nresidue_scan_status=%s\nprivate_directory_mode=%s\n' \
    "$label_count" "$residue_status" "$(stat -c '%a' "$query_dir")"
)
test ! -e "$query_dir"
printf 'secure_query_dir_absent=1\n'
```

The block exited 0 and reported `bazel_query_labels=7283`, `residue_scan_status=1`, `private_directory_mode=700`, and `secure_query_dir_absent=1`. It printed neither label contents nor query diagnostics.

The deployment renders and locked Cargo metadata were rerun from the corrected report tree in one private, copy-pasteable block. The main default inventory intentionally activates 5 services, while `--profile '*'` expands it to all 11 declared services. The e2e default and all-profile inventories each contain the same 2 services. The exact block was:

```bash
set -euo pipefail
umask 077
evidence_dir=$(mktemp -d /tmp/ohc-chatwoot-verify.XXXXXX)
printf 'evidence_dir=%s\n' "$evidence_dir"
(
  trap 'rm -rf "$evidence_dir"' EXIT HUP INT TERM
  test "$(stat -c '%a' "$evidence_dir")" = 700

  cargo metadata --locked --format-version=1 --no-deps >"$evidence_dir/cargo-metadata.json"

  docker compose -f deploy/docker-compose.yml config >"$evidence_dir/compose-main-default.yaml"
  docker compose -f deploy/docker-compose.yml config --services >"$evidence_dir/compose-main-default-services.txt"
  docker compose -f deploy/docker-compose.yml --profile '*' config >"$evidence_dir/compose-main-all-profiles.yaml"
  docker compose -f deploy/docker-compose.yml --profile '*' config --services >"$evidence_dir/compose-main-all-profile-services.txt"
  docker compose -f deploy/docker-compose.e2e.yml config >"$evidence_dir/compose-e2e-default.yaml"
  docker compose -f deploy/docker-compose.e2e.yml config --services >"$evidence_dir/compose-e2e-default-services.txt"
  docker compose -f deploy/docker-compose.e2e.yml --profile '*' config >"$evidence_dir/compose-e2e-all-profiles.yaml"
  docker compose -f deploy/docker-compose.e2e.yml --profile '*' config --services >"$evidence_dir/compose-e2e-all-profile-services.txt"

  main_default_count=$(wc -l <"$evidence_dir/compose-main-default-services.txt")
  main_all_count=$(wc -l <"$evidence_dir/compose-main-all-profile-services.txt")
  e2e_default_count=$(wc -l <"$evidence_dir/compose-e2e-default-services.txt")
  e2e_all_count=$(wc -l <"$evidence_dir/compose-e2e-all-profile-services.txt")
  test "$main_default_count" -eq 5
  test "$main_all_count" -eq 11
  test "$e2e_default_count" -eq 2
  test "$e2e_all_count" -eq 2

  mkdir -p "$evidence_dir/chart/ohc"
  cp -a deploy/helm/ohc/. "$evidence_dir/chart/ohc/"
  helm dependency build "$evidence_dir/chart/ohc"
  helm lint "$evidence_dir/chart/ohc"
  helm template ohc "$evidence_dir/chart/ohc" >"$evidence_dir/helm-render.yaml"

  rendered_outputs=(
    "$evidence_dir/cargo-metadata.json"
    "$evidence_dir/compose-main-default.yaml"
    "$evidence_dir/compose-main-default-services.txt"
    "$evidence_dir/compose-main-all-profiles.yaml"
    "$evidence_dir/compose-main-all-profile-services.txt"
    "$evidence_dir/compose-e2e-default.yaml"
    "$evidence_dir/compose-e2e-default-services.txt"
    "$evidence_dir/compose-e2e-all-profiles.yaml"
    "$evidence_dir/compose-e2e-all-profile-services.txt"
    "$evidence_dir/helm-render.yaml"
  )
  if rg -q -i 'chatwoot' -- "${rendered_outputs[@]}"; then
    printf 'rendered Chatwoot residue found\n' >&2
    exit 1
  else
    rg_status=$?
    test "$rg_status" -eq 1
  fi

  chmod -R go-rwx "$evidence_dir"
  test "$(stat -c '%a' "$evidence_dir")" = 700
  if find "$evidence_dir" -mindepth 1 -perm /077 -print -quit | grep -q .; then
    printf 'non-private evidence entry found\n' >&2
    exit 1
  fi
  printf 'service_counts main_default=%s main_all_profiles=%s e2e_default=%s e2e_all_profiles=%s\n' \
    "$main_default_count" "$main_all_count" "$e2e_default_count" "$e2e_all_count"
  printf 'rendered_residue_matches=0\nprivate_directory_mode=%s\n' "$(stat -c '%a' "$evidence_dir")"
)
test ! -e "$evidence_dir"
printf 'secure_evidence_dir_absent=1\n'
```

The block exited 0. Locked metadata, all eight explicit Compose outputs/inventories, isolated Helm dependency build, lint, template, negative scans, permission checks, and cleanup therefore completed in sequence. Helm reported 1 chart linted and 0 failed, with only the recommendation to add a chart icon. It reported `main_default=5`, `main_all_profiles=11`, `e2e_default=2`, `e2e_all_profiles=2`, zero rendered residue matches, directory mode 0700, no group/world-accessible descendant, and `secure_evidence_dir_absent=1`. No chart dependency or generated evidence artifact was added to the repository.

Before the secure rerun, the twelve earlier predictable artifacts—`ohc-cargo-metadata-final.json`, both default Compose renders, both all-profile Compose renders, four Compose service inventories, `ohc-helm-final.yaml`, and both Bazel-query output files—were inventoried at mode 0664, removed by their exact paths, and confirmed absent. They were not retained.

Post-report checks at the final report HEAD are distinct from the removal matrix at `36d36f4`: `git grep -l -i 'chatwoot' -- . | sort` returned exactly eight allowed tracked paths in the corrected report tree: `.github/workflows/ci.yml`, the residue guard, this evidence report, the native removal plan and omnichannel design, and exactly three marked historical research reports. `git diff --check` and the residue guard exited 0 with no output. The only remaining worktree entry outside the report was the unrelated untracked `docs/superpowers/plans/2026-07-13-authentication-hardening.md`.

All required local tools were available. This evidence does not claim a remote CI run, live-cluster rollout, or external production-system check; those checks were not run. Earlier default Bazel remote-cache/BES attempts in this environment were unauthorized, so the explicit local Bazel matrix above is the authoritative final evidence.

## Boundary matrix — initial pre-remediation snapshot

This matrix records the state observed at the initial audit head. It is retained as finding evidence, not as a description of current HEAD; the dated status blocks under F-01 through F-11 are the current-state record.

| Path | Trust boundary and authorization source | Tenant handling | Deadline/cancellation | Telemetry | Conclusion |
|---|---|---|---|---|---|
| HTTP agent chat (`src/server/api/agents/chat.rs`) | Axum `Claims` extension | Rejects missing `organization_id`; passes the claim-derived tenant to routing and orchestration | No explicit route-level deadline found | Prompt is placed in action payload; not directly logged here | Tenant source is sound; deadline remains unverified |
| Built-in agent gRPC (`src/agents/builtin/service.rs`) | Token auth or a non-serializable trusted in-process extension | No request tenant; successful memory writes use process-global `OHC_ORGANIZATION_ID`, defaulting to `system` | Provider clients and each run attempt have deadlines; output channel is bounded, but producer ignores receiver closure | Instrumentation skips request values | Authentication improved; tenant attribution and cancellation fail requirements |
| Agent manager gRPC (`src/server/services/agent/service.rs`) | Server-wide `spiffe_interceptor` | Header-derived org is used for snapshots, but not all mutations/reads are scoped to it | Spawned tasks have no local timeout | No prompt logging in reviewed methods | Authentication and multiple ownership checks fail |
| Session-memory pipeline (`src/server/workers/agent_memory_pipeline.rs`) | Background system worker | Cross-tenant fetch intentionally clears tenant context; final Postgres insert sets row tenant context, while failure updates use the pool without tenant context | Embedding is limited to 60 seconds; summarization calls have no deadline | Provider errors are logged verbatim | Mixed controls; several high-risk gaps |
| Native business tools (`src/agents/builtin/tools`) | Model-generated tool arguments | Quote and booking schemas expose `tenant_id`; executors trust it | Database connect/query paths have no explicit deadline | Database errors are returned to the model | Confirmed tenant-confusion boundary |

## Confirmed findings

### F-01 — Critical — client-spoofable SPIFFE authentication

**Status (2026-07-12): Remediated.** Commits `629d99030`, `4a4a5e4d3`, and `b7b50ddb8` enforce exact trusted SPIFFE identity structure, extract exactly one validated SPIFFE URI SAN from the verified peer certificate, require server identity/client CA configuration in cloud mode, and ignore caller-supplied identity metadata there. Standalone mode remains loopback-bound and may use its explicit validated metadata path. Certificate regressions reject missing, invalid, ambiguous, and untrusted SANs; strict parser regressions reject empty, encoded, extra-segment, and untrusted identities; cloud regressions require peer-certificate identity and complete mTLS configuration. The later full `server_auth` suite passed 28 tests and its Bazel target passed.

`src/server/lib.rs:418` reads `x-spiffe-id` directly from request metadata and treats successful string parsing as authentication. The Tonic server is not configured with client-certificate verification on this path. `src/server/auth/mod.rs:768` only checks slash positions; it accepts untrusted domains and empty organization/agent components. A remote client can therefore manufacture the identity used by intercepted gRPC services.

Smallest regression: `spiffe_interceptor_rejects_identity_without_verified_peer_certificate`.

### F-02 — High — cross-tenant agent-manager reads and mutations

**Status (2026-07-12): Remediated.** Commit `229b22937` derives one authenticated organization for every agent-manager operation; verifies agent ownership before fire/delegate mutations; scopes identities, skills, snapshots, restore, dashboard agents, costs, meetings, and approvals; and removes the agent-ID-prefix ownership fallback. The regression `cross_org_resources_and_mutations_are_rejected` exercises cross-organization fire/delegation plus identity, skill, snapshot, and restore isolation. The focused `cargo test -p ohc-mono --lib services::agent -- --nocapture` suite passed 7 tests.

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

**Status (2026-07-12): Remediated in code; the worker-specific real Postgres/RLS path remains unverified.** Commits `8be295f0c` and `68b3649c2` add a testable summarization boundary with a 60-second deadline, redact provider error bodies, use explicit system authority for cross-tenant acquisition and filesystem ingestion, and run failure/final mutations under the row's organization. Failure resets require both session and agent IDs. Five focused Cargo tests and `//src/server/workers:server_workers_unit_test` pass. Because `OHC_DATABASE_URL` was unset for that worker test, its Postgres body returned early. F-10 separately verifies the six `server_auth` tenant-isolation bodies; it does not execute this worker-specific path.

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

**Status (2026-07-13): Remediated in the current tree; operational rotation and history remediation remain.** The tracked `src/server/auth/.ohc_jwt_secret` artifact was removed, `.gitignore` now excludes that runtime-secret basename at any directory depth, and `.github/scripts/check_repo_hygiene.sh` rejects any tracked path with that basename. The behavioral regression command `.github/scripts/check_repo_hygiene_test.sh` passes disposable Git fixtures covering the exact root basename, nested and space-containing paths, a newline-containing directory, and allowed prefix/suffix near misses; CI runs it before the clean-tree `.github/scripts/check_repo_hygiene.sh` invocation. Diagnostics shell-escape tracked paths so control characters cannot spoof additional log lines. Before deletion, the checker failed by naming only the forbidden path. After deletion, the clean-tree checker passes, `git check-ignore -v src/server/auth/.ohc_jwt_secret` confirms the path is ignored, and `git ls-files -z | while IFS= read -r -d '' path; do case "$path" in .ohc_jwt_secret|*/.ohc_jwt_secret) printf '%s\n' "$path";; esac; done` produces no paths.

Runtime code constructs the active fallback-secret path with `server_config::get_safe_user_dir().join(".ohc_jwt_secret")`; no source-tree linkage was found, so production use of the removed artifact was not proven. Git history may still contain the removed material. Operators must rotate any potentially related JWT signing keys and tokens and separately decide whether history rewriting is required; neither history purge nor operational rotation was performed as part of this remediation.

### F-10 — Medium — tenant tests silently skip while reporting success

All six `server_auth` multitenancy tests return immediately when `OHC_DATABASE_URL` is unset. In this environment Cargo reported 6 passed in 0.00 seconds even though no Postgres assertions ran. Several Postgres queue and memory tests use similar environment-sensitive setup. CI needs an explicit Postgres lane, and skip conditions must be surfaced as skips or failures in the security lane.

Smallest regression: `multitenancy_suite_requires_postgres_in_ci`.

**Status (2026-07-13): Remediated.** Commits `627b64a87` and `104d87a6a` route all six tests through one policy/setup helper. Follow-up commits `ace8a54c0`, `5162136bd`, `696c635b4`, `17c7959a2`, `5736ab20c`, `8414f9a3c`, `68e06054c`, `19a2119c9`, and `a388c94c9` require exact database outcomes and reject inactive, unreachable, inert, shell-swallowed, shadowed, or metadata/environment-overridden CI checks. Local optional mode prints `SKIPPED postgres security test: ...` once per test, while `OHC_REQUIRE_POSTGRES_TESTS=1` turns a missing or non-Postgres URL into a test-entry failure. Four pure policy tests cover the decision table without mutating process-global environment state. The helper creates `vector` and `uuid-ossp`, runs the embedded `src/server/migrations` migrator once, provisions/grants the application role, connects the test pools through that role, and asserts that the initial login is neither superuser nor `BYPASSRLS`, is `NOINHERIT`, is an explicit member of `ohc_bypassrls`, and has `row_security` on. The connection-leakage pool remains limited to one connection, all setup/query errors fail the security tests, and deterministic token fixtures are cleaned up.

The required `postgres-security` CI job uses `pgvector/pgvector:pg16`, sets required mode, proves the application-role attributes before running the exact active quoted scalar `cargo test -p server_auth multitenancy_isolation:: -- --nocapture`, and is enforced by `ci-required` for every non-markdown change. The contract performs a real YAML parse before its indentation/step-aware semantic checks; this caught and corrected the invalid unquoted colon-space suite scalar. PyYAML is used when available, with Ruby Psych as fallback, and absence of both parsers fails closed. Both parser paths recursively reject duplicate mapping keys and non-scalar keys before semantic extraction, preventing later `env`, `defaults`, or `jobs` mappings from shadowing the graph that was validated. Behavioral regressions cover all three top-level duplicates and a nested duplicate. Additional mutations prove that commented commands/assertions, explicit dead branches and mapping keys, compound and multiline early termination (including `exit 00`), critical step-level `if`, `continue-on-error`, or `shell`, and critical job-level `continue-on-error` or run defaults are rejected rather than counted as coverage. The workflow default must remain exactly `defaults.run.shell: bash`; critical jobs and steps cannot replace it with swallowing templates such as `bash {0} || true`. Canonicalized critical job/step key allowlists treat quoted keys, explicit keys, and whitespace before `:` as their YAML-equivalent names. The top-level workflow environment must contain only `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24`; a disposable Bash regression proves that a `BASH_ENV` containing `exit 0` otherwise preempts a step body. The PostgreSQL job environment and required-result environment must match exact allowlists, while role, suite, and contract steps permit no environment, PATH, working-directory, timeout, or other execution metadata. This prevents inherited or step precedence from changing required mode or clearing the application URL. The `ci-required` job must retain its exact active `always()` condition. No active `exec`, `exit`, or `return` token may precede the application-role assertions, required-result enforcement, or either always-run contract command. The required-result and check-changes scripts must match exact fail-closed allowlists, including the `require_success` condition and failure command. Role assertions count only inside the application URL's heredoc, which must be owned by the exact active `psql` command immediately after the exact admin `psql` heredoc; inert, wrapped, or command-substituted owners are rejected.

The new remote GitHub Actions `postgres-security` job has not yet run, so its remote CI result remains unverified. This does not change the current-code remediation status: the enforcement is implemented and the exact disposable Postgres/application-role flow passed locally.

Local verification used a fresh disposable pgvector PostgreSQL 16 container and the same admin/application-role flow. Before the suite, PostgreSQL reported `session_user=current_user=ohc_security_test`, `rolsuper=false`, `rolinherit=false`, `rolbypassrls=false`, explicit `ohc_bypassrls` membership, and `row_security=on`. The standalone body proved an explicit switch to `current_user=ohc_bypassrls` with `rolbypassrls=true`, then produced the deterministic `user not found` repository result. The cloud IDOR body matched the exact application-validation rejection, and the cross-tenant write body accepted only PostgreSQL error `42501`; arbitrary database/setup errors can no longer produce green tests. The exact required-lane command executed all six bodies: 6 passed, 0 failed in 5.13 seconds. The full `server_auth` Cargo suite passed 28 tests, and `//src/server/auth:server_auth_unit_test` passed under Bazel with the embedded migrations declared as compile data. The container was removed afterward. No repository or production credentials were used or recorded.

### F-11 — Critical — tracked Bazel remote-cache and BES API credentials

**Status (2026-07-13): Remediated in the current tree; external credential response and history remediation remain.** Three tracked literal API-key header assignments were removed from `.bazelrc` without displaying, copying, hashing, or recording their values. All tracked `--announce_rc` enablement was also removed so future local or CI-injected header values are not printed automatically, including under command-specific platform configurations. Default and local builds no longer require tracked credentials. The tracked rc now only try-imports an optional, narrowly ignored `/.bazelrc.local`; a disposable Bazel 9.1.1 probe proved both the present-file and absent-file behavior. Existing CI injection through `bazel-contrib/setup-bazel` and protected GitHub secret expressions remains unchanged.

Repository hygiene now fails closed when any tracked text file contains a literal `--bes_header` or `--remote_header` assignment to an `x-*-api-key` header. Pure GitHub Actions secret and shell environment references remain allowed. The scanner emits only NUL-delimited paths to the shell wrapper, and diagnostics render only shell-escaped paths, never matching lines or values. Disposable Git regressions cover both flags, equals and quoted/spaced assignments, paths containing spaces and newlines, protected GitHub and environment references, and a non-credential near miss. The behavior suite and current-tree checker pass; a non-outputting tracked-file scan reports zero current literal matches. Bazel loaded only the tracked workspace rc during a captured `--announce_rc` probe, and a non-outputting scan of that mode-0600 capture reported zero API-key header occurrences before the capture was deleted.

This repository change cannot invalidate material that may already have escaped. The affected BuildBuddy/Nativelink credentials must be rotated or revoked externally. Git history must be assessed and, if required, purged; remote-cache and BES logs/artifacts must also be assessed for credential exposure. Credential rotation/revocation, history rewriting, and external log/artifact assessment were **not performed** by this remediation.

## Passing and unverified test evidence

| Command | Result | Interpretation |
|---|---|---|
| `OHC_REQUIRE_POSTGRES_TESTS=1 cargo test -p server_auth multitenancy_isolation:: -- --nocapture` with disposable pgvector PostgreSQL and admin/application-role URLs | 6 passed, 0 failed in 5.13s | **Verified:** all six bodies began as `ohc_security_test` with `rolsuper=false`, `rolinherit=false`, `rolbypassrls=false`, explicit bypass-role membership, and `row_security=on`; system context explicitly switched roles |
| `cargo test -p ohc-mono --lib agent_memory_pipeline -- --nocapture` | 3 passed | SQLite and timeout behavior covered; real Postgres isolation remains environment-dependent |
| `cargo test -p ohc-mono --lib services::agent -- --nocapture` | 7 passed | Includes the cross-organization agent/resource/mutation isolation regression |
| `cargo test -p ohc-mono --lib orchestration::queue -- --nocapture` | 17 passed | SQLite behavior covered; Postgres/RLS claims require a configured database to be considered verified |
| `cargo test -p ohc_builtin_agent service -- --nocapture` | 9 passed | Current filtered service suite includes verified-mTLS identity, captured-tenant memory, and receiver-drop cancellation regressions |
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

## Historical remediation sequence — completed in current-tree code

This is the execution order used after the initial audit, not an open to-do list. Remaining operational actions are documented in the individual status blocks.

1. Replace header-trusting SPIFFE interception with verified mTLS peer identity and reject empty/untrusted identities.
2. Scope all agent-manager operations, skills, identities, and snapshots to the authenticated organization.
3. Remove tenant IDs from model-facing tool schemas and inject an authenticated tenant capability into executors.
4. Propagate stream cancellation into producer tasks and replace unbounded event channels.
5. Carry authenticated tenant context through built-in task execution and memory writes.
6. Add deadlines and tenant-scoped transactions to every memory-worker provider/database path.
7. Redact task/error bodies from logs and add telemetry field tests.
8. Upgrade vulnerable JavaScript dependency paths and add enforced advisory scanning.
9. Remove the tracked secret candidate and make Postgres security tests explicit in CI.
10. Remove tracked Bazel remote-cache/BES credentials and enforce secret-free tracked configuration.

## Benchmark and final verification

The deterministic quality anchor is commit `a9803a9fa5b3065d6c063ff49ccb4f10c7ba590a`. The controlled comparison uses exact adjacent production trees: pre-optimization commit `e6fa6cc48d42ddbc33c1db21ab495957a3629a06` and its immediate child, tool-schema optimization commit `56766ff9f34188dabdd2e977de0df27e367bbcd5`. The pre-optimization tree contains the quality anchor plus build-only fixes. Because the runner did not exist in either production tree, detached disposable worktrees received identical benchmark-only instrumentation: the runner source from `5aa771b84` and the unchanged Cargo/Bazel example wiring introduced by `bad6e57a3`. No production source was changed, and the worktrees were removed after measurement.

The runner uses the same deterministic production fixture as the quality regression: one `Lookup` native tool, exactly one `Role::User` message whose content is exactly `What is six times seven?`, one fake-provider response, and the exact answer `The verified answer is 42.` It runs on a Tokio current-thread runtime and performs exactly 20 warmups followed by 200 measured turns. `Instant` brackets only `run_tao_orchestration_loop(...).await`; elapsed time is captured before answer, LLM-call, tool-execution, request, schema, message, or profile validation. Every turn then asserts the answer, complete native tool schema, exact user message, stable request profile, exactly one LLM call, and zero `NeverExecutor` invocations. The runner prints only the requested aggregate JSON object; the zero tool-execution counter is an asserted invariant, not an added JSON field. Each production tree was built in a separate release target directory to prevent cross-worktree Cargo artifact reuse, then executed seven times pinned to CPU 0 with `taskset -c 0`.

Environment: `rustc 1.95.0 (59807616e 2026-04-14)` on x86_64 Linux, AMD Ryzen 7 6800U with Radeon Graphics, 16 online logical CPUs. The measurement commands were equivalent to:

```text
CARGO_TARGET_DIR=/tmp/agent-benchmark-e6fa-target cargo build --release -p ohc_builtin_agent --example agent_path_baseline
taskset -c 0 /tmp/agent-benchmark-e6fa-target/release/examples/agent_path_baseline
CARGO_TARGET_DIR=/tmp/agent-benchmark-56766-target cargo build --release -p ohc_builtin_agent --example agent_path_baseline
taskset -c 0 /tmp/agent-benchmark-56766-target/release/examples/agent_path_baseline
```

The seven run-level `(median_micros, p95_micros)` results, in execution order, were:

- Pre-optimization: `[(50,64),(39,51),(39,52),(50,65),(46,88),(48,81),(47,62)]`
- Optimization child: `[(40,55),(44,58),(42,60),(45,62),(49,64),(40,58),(40,59)]`

Representative exact outputs are the seventh pre-optimization run and third optimized run, whose median values equal the median of their respective seven run-level medians:

```json
{"schema_version":1,"iterations":200,"median_micros":47,"p95_micros":62,"request_profile":{"system_chars":159,"history_chars":24,"tool_result_chars":0,"tool_schema_chars":87,"message_count":1,"tool_count":1,"estimated_input_tokens":68},"llm_calls_per_turn":1.0,"quality_passed":true}
{"schema_version":1,"iterations":200,"median_micros":42,"p95_micros":60,"request_profile":{"system_chars":0,"history_chars":24,"tool_result_chars":0,"tool_schema_chars":87,"message_count":1,"tool_count":1,"estimated_input_tokens":28},"llm_calls_per_turn":1.0,"quality_passed":true}
```

Across all seven pre-optimization runs, run-level medians ranged from 39–50 microseconds and p95 values from 51–88 microseconds. Across all seven optimized-child runs, medians ranged from 40–49 microseconds and p95 values from 55–64 microseconds. These ranges overlap, so this evidence does **not** establish a latency improvement. This is an intentionally deterministic in-process harness, not a provider-network or end-user latency benchmark.

The request-size change is deterministic: the native provider schema remains exactly 87 characters in the `tools` field, while its duplicate textual representation is removed from the system prompt. Attributed input characters fall from 270 to 111, a reduction of 159 characters (58.9%); the documented four-characters-per-token estimate falls from 68 to 28, a reduction of 40 estimated tokens (58.8%). This estimate is attribution telemetry rather than a provider tokenizer or billing count. LLM calls remain exactly 1.0 per turn, native tool executions remain zero, and all 1,400 measured turns per production tree preserved the answer/schema/message invariants.

Focused runner verification includes `rustfmt --edition 2024 --check src/agents/builtin/examples/agent_path_baseline.rs`; `cargo test -p ohc_builtin_agent --example agent_path_baseline` (4 passed); release compilation for both measured production trees; `bazel build //src/agents/builtin:agent_path_baseline`; and `git diff --check`. No lint suppression or unrelated cleanup was added.

### Final verification matrix

The broad matrix was executed after the benchmark review. The combined all-target Cargo command reached one deterministic pre-existing core failure after all 535 built-in-agent unit tests, the 4/4 and 2/2 integration groups, the 1/1 production-path regression, and the 4/4 benchmark tests had passed. Isolated package runs then passed 11/11 LLM tests and 158/158 tool tests. The core failure exposed a real quality defect rather than being waived: fenced JSON text bypassed the documented native-tool-only structured-output recovery path. Commit `ef31a5af3` removed that bypass from both parsers, added direct rejection coverage, and strengthened the retry integration to require the recovered value and exactly two provider calls. The post-fix core suite passed 77/77 plus doc tests, and the affected Bazel unit target passed.

The exact Bazel matrix `bazel test //src/agents/builtin:all //src/agents/builtin/llm:all //src/agents/builtin/tools:all --test_output=errors` passed 10/10 targets in 123.379 seconds; a separate `bazel build //src/agents/builtin:agent_path_baseline` passed. Bazel refreshed the tracked crate-universe fingerprints for the changed built-in and earlier tools manifests; a second build left `MODULE.bazel.lock` stable.

The corrected local auth command `cargo test -p server_auth --lib multitenancy_isolation -- --nocapture` passed 6/6 while printing six explicit optional-local skips because `OHC_DATABASE_URL` was unset. This does not replace the F-10 required-mode evidence: the fresh disposable pgvector run above executed all six bodies through the non-superuser application role and passed 6/6. Repository hygiene behavior/static checks, PostgreSQL CI contract behavior/static checks, and Python compilation passed. Root/UI pnpm production audits reported no known vulnerabilities, and root/UI npm production audits reported zero vulnerabilities. `cargo-audit` remains unavailable; `cargo tree -d` completed and its duplicate dependency families remain maintenance debt.

Two repository-wide quality gates remain nonzero for pre-existing code outside these changes. `cargo fmt --all -- --check` emitted the existing broad formatting diff; the benchmark file passes focused Rustfmt, while the pre-existing `output_parser.rs` formatting backlog prevents a whole-file focused pass even though the parser fix itself is narrowly scoped. The requested all-target Clippy command stopped on six unchanged `server_pricing::cost_aggregator` warnings (`type_complexity` and `collapsible_if`) before it could lint every requested target; earlier focused runs also exposed unrelated existing warnings in prompt construction and stores. These are recorded as remaining cleanup rather than misreported as a green lint gate.
