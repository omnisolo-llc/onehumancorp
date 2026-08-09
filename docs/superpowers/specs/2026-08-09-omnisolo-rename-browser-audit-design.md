# OmniSolo Rename and Browser Audit Design

**Date:** 2026-08-09

## Goal

Rename the first-party OHC/One Human Corp product surface to OmniSolo across the mono repository and its GitOps deployment configuration, then manually verify the deployed and local browser feature surfaces and prevent every reproducible defect found during that audit with both a unit test and a Playwright end-to-end test.

## Scope

The canonical product name is `OmniSolo`. First-party names are migrated as follows:

- Human-facing `OHC`, `One Human Corp`, and `OneHumanCorp` become `OmniSolo`.
- Configuration prefixes `OHC_*` become `OMNISOLO_*`.
- First-party lowercase package, crate, module, file, chart, binary, Bazel target, and release identifiers using `ohc` or `onehumancorp` become `omnisolo` equivalents.
- First-party UI copy, page titles, generated embed snippets, test fixtures, documentation, and origin references use OmniSolo and `cloud.omnisolo.co` where a cloud origin is required.
- External vendor names, third-party image repositories, cryptographic hashes, and unrelated text containing the same letters remain unchanged.

The application will not accept OHC configuration aliases after the migration. Stateful Kubernetes and database identities are migration-sensitive: they must not be deleted or recreated solely to change a name. If an identity cannot be safely migrated in place, its preservation is documented as an operational storage identity rather than treated as a product-brand alias.

The browser audit covers every Next.js page route, public/authenticated/embed/API-docs surface, and every existing Playwright workflow in `src/ui/next` and `src/e2e`. Cloud checks are read-only unless safe credentials and a test tenant are already available. State-changing workflows run against the local fixture/mock or test environment.

## Architecture

The work is organized into four bounded lanes:

1. **Rename inventory and contracts.** Scan tracked source, generated configuration, release packaging, tests, docs, and GitOps files. Classify matches as first-party contract, stateful deployment identity, or external data. Apply path-aware renames and update all consumers. Add a forbidden-residue contract test for first-party OHC names.
2. **Browser audit.** Build a route inventory from `src/ui/next/src/app/**/page.tsx`, group it by access and interaction type, and exercise it with a real Chromium browser. Record navigation status, visible error states, console errors, failed requests, and the primary user action. Existing feature specs remain the source of truth for multi-step workflows.
3. **Issue-to-test loop.** Each reproducible issue is given a focused unit test for the rendering or logic contract and a focused Playwright test for the visible regression. The test is written and observed failing before the fix, then rerun after the fix.
4. **Deployment verification.** Render the mono Helm chart and the `myk3s/apps/onehumancorp` chart/configuration, validate image and origin contracts, and run the cluster repository’s chart/live verification scripts. Local files may be updated; no Git push or live cluster mutation is part of this design.

The end-to-end flow is:

```text
canonical source names
        |
build artifacts and images
        |
Helm render + GitOps values
        |
cloud.omnisolo.co and local browser harness
        |
browser audit findings
        |
unit + Playwright regression tests
        |
focused fixes and full verification
```

## Naming and deployment boundaries

The mono repository owns application code, build metadata, Helm templates, release artifacts, CI contracts, and product documentation. The `myk3s` repository owns the deployed GitOps chart and live verification scripts. The cluster repository’s existing unrelated modification to `tests/verify-onehumancorp-live.sh` must be preserved.

The rename must update first-party image/origin references and environment/config contracts in both repositories. It must not silently change persistent database contents, secret material, or storage ownership. Helm output must prove that existing stateful resources remain addressable and that the application receives the new canonical configuration names. Any preserved stateful identifier must be called out in the final report.

No generated dependency lockfile entry, third-party URL, vendor product name, hash, or protocol field is changed merely because it contains `ohc` as a substring. Product-owned API/config names are changed only with all local producers, consumers, fixtures, and tests updated in the same change.

## Browser audit behavior

The audit runner will use the configured Playwright Chromium executable and a base URL selected by environment. It will first inspect the public login and health surfaces, then use a supplied storage state or existing test login flow for authenticated routes. It will never print credentials or persist session material in the repository.

For each route or workflow, the audit records:

- HTTP/navigation result and final URL.
- Visible page title, primary heading, and error/empty-state content.
- Browser console errors and uncaught page exceptions.
- Failed network requests and unexpected 4xx/5xx responses.
- Completion of the route’s key user action, where a safe fixture exists.

Expected authentication redirects, intentionally unavailable optional integrations, and known infrastructure limitations are classified rather than reported as product defects. A defect is actionable only when it is reproducible and attributable to repository code or repository-owned deployment wiring.

## Testing strategy

Rename coverage includes:

- Repository residue and naming-contract tests.
- Rust unit/build checks for renamed crate/module/config symbols.
- UI Vitest tests for renamed labels, titles, generated content, and changed configuration helpers.
- Helm render and shell contract tests for new names, images, origins, secrets, and stateful-resource safety.
- Existing full Playwright suites plus new tests for each browser-found defect.

Every browser-found defect must have both test layers unless the defect is purely infrastructure-owned; infrastructure-only findings are reported with evidence and are not disguised as application fixes. Tests should use accessible roles and stable behavior contracts rather than implementation-specific selectors.

## Acceptance criteria

The work is complete when all of the following are true:

1. First-party tracked code and deployment contracts use OmniSolo naming, with only explicitly documented stateful identities or external/vendor references remaining.
2. The application uses `OMNISOLO_*` configuration names without OHC aliases.
3. Mono and GitOps Helm renders pass their contract checks without destructive state changes.
4. The browser audit has exercised every discovered route and existing workflow, with coverage and limitations recorded.
5. Every reproducible repository-owned issue found by the audit has a corresponding unit test and Playwright regression test, and those tests pass.
6. Focused and full relevant test suites pass, or every remaining failure is classified with exact evidence.
7. The final report distinguishes local fixes, live verification, and any deployment action that was not performed.

## Risks and mitigations

- **State loss from resource renaming:** preserve stateful identities and validate Helm output before applying any change.
- **Hidden OHC consumers:** use repository-wide searches, compiler/build checks, lockfile-aware scans, and runtime smoke tests.
- **Large route surface:** generate the route inventory and run it in batches while retaining per-route evidence.
- **Authenticated cloud coverage:** use existing safe credentials/storage state if present; otherwise use read-only public checks and local fixture coverage, reporting the limitation.
- **Pre-existing failures:** capture the baseline before edits and classify unchanged failures separately from regressions.
