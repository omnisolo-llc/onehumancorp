# Static Route Retirement Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan task-by-task. The owner already approved bounded removal and draft-PR publication.

**Goal:** Remove six redundant Next static implementations while retaining authenticated compatibility navigation and real E2E regression coverage.

**Architecture:** Add a small exact route map consumed only after existing authentication/origin checks. Canonical Next pages retain their existing real backend APIs. Migrate shipped links and keep retained standalone/public contracts.

**Tech Stack:** Next.js, TypeScript, Vitest, Node test runner, Playwright, real Rust/PostgreSQL/Valkey fixture.

**Spec:** `docs/superpowers/specs/2026-10-04-static-route-retirement.md`

## Global Constraints

- Retire only the six paths in the spec; no broad redesign or invented feature completion.
- Preserve existing authentication, tenant, public-publication, callback and Tauri bootstrap contracts.
- No successful API mocks in browser regression; no production/provider mutations.
- Keep Chromium sandbox enabled; use one heavy Cargo invocation per checkout.
- New isolated branch and draft PR only; do not alter PR40076, merge or deploy.

## Review Focus

- Anonymous API-shaped aliases must retain 401 instead of accidental login HTML/200.
- Expired cookies and mutation methods must not bypass existing checks.
- Exact mapping must not rewrite arbitrary names, encoded paths or off-origin destinations.
- Queries/fragments and retained helper links must survive compatibility navigation.
- Deleted source-presence tests must be replaced with relevant behavior coverage, not silently discarded.

### Task 1: Authenticated compatibility and static retirement

**Files:** `src/ui/next/src/lib/auth/middlewareCore.ts`, its test, a focused `retiredPageRoutes.ts` map, six listed public HTML files, shipped help/dashboard references, `scripts/legacy-api-docs-contracts.test.mjs`, new `scripts/retired-static-pages.test.mjs`.

**Interfaces:** `retiredPageDestination(pathname: string): string | null` returns an exact same-origin pathname; authentication remains owned by `evaluateAuthMiddleware`.

- [ ] Establish focused existing auth/docs/component/legacy-contract baseline.
- [ ] Add tests for all six mappings with verified session, anonymous/expired session, GET/HEAD vs unsafe methods, private/no-store and query preservation. Run and observe expected failures against current behavior.
- [ ] Add file/link regression tests; observe expected failures while obsolete files and links exist.
- [ ] Implement exact map after auth checks, remove six files, migrate shipped references, retain historical Tauri DOM contracts and canonical tooltip behavior tests.
- [ ] Run focused tests and review the change.

### Task 2: Real served browser regression

**Files:** `src/ui/next/src/e2e/static-route-retirement.spec.ts`, relevant existing documentation E2E expectations, audit-derived remaining-work documentation.

- [ ] Assert every approved alias reaches the expected canonical route and loaded backend-backed UI at desktop/narrow sizes, with no required asset404s.
- [ ] Verify anonymous page/API aliases, retained public routes and a nonexistent reviewed-publication document; do not invent successful provider responses.
- [ ] Follow actual shipped Help and dashboard links; check canonical tooltip keyboard behavior, discovery and retained source contracts.
- [ ] Run against the real isolated backend and source-bound served web build, record exact outcomes and screenshots.

### Task 3: Acceptance, review and draft PR

- [ ] Run required `make lint` and `make test`; report all failures/unavailable prerequisites honestly and run unaffected focused checks.
- [ ] Record exact test inventory change for coordination with PR40076; do not modify CI implementation.
- [ ] Obtain independent whole-branch review and resolve important findings with regression evidence.
- [ ] Commit, push this branch and create a draft PR with findings, exact verification, remaining limitations and bounded future implementation scope. No merge/deployment.
