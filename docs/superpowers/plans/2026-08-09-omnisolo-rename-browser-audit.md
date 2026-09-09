# OmniSolo Rename and Browser Audit Implementation Plan

**Date:** 2026-08-09
**Design:** `docs/superpowers/specs/2026-08-09-omnisolo-rename-browser-audit-design.md`

## Objective

Rename first-party OHC/One Human Corp branding and identifiers to OmniSolo in the mono repository and `/home/kevin/myk3s`, manually exercise every Next.js route and existing Playwright workflow in Chromium, and give every reproducible repository-owned browser defect a unit test plus a Playwright regression test. Do not commit, push, or mutate the live cluster.

## Compatibility rules

- `OHC`, `One Human Corp`, and `OneHumanCorp` are removed from first-party human-facing text, package/module/chart/file/binary/Bazel/release identifiers, docs, fixtures, and origin references.
- `OHC_*` configuration names become `OMNISOLO_*`; update producers, consumers, deployment values, tests, and documentation together. Do not add an OHC alias.
- API/protobuf package names, wire fields, database/table migrations, metrics, SPIFFE identities, vendor names, hashes, and other externally-owned protocol/storage identities are retained when renaming would change a compatibility contract. Record each retained family in the residue audit.
- The canonical cloud origin is `https://cloud.omnisolo.co`; local/test origins remain environment-selected.
- Existing stateful Kubernetes resource names are preserved when a rename would recreate storage. New product-facing labels and chart/release identifiers use OmniSolo.
- The pre-existing modification in `/home/kevin/myk3s/tests/verify-onehumancorp-live.sh` must remain untouched.

## Work sequence

### 1. Establish the contract with failing tests first

- Add a tracked residue-audit test/script that scans only first-party source, deployment, docs, fixtures, and release files, with a small explicit allowlist for retained protocol/storage/vendor identifiers.
- Add unit tests for the current browser-audit route inventory and for canonical origin/branding helpers before changing their implementation.
- Add focused unit tests for the known UI defects from the initial source audit: referral/embed content uses the OmniSolo cloud origin and all visible `Powered by`/help/setup labels use OmniSolo.
- Add chart contract tests that initially require the OmniSolo chart/release names, image/origin defaults, and stateful-resource preservation.

### 2. Rename first-party mono identifiers and contracts

- Rename first-party Rust package/crate/module/directory identifiers, Bazel/Gazelle prefixes and targets, release/package metadata, scripts, and Helm chart paths using path-aware moves.
- Rename Go module/build metadata and first-party repository links to the OmniSolo owner/repository form while leaving generated protobuf Go package paths unchanged where they are API compatibility contracts.
- Rename `OHC_*` configuration constants, environment lookups, deployment values, test fixtures, and scripts to `OMNISOLO_*` without changing unrelated `ohc` protocol, metric, SPIFFE, or database identities.
- Update all first-party imports, generated-code inputs, lockfiles, CI, Docker/compose files, docs, dashboards, and tests; use compiler/build failures and the residue test to find missed consumers.

### 3. Rename the GitOps deployment surface

- Rename the first-party chart/application directory in `/home/kevin/myk3s` to its OmniSolo equivalent and update GitOps references, chart metadata, helper names, values, image repositories, and canonical origins.
- Preserve stateful resource names and existing secret/storage ownership; validate this with rendered manifests and a focused shell/unit contract test.
- Keep the unrelated live-verification script diff intact and do not run commands that apply to the cluster.

### 4. Fix and test browser surfaces

- Update the affected Next.js UI copy, page metadata, generated embed snippets, referral URLs, and browser-audit terminology to OmniSolo/cloud.omnisolo.co.
- Add a Vitest test and a Playwright test for each issue found during the manual audit. Tests must assert accessible, user-visible behavior and be written red before the corresponding fix.
- Extend the existing visual audit runner only where required to cover public login/health, all discovered page routes at desktop and mobile viewports, console exceptions, failed requests, unexpected 4xx/5xx responses, shell presence, and horizontal overflow.
- Run local production `next build`/`next start` with a temporary test session, then run the route audit and every existing Playwright workflow in batches. Run read-only cloud smoke checks against `https://cloud.omnisolo.co` without exposing credentials.

### 5. Verify and hand off

- Run focused unit tests after each lane, then Rust, UI, Playwright, chart-render, shell-contract, and residue suites.
- Re-run the browser audit after every fix and retain its route/viewport report and screenshots outside the repository unless a tracked artifact is required.
- Inspect both worktrees for unintended changes, confirm the cluster’s unrelated modification is preserved, and report exact remaining classified legacy identifiers and any cloud/auth limitations.
- Do not commit or push.

## Verification commands

From the mono worktree:

```bash
python3 scripts/omnisolo_branding_contract_test.py
cargo test --workspace --lib
npm --prefix src/ui/next test -- --run
npm --prefix src/ui/next run build
npm --prefix src/ui/next exec playwright test
```

From `/home/kevin/myk3s`:

```bash
./tests/test-omnisolo-chart.sh
helm template omnisolo ./apps/omnisolo --namespace omnisolo
```

The exact chart/test commands may be adjusted to the repositories’ existing scripts after the path migration is applied.
