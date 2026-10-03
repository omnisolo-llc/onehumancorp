# Recorded text analysis in Agent Protocol

The Agent Protocol page now includes a separate **Text analysis only** panel. It reuses the mounted, authenticated workflow receipt endpoints and funded admission path. It does not enable the legacy workspace runtime or establish actor, expert-team, swarm, Git, repository-map, visual-workflow, or 999-agent execution capability.

## Data and authority

- `GET /api/v1/agents/execution-policy` describes the configured supplied-text provider/model and explicitly disallows tools/workspace access. Unconfigured policy disables submission but does not fabricate an empty receipt history.
- `GET /api/v1/agents/workflows` and its bounded cursor pages retain their existing tenant-wide semantics. The personal panel validates every receipt’s tenant, then filters by current actor; it preserves the server cursor across teammate-only pages. `GET /api/v1/agents/workflows/:id` supplies status and output. Current tenant and actor must both match before rendering a record; malformed, foreign, contradictory, or output-free completed receipts are rejected.
- Explicit **Start text analysis** uses the existing `useTenantAnalysis` owner session, cross-tab lock, durable local request marker, idempotency key, and `POST /api/v1/agents/hire`. Existing backend authority, provider selection and hard usage admission remain authoritative. A queued receipt is not a completion.
- A lost submission acknowledgement remains held across reload. Only authenticated request-ID/receipt readback can recover acceptance; reads never trigger another model turn.
- Explicit **Cancel text analysis** calls the existing `POST /api/v1/agents/workflows/:id/cancel` with current owner preconditions and a bounded request. A mutation acknowledgement never invents cancellation: the panel reads the actual durable status. Failed cancellation holds further clicks until an explicit status refresh. A running call can become `outcome_unknown`; cancellation is not a refund or proof that no provider work occurred.
- Pending selected receipts are polled through the existing authenticated read path. Account/session invalidation clears private panel state and fences late read, submit and cancellation responses.

The existing Agent Protocol methods and controls remain in a separate **Workspace runtime** section. Its task history must be loaded explicitly. Unavailable runtime history is not reported as an empty task list. Legacy runtime implementation and workspace authority gaps remain open; this text-only panel does not grant them access or satisfy their required real-runtime journeys.

## Verification and release ownership

Receipt-driven UI tests use actual HTTP `Response` envelopes at the transport boundary, not canned production implementations. They exercise real mounted React components, the current owner session and the existing submission hook, including provider-unavailable state, actual output, owner changes, malformed data, hard-budget rejection, duplicate clicks, lost acknowledgement recovery, cancellation and status readback. These tests do not prove external provider execution.

`src/e2e/recorded_text_analysis.spec.ts` adds ordinary exact-policy/history verification with no dispatch and retains a separate real-provider acceptance journey. The release owner must run that acceptance against an isolated seeded stack built from the exact source, with an authorized configured provider and genuine tariff/budget. It must produce and persist real provider output and read it back after reload. No fake provider key, fixed-response service, mocked network, production target, or implicit spending authority may satisfy it.

Use the repository's real-stack setup in [generation-acceptance.md](generation-acceptance.md), then select this gate only after its prerequisites are authorized:

```sh
PLAYWRIGHT_TEST_DIR=./src OMNISOLO_E2E_TEXT_ANALYSIS_ACCEPTANCE=1 \
  node node_modules/@playwright/test/cli.js test --config playwright.config.ts \
  --grep @text-analysis-acceptance --workers=1 --retries=0
```

The ordinary native runner must continue stripping provider/runtime credentials. The selector does not configure a provider, change a budget, grant a workspace, or authorize paid calls. This repair has not executed the configured-provider gate or the outstanding workspace/runtime journeys. Full repository `make lint` / `make test` and exact-source hosted browser execution remain required acceptance gates.

## Local checkpoint evidence, 2026-10-03

On the final recorded source fingerprint with pinned Node 22.22.1:

- `npm --prefix src/ui/next test -- --maxWorkers=1`: **484 files / 3,237 tests passed**, 534.78 seconds. The earlier 3,232-test run overlapped a recovery repair and is not final-source certification.
- `node --test --test-concurrency=1 scripts/*.test.mjs`: **827/827 passed**, 154.33 seconds.
- `npm run test:contracts`: **20/20 passed**.
- Web TypeScript, browser TypeScript and changed-file ESLint passed. Both new Playwright journeys were discovered; discovery is not execution.
- The focused owner/receipt/runtime compatibility group passed **87 tests**. New negative tests first reproduced missing personal-history handling, mixed-actor pagination rejection, contradictory response acceptance and foreign-tenant lost-ACK recovery, before the corresponding repairs passed.

No source fingerprint changed during the final full Next run. No Cargo/PG gate, actual model call, browser stack, deployment or workspace runtime was started for this UI checkpoint. The release requirements above remain open.
