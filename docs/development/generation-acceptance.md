# Generation and repository-runtime acceptance

The default native browser suite deliberately starts without model/payment credentials or a legacy agent runtime. It must verify the exact missing prerequisite and retained owner inputs. This is **not successful generation or completed runtime support**.

## Current implementation and remaining capability gaps

- The builder and Brand Studio use the genuine, bounded text-generation path in `src/server/builder/generation.rs`. Admission binds the configured operator tenant and provider to current owner authority and the usage funding policy. Generated content remains an unverified proposal.
- Builder rendering must preserve returned `HeroBlock.subtitle` and `TextBlock.text`. Both editors validate generated fields before replacing their owned local draft. A provider failure retains the brief, existing blocks, and pending edit request; an unknown outcome never triggers an automatic retry.
- Brand Studio currently supports supplied-text brand/copy suggestions and a persisted toolbox. Website/product URL fetching, catalog creation, generated logos/photos, uploaded-media analysis, and GEO scoring remain unavailable. Disabled URL fields are a visible boundary, not acceptance evidence for those features.
- Public website acceptance requires explicit review, a recorded publication receipt, and anonymous readback of the actual application-hosted path. Neither a private save nor a made-up `cloud.omnisolo.co` domain proves publication.
- The legacy RepoMap implementation is source-present, but the main server's raw `/rpc` is contained because its global runtime lacks tenant-bound workspace authority. `/api/v1/rpc` requires a separately configured approved runtime. An absent runtime must show its actual prerequisite, never a repository map or generic “Unknown error.” Rendering a confirmed RPC result is not proof that a safe tenant runtime exists.

## Separately owned gates

The release owner is responsible for provisioning and running these gates, recording the exact source commit, provider/runtime configuration, authority/budget conditions, counts, and outcomes. A skipped or unexecuted gate is an outstanding release prerequisite. Passing ordinary unconfigured tests must never be reported as completion of these journeys.

| Gate | Browser journeys retained | Required evidence |
| --- | --- | --- |
| `@provider-acceptance` | Agentic storefront generation and edit; builder wizard selection; Brand toolbox persistence and reviewed publication; desktop/mobile publication; narrative proposal draft; text-analysis workflow admission | Real provider output for the authenticated tenant, actual persisted Brand/workflow readback, reviewed publication receipt and anonymous HTTP readback. Queued workflow admission is not completed analysis. |
| `@runtime-acceptance` | Aider RepoMap; Actor Model; LangGraph tools; expert synthesis; scalable agent outputs; computational success/failure; SONA reads and recording; provider-backed visual workflow approval | Approved tenant-bound runtime returns genuine outputs through the mounted gateway, real shell/tool execution and SONA readback. Visual workflow additionally needs its configured local model. No legacy global runtime or fixed-response provider may satisfy these cases. |

### Ordinary unconfigured verification

Use the normal native build/test workflow in [native-build.md](native-build.md). No provider or runtime credentials are needed. The generation/runtime availability tests require exact HTTP 503 responses and expected error codes/messages; arbitrary authorization, validation, transport or database failures do not count as passing availability checks. The configured acceptance tests are explicitly skipped with their outstanding prerequisite visible in the report.

### Configured acceptance

The regular `scripts/native-e2e.mjs` runner intentionally removes provider/runtime credentials. Do not weaken that boundary or add credentials to CI merely to change the pass count.

Run the tagged tests directly only against a separately provisioned, isolated, seeded test stack built from the exact current source. Set `BASE_URL`, `DATABASE_URL`, and `PLAYWRIGHT_STORAGE_STATE` to that test stack. The global setup requires those values and uses the actual seeded users. This is not authorization to point tests at production or to spend money.

For text generation, configure the backend's actual supported provider/model, `OMNISOLO_BUILDER_TENANT_ID`, current owner membership, `OMNISOLO_USAGE_PAYER`, and the applicable genuine tariff/usage budget. The configured operator tenant must match `OMNISOLO_E2E_ADMIN_ORGANIZATION_ID` (the seeded admin tenant by default). Managed API usage additionally needs its supported rate card, request ceiling, and explicit spending authority. No dummy provider keys, fixed-response service, mock success, or fabricated model output may satisfy this gate.

```sh
PLAYWRIGHT_TEST_DIR=./src OMNISOLO_E2E_GENERATION_ACCEPTANCE=1 \
  node node_modules/@playwright/test/cli.js test --config playwright.config.ts \
  --grep @provider-acceptance --workers=1 --retries=0
```

For runtime acceptance, first implement/provision and review the tenant-bound repository runtime with a bounded workspace grant for the seeded unlimited admin tenant. Configure `OMNISOLO_AGENT_URL` and any approved runtime authentication on the test backend. Do not expose the unscoped legacy AppServer as a workaround.

```sh
PLAYWRIGHT_TEST_DIR=./src OMNISOLO_E2E_RUNTIME_ACCEPTANCE=1 \
  node node_modules/@playwright/test/cli.js test --config playwright.config.ts \
  --grep @runtime-acceptance --workers=1 --retries=0
```

These switches select acceptance tests only. They do not create credentials, authorize external requests, fund generation, grant filesystem access, or certify missing capabilities. No paid provider call or configured runtime acceptance was performed for the UI/contract repair.

## Repair verification, 2026-10-03

Using the pinned Node 22.22.1 runtime on the final UI source:

- `npm --prefix src/ui/next test -- --maxWorkers=1`: 476 files, 3,114 tests passed in 519.94 seconds.
- `node --test --test-concurrency=1 scripts/*.test.mjs`: 819 passed in 143.11 seconds, including 18 executed legacy-Aider DOM checks.
- `npm run test:contracts`: 20/20 passed. An earlier local checkout-layout failure was resolved by replacing only the disposable worktree's top-level dependency-directory symlink with a directory of links; no repository contract was weakened.
- Web TypeScript, browser TypeScript, changed-file ESLint, and all 12 affected Playwright test discovery checks passed.
- Real-stack browser execution, full `make lint`/`make test`, the five configured-provider journeys and the configured-runtime journey remain unexecuted for this checkpoint. No native compilation or external generation was claimed by the frontend test results.

Negative regressions were run before their repairs: provider failures, contradictory/empty/malformed generated layouts, array-valued text properties, absent catalog items, dropped real subtitle/text content, lost agent edit input, and invalid Aider HTTP/RPC envelopes all failed as expected. Production repairs then passed those checks.


## Current runtime and browser-contract repair, 2026-10-03

Run `37108893789` exposed assertions that still depended on fabricated legacy runtime output even though the native runner now intentionally starts without an approved runtime. This repair retains **13 configured acceptance cases** in addition to the existing six: three Actor cases, two SONA cases, two computational verification cases, and one each for Scaling, LangGraph, expert synthesis, visual workflow, narrative proposal generation and text-analysis workflow admission. These are **required open product gates**, not completed features or success inferred from a 503. Agent Protocol's independent repair also retains its real task and checkpoint journeys.

Ordinary unconfigured browser cases now verify the exact runtime prerequisite, retained input, recovered controls and absence of fabricated outputs. Scaling must not count an error as one agent result. SONA must show read errors rather than an invented empty store, and a successful POST is insufficient: the exact submitted pattern must be returned by the runtime before it appears in the UI or the inputs clear. The page crawl classifies only the exact same-origin GET `/api/v1/sona` 503 body and requires the corresponding visible error; unrelated statuses, routes, methods and response bodies still fail.

### Source-specific prerequisites and product gaps

- `src/server/lib.rs::proxy_agent_rpc_handler` needs `OMNISOLO_AGENT_URL` for a reviewed tenant-bound runtime. Multitenant runtime dispatch is explicitly unavailable. The server's legacy raw `/rpc` remains contained because it has global state without tenant-bound workspace authority. Pointing the gateway back to that endpoint is not a valid repair.
- `src/server/api/proposals.rs::draft_narrative` is mounted and calls the genuine local model adapter. The default local endpoint is `http://127.0.0.1:11434/api/generate`, model `llama3`, and requires actual usage counters. The native runner has no local model process. Its default browser case only proves handling of the current empty 502 response, not why the provider failed or successful generation. A source-specific prerequisite response remains a product gap.
- `src/server/lib.rs` mounts the visual workflow with hardcoded `http://localhost:11434` Ollama. A real Input→HumanInLoop graph can exercise the approval boundary without a model. The original Input→LLM→HumanInLoop journey remains required and unverified until a real authorized local model is configured.
- The Agents workflow form submits supported text analysis through `/api/v1/agents/hire`. It cannot claim a repository review, a CLI command, or `ohc_review_branch` from that admission. Configured acceptance requires its genuine queued receipt and persisted supplied task; execution completion needs separate evidence.
- Storefront product URLs under `/api/v1/storefront` are owner-only private previews. Anonymous positive SEO checks use the actual reviewed snapshot published at `/api/v1/public/sites/{id}`. A private price change must update private SEO without changing previously approved public bytes; an explicit fresh publication is required for a new public price. Both surfaces intentionally use no-store policies instead of public mutable-draft CDN caching.

No paid provider, local model service or unsafe legacy runtime was provisioned by this repair. On the isolated browser worktree, 19 total configured cases were discovered, not executed; the normal browser runner stopped at missing `target/debug/server`. Fresh real-stack execution and full repository acceptance remain outstanding until the current-source server/web artifacts and Docker prerequisites are available.
