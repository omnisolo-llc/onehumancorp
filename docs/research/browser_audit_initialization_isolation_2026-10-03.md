# Browser audit initialization and setup isolation, 2026-10-03

## Exact failing evidence

Source: `aa377e53d1fc8bb422f7bd405f53ff5453e685c9`, [CI run 37153128293](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37153128293).

- [Shard 1, job 111292482176](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37153128293/job/111292482176) reported a missing `Refresh analysis history` button in the `/agent-protocol` click baseline. Its recorded final view was still checking the session, execution policy and history, with Refresh disabled.
- [Shard 3, job 111292482190](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37153128293/job/111292482190) reported missing onboarding `Upload Image`. Its click receipt records Log out, Voice Assistant, Skip setup, Help, Back and Open help chat before the failed inventory comparison. The video also shows the final scan during setup initialization. Back and Skip mutate retained setup state; merely reloading the shared owner does not restore the original view.
- That shard's `/share-card` test recorded zero click observations. Its video shows `/share-card` → `/onboarding` → dashboard while resolving the first Log out target. The onboarding source automatically redirects when the loaded state is skipped. The shared-owner setup mutations and delayed restore explain why accepting the first URL alone did not make that document stable.

These three failures are distinct from the older finite-response-body failures addressed in `browser_ci_response_completion_2026-10-03.md`. No known body fix is duplicated here.

## Repairs

### Truthful readiness

Onboarding exposes its actual restoring state through `aria-busy`. Recorded analysis separately tracks owner/policy verification and history loading. Failed or revoked verification settles readiness and retains unavailable/disabled controls; it is not represented as perpetual loading or configured runtime success. Existing owner-generation checks prevent an older response from settling a new owner's state.

### Fresh factual setup cases

The existing case-owned PostgreSQL fixture now covers `/onboarding` and `/share-card`. Each observed control receives a new signed owner, canonical database graph, cookie jar and browser storage. The fixture proves that new owner's onboarding table is empty before navigation, then uses the actual backend's step-0 conversational default. `Upload Image` remains explicitly required in the initial view.

Additional view-qualified inventories cover intro, instant draft and manual draft. They are prepared through the real Back/selection UI and acknowledged HTTP 204 state writes. They do not submit provider-generation requests, invent business outputs, mock endpoints or rewrite application responses. The original complete-inventory comparison and same-document target checks remain unchanged.

Two new real-stack regressions exercise Back and Skip on both routes, assert the actual persisted `state_json` values, then prove another fresh owner still reaches onboarding with the same Upload Image inventory. These browser/PostgreSQL regressions are pending execution in fresh CI, not claimed as locally passed.

### Preparation must finish before observation

Two autosave races were reproduced: an already-acknowledged explicit save could still trigger its debounce before React cleanup, and a slow explicit save could admit another save behind its origin lock. A first HTTP 204 alone therefore did not establish readiness.

The existing draft-write gate now tracks per-owner admitted writes before lock acquisition and through final settlement, including queued writes. The setup surface stays busy until all such work settles. A held/unknown write remains visibly held while busy becomes false; debounce cannot readmit that held operation. Later unsaved edits retain their own acknowledgement path. The fixture waits for this truthful settled state and rejects transport-completion errors before beginning click observation.

## Verification and limits

- Production readiness/write lifecycle regressions demonstrated RED→GREEN; final focused production run: 111/111 tests across seven suites, 8.67 seconds.
- Fixture/navigation/target-identity suites: 42/42 tests across three files. All eight new fixture tests failed before implementation or the follow-up readiness repair.
- Final unchanged-source complete Next suite: **3,416/3,416 tests in 496 files**, 211.14 seconds, exit 0 (`npm --prefix src/ui/next test -- --maxWorkers=4`). This count is for this patch on aa377, not the separately frozen response/decision patches.
- Root scripts: **950/950**, zero skips, 68.62 seconds, exit 0. Final web TypeScript, E2E TypeScript, repository ESLint and `git diff --check` passed.
- Complete Playwright discovery: **1,805 tests in 461 files**, adding the two real-stack regressions without dropping existing discovery.
- Independent read-only review approved the final source after the delayed-write readiness repair, with no assertion weakening or dropped controls found.
- Source/test fingerprint excluding this document: SHA-256 `3054169a5426f7126d12ff19b2755248d457f856af79c17025b870e321e9e520` over sorted path-to-SHA256 JSON. The source remained unchanged through final validation.
- No `ui-click-audit.cjs` acceptance requirement, inventory equality, target-document check, test exclusion or runtime prerequisite was relaxed.
- The 30-minute required-CI timing threshold is unchanged. The observed run exceeded it; only a subsequent actual run can establish whether the repaired waits and expanded cases meet it.

Fresh source-bound browser CI and final click receipts remain required. Configured provider/runtime/text-analysis positive acceptance remains OPEN. No provider setup, runtime success or delivery is inferred from these UI/harness checks.
