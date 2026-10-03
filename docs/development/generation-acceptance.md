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
| `@provider-acceptance` | Agentic storefront generation and edit; builder wizard selection; Brand toolbox persistence and reviewed publication; desktop builder publication; mobile editing and publication | Real provider output and provenance for the authenticated operator tenant, actual persisted Brand readback, reviewed publication receipt and anonymous HTTP readback |
| `@runtime-acceptance` | Aider RepoMap of `src/agents/builtin` | Approved tenant-bound workspace runtime returns the actual `agent.rs` and `aider_repomap.rs` map through the mounted authenticated gateway |

### Ordinary unconfigured verification

Use the normal native build/test workflow in [native-build.md](native-build.md). No provider or runtime credentials are needed. The six availability tests require exact HTTP 503 responses and expected error codes/messages; arbitrary authorization, validation, transport or database failures do not count as passing availability checks. The configured acceptance tests are explicitly skipped with their outstanding prerequisite visible in the report.

### Configured acceptance

The regular `scripts/native-e2e.mjs` runner intentionally removes provider/runtime credentials. Do not weaken that boundary or add credentials to CI merely to change the pass count.

Run the tagged tests directly only against a separately provisioned, isolated, seeded test stack built from the exact current source. Set `BASE_URL`, `DATABASE_URL`, and `PLAYWRIGHT_STORAGE_STATE` to that test stack. The global setup requires those values and uses the actual seeded users. This is not authorization to point tests at production or to spend money.

For text generation, configure the backend's actual supported provider/model, `OMNISOLO_BUILDER_TENANT_ID`, current owner membership, `OMNISOLO_USAGE_PAYER`, and the applicable genuine tariff/usage budget. The configured operator tenant must match `OMNISOLO_E2E_ADMIN_ORGANIZATION_ID` (the seeded admin tenant by default). Managed API usage additionally needs its supported rate card, request ceiling, and explicit spending authority. No dummy provider keys, fixed-response service, mock success, or fabricated model output may satisfy this gate.

```sh
PLAYWRIGHT_TEST_DIR=./src OMNISOLO_E2E_GENERATION_ACCEPTANCE=1 \
  node node_modules/@playwright/test/cli.js test --config playwright.config.ts \
  --grep @provider-acceptance --workers=1 --retries=0
```

For RepoMap, first implement/provision and review the tenant-bound repository runtime with a bounded workspace grant for the seeded unlimited admin tenant. Configure `OMNISOLO_AGENT_URL` and any approved runtime authentication on the test backend. Do not expose the unscoped legacy AppServer as a workaround.

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
