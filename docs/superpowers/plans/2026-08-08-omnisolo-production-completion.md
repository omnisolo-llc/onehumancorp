# OmniSolo production feature completion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task with review checkpoints.

**Goal:** Repair every production failure found by browser verification, remove fabricated responses, add regression coverage, rename product branding to OmniSolo, publish the application from the OmniSolo repository, and prove the live HeatWave deployment through a real browser session.

**Architecture:** Keep SeaORM/HeatWave as the canonical persistence layer. Add one explicit legacy `Arc<DB>` compatibility layer at the Rust router boundary, make the Next authenticated transport the only browser-to-backend proxy, and return typed disabled/unavailable state for optional integrations instead of 5xx or fake data. Keep legacy environment/API aliases only at compatibility boundaries.

**Tech Stack:** Rust/Axum, SeaORM, SQLx compatibility handlers, Next.js, TypeScript, Playwright, Vitest, Bazel, Helm/Flux, OCI HeatWave.

---

### Task 1: Freeze the live failure inventory in automated contracts

**Files:**
- Create: `src/server/production_feature_contract_test.rs`
- Modify: `src/server/BUILD.bazel`
- Create: `src/ui/next/src/e2e/production_feature_smoke.spec.ts`
- Modify: `playwright.config.ts`
- Test: `src/server/production_feature_contract_test.rs`, `src/ui/next/src/e2e/production_feature_smoke.spec.ts`

- [ ] **Step 1: Write failing Rust contracts** for the missing DB extension, placeholder catalog output, canonical aliases, and typed disabled status. The test must assert the source/router contract rather than seed business rows:

```rust
#[test]
fn production_router_injects_legacy_db_for_docs_and_legacy_handlers() {
    let source = std::fs::read_to_string("src/server/lib.rs").unwrap();
    assert!(source.contains("Extension(std::sync::Arc::new(db.clone()))"));
    assert!(source.contains("/api/v1/help"));
    assert!(source.contains("/api/v1/tooltips"));
    assert!(source.contains("/api/v1/videos"));
}

#[test]
fn catalog_generation_has_no_fabricated_success_values() {
    let source = std::fs::read_to_string("src/server/api/catalog.rs").unwrap();
    assert!(!source.contains("Generated Offering"));
    assert!(!source.contains("AI description"));
}
```

- [ ] **Step 2: Run the focused Rust target and confirm RED** because the current router/fallback violates both assertions:

```bash
bazel test //src/server:production_feature_contract_test --test_output=errors
```

- [ ] **Step 3: Add the Playwright test skeleton** with real login through `/login`, no mocked network, and a route inventory loaded from `src/ui/next/src/app`. Assert that a page response is below 400 and record every application response at or above 400 for the next tasks to eliminate.

- [ ] **Step 4: Run the browser test against the current cluster and save its failure list**; this is the baseline artifact used to prove each later fix.

- [ ] **Step 5: Commit the red contracts and baseline harness**:

```bash
git add src/server/production_feature_contract_test.rs src/server/BUILD.bazel src/ui/next/src/e2e/production_feature_smoke.spec.ts playwright.config.ts
git commit -m "test: freeze production feature failure inventory"
```

### Task 2: Repair legacy database extension wiring

**Files:**
- Modify: `src/server/lib.rs:7740-7795`
- Modify: `src/server/api/docs.rs`
- Create: `src/server/api/docs_runtime_test.rs`
- Modify: `src/server/BUILD.bazel`
- Test: `src/server/api/docs_runtime_test.rs`, `src/server/production_feature_contract_test.rs`

- [ ] **Step 1: Add a failing handler-level test** that builds the docs router with a real SQLite test database and verifies `/api/v1/help`, `/api/v1/tooltips`, and `/api/v1/videos` return JSON rather than an Axum missing-extension error.
- [ ] **Step 2: Run the focused test and confirm RED** with the exact missing `Arc<DB>` extension error seen in production.
- [ ] **Step 3: Build one `Arc<DB>` compatibility value from the selected runtime database and attach it to the complete legacy route group before the route-level auth layers. Do not create a second database, SQLite fallback, or mock store.**
- [ ] **Step 4: Run the focused unit target and the production source contract; confirm GREEN.**
- [ ] **Step 5: Run authenticated live GETs for `/api/v1/help`, `/api/v1/tooltips`, `/api/v1/videos`, and `/api/v1/walkthrough/dashboard`; confirm JSON 2xx responses.**
- [ ] **Step 6: Commit:** `git commit -am "fix: wire legacy handlers to selected database"`.

### Task 3: Remove fabricated AI/catalog data and stabilize provider states

**Files:**
- Modify: `src/server/api/catalog.rs:620-700`
- Modify: `src/server/api/assistant.rs`
- Modify: `src/ui/next/src/app/products/new/page.tsx`
- Modify: `src/ui/next/src/app/assistant/page.tsx`
- Modify: `src/ui/next/src/app/settings/page.tsx`
- Test: `src/server/api/catalog.rs`, `src/server/api/assistant_contract_test.rs`, `src/ui/next/src/app/products/new/page.test.tsx`

- [ ] **Step 1: Add a failing catalog test** that makes the model client fail and asserts `503 {"error":"catalog generation unavailable"}`; assert no title, description, or price is returned.
- [ ] **Step 2: Run the test and confirm RED** because the current handler returns `Generated Offering`, `AI description`, and `10.00`.
- [ ] **Step 3: Replace the fabricated response with an explicit provider error; only return fields parsed from a real model response that passes schema validation.**
- [ ] **Step 4: Add unit tests for each assistant endpoint currently returning `501`; require `{ "configured": false, "feature": ..., "message": ... }` for optional capabilities and real data for core assistant tasks/settings.**
- [ ] **Step 5: Update UI components to render a truthful disabled/configuration panel and never invent rows, plans, metrics, or tasks.**
- [ ] **Step 6: Run focused Rust/Vitest targets and confirm GREEN.**
- [ ] **Step 7: Commit:** `git commit -am "fix: remove fabricated feature responses"`.

### Task 4: Repair authenticated transport, aliases, and tenant propagation

**Files:**
- Modify: `src/ui/next/src/lib/auth/backendTransport.ts`
- Modify: `src/ui/next/src/lib/auth/publicBackendProxy.ts`
- Modify: `src/ui/next/src/app/orders/page.tsx`
- Modify: `src/ui/next/src/app/inventory/page.tsx`
- Modify: `src/ui/next/src/app/analytics/page.tsx`
- Modify: `src/ui/next/src/app/onboarding/page.tsx`
- Modify: `src/ui/next/src/app/settings/page.tsx`
- Modify: `src/ui/next/src/app/assistant/page.tsx`
- Create: `src/ui/next/src/lib/auth/backendTransport.production.test.ts`
- Test: `src/ui/next/src/lib/auth/backendTransport.production.test.ts`

- [ ] **Step 1: Add failing transport tests** for `x-tenant-id`, `x-user-id`, bearer forwarding, canonical `/api/v1/ui/*` aliases, and timeout/unavailable mapping.
- [ ] **Step 2: Run Vitest and confirm RED** for missing tenant/query/header propagation and current 502/504 mappings.
- [ ] **Step 3: Implement the shared transport contract** so every authenticated request derives identity from the sealed session, appends required tenant parameters exactly once, and returns stable typed JSON for timeouts.
- [ ] **Step 4: Replace page-local incorrect endpoint paths with canonical routes; provide a safe `tenant_id` only from the verified session, never a hard-coded tenant.**
- [ ] **Step 5: Run focused Vitest and live GETs for orders, inventory, dashboard metrics, onboarding state/draft, staff, and integrations.**
- [ ] **Step 6: Commit:** `git commit -am "fix: propagate authenticated tenant context"`.

### Task 5: Fix logout and realtime browser paths

**Files:**
- Modify: `src/ui/next/src/app/api/v1/auth/logout/handler.ts`
- Modify: `src/ui/next/src/app/components/LogoutButton.tsx`
- Modify: `src/ui/next/src/lib/auth/middleware.ts`
- Modify: `deploy/helm/ohc/templates/ingress.yaml`
- Modify: `src/server/api/agent_feed.rs`
- Modify: `src/server/api/realtime.rs`
- Create: `src/ui/next/src/app/components/LogoutButton.production.test.tsx`
- Test: `src/ui/next/src/app/api/v1/auth/logout/route.test.ts`, `src/ui/next/src/app/components/LogoutButton.production.test.tsx`, `src/server/api/realtime.rs`

- [ ] **Step 1: Add failing unit tests** for an empty trusted logout POST, cookie deletion when backend revocation fails, and idempotent repeated logout.
- [ ] **Step 2: Add a failing Playwright test** that logs in, clicks the visible Log out button, and requires navigation to `/login` with no protected content.
- [ ] **Step 3: Fix the logout request body/origin handling and make cookie deletion independent of backend revocation.
- [ ] **Step 4: Add an authenticated realtime ticket/upgrade path and ingress route that preserves WebSocket upgrade headers; add a browser test that accepts either a live feed message or the documented polling fallback without 502.**
- [ ] **Step 5: Run unit tests and the focused browser flow against the live service.**
- [ ] **Step 6: Commit:** `git commit -am "fix: complete browser logout and realtime paths"`.

### Task 6: Complete feature status APIs and UI disabled states

**Files:**
- Modify: `src/server/lib.rs` route aliases near `7600-7780`
- Modify: `src/server/api/billing_api.rs`
- Modify: `src/server/api/payment_ledger.rs`
- Modify: `src/server/api/staff_mesh.rs`
- Modify: `src/server/api/pos.rs`
- Modify: `src/server/api/onboarding/mod.rs`
- Modify: `src/server/api/mesh_handler.rs`
- Modify: `src/ui/next/src/app/payments/page.tsx`
- Modify: `src/ui/next/src/app/pos/terminal/page.tsx`
- Modify: `src/ui/next/src/app/goose-mcp/page.tsx`
- Modify: `src/ui/next/src/app/sona/page.tsx`
- Test: Rust module tests for each alias/status handler and Playwright feature-state assertions.

- [ ] **Step 1: Add failing API contract tests** for each observed 404/501/503/504 endpoint, asserting the canonical route or a typed `configured` response.
- [ ] **Step 2: Run focused tests and confirm RED.**
- [ ] **Step 3: Wire missing aliases to existing real handlers, add required `tenant_id` extraction from claims, and make optional providers return truthful status JSON instead of transport errors. Core HeatWave-backed reads must execute real queries.**
- [ ] **Step 4: Update pages to render those states and stop treating a disabled provider as a fake empty success.**
- [ ] **Step 5: Run the direct authenticated API sweep and Playwright pages for assistant, onboarding, billing, payment, POS, staff, mesh, Goose, and Sona.**
- [ ] **Step 6: Commit:** `git commit -am "fix: complete feature status contracts"`.

### Task 7: Rename product branding to OmniSolo with compatibility checks

**Files:**
- Modify: all user-facing files returned by `rg -l -i 'ohc|one human corp|onehumancorp'` after excluding generated/build/vendor output
- Create: `src/omnisolo_branding_contract_test.rs`
- Modify: `README.md`, `CHANGELOG.md`, `docs/**`, `src/ui/next/src/app/layout.tsx`, `src/ui/next/src/app/components/**`, `package.json`, `Cargo.toml`
- Test: branding contract and browser title/visible-brand assertions.

- [ ] **Step 1: Add a failing source contract** that rejects user-visible `OHC`/`One Human Corp` strings while allowing an explicit compatibility allowlist for `OHC_DATABASE_URL`, legacy cookie/API names, Kubernetes secret keys, and migration comments.
- [ ] **Step 2: Run the contract and confirm RED; record the allowlist in `docs/omnisolo-compatibility.md`.
- [ ] **Step 3: Apply a reviewable codemod to product-facing strings, metadata, help content, package descriptions, docs, and release labels. Keep compatibility aliases at their boundary and add `OMNISOLO_*` canonical names.
- [ ] **Step 4: Run branding/unit tests and a Playwright assertion that page title and navigation show OmniSolo.
- [ ] **Step 5: Commit:** `git commit -am "refactor: rename product branding to OmniSolo"`.

### Task 8: Add exhaustive browser verification and run it on the live cluster

**Files:**
- Modify: `src/ui/next/src/e2e/production_feature_smoke.spec.ts`
- Create: `src/ui/next/src/e2e/production_route_inventory.ts`
- Modify: `playwright.config.ts`
- Modify: `/home/kevin/myk3s/tests/verify-onehumancorp-live.sh`
- Test: live Playwright suite only; no mocked network.

- [ ] **Step 1: Expand the suite to log in through the rendered form, crawl all static and dynamic routes, capture application responses, console errors, failed WebSocket upgrades, and protected-route redirects.**
- [ ] **Step 2: Add explicit browser tests for every previously failing feature family and real catalog create/read.**
- [ ] **Step 3: Run locally against `https://cloud.omnisolo.co` with the real admin credentials and confirm the suite fails only for the currently known baseline.**
- [ ] **Step 4: After fixes, require zero unexpected HTTP 4xx/5xx, zero placeholder strings, successful logout, successful catalog round trip, and correct public/protected route policy.**
- [ ] **Step 5: Commit:** `git commit -am "test: verify OmniSolo features through real browser"`.

### Task 9: Publish from the OmniSolo repository and deploy through GitOps

**Files:**
- Modify: Git remote for `/home/kevin/mono/.worktrees/onehumancorp-seaorm`
- Modify: `/home/kevin/myk3s/data/configs/cluster.yml`
- Modify: `/home/kevin/myk3s/apps/onehumancorp/**` only where the verified image or route contract requires it
- Test: Bazel release build, image manifest inspection, Flux HelmRelease readiness, migration Job completion.

- [ ] **Step 1: Verify access to `git@github.com:omnisolo-llc/onehumancorp.git`; update the application remote only after `git ls-remote` succeeds.**
- [ ] **Step 2: Run Rust unit/contract targets, Next unit tests, and the release Bazel build before publishing.**
- [ ] **Step 3: Publish immutable ARM64 backend and web images from the new remote and record their digests.**
- [ ] **Step 4: Update HeatWave production image pins in `cluster.yml`, commit/push GitOps, reconcile Flux, and wait for ExternalSecret, migration, backend, core, web, and Valkey readiness.**
- [ ] **Step 5: Verify the deployed pod image digests and migration Job completion through the live cluster.**
- [ ] **Step 6: Commit/push GitOps changes:** `git commit -am "chore: deploy OmniSolo production completion"`.

### Task 10: Final acceptance and branch handoff

**Files:**
- No new production files; use the checked-in specs, plans, tests, and verification script.

- [ ] **Step 1: Run all focused Rust and TypeScript tests and record zero failures.
- [ ] **Step 2: Run the complete live Playwright suite and the direct API sweep twice to catch transient 502/504 behavior.
- [ ] **Step 3: Verify HeatWave catalog create/read in a separate request and confirm no placeholder values are present in responses or source.
- [ ] **Step 4: Verify public/protected routes, OmniSolo branding, logout, realtime, and configured/disabled provider states manually in the browser.
- [ ] **Step 5: Confirm both worktrees are clean, remotes point to the requested repository, and immutable deployed digests match GitOps.
- [ ] **Step 6: Use the finishing-development-branch workflow to report the final commits, test evidence, deployment status, and any third-party provider paths that could not be exercised without credentials.
