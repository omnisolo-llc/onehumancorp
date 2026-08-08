# OmniSolo production feature completion design

**Date:** 2026-08-08  
**Status:** Proposed for implementation

## Goal

Make the deployed OmniSolo application truthful and usable across every feature currently exposed by the browser: every discovered page must render under the correct authentication policy, every page-backed API must return a real database-backed result or an explicit, typed configuration state, and no browser path may silently return mock data. The work must be covered by browser Playwright tests and focused unit tests, deployed to `cloud.omnisolo.co`, and verified again with a real administrator session.

## Current evidence and scope

The production browser crawl found 198 static page routes that returned HTTP 200, but it also found failures in documentation, walkthroughs, assistant, billing, payments, inventory, orders, onboarding, staff, POS, mesh, and realtime surfaces. The observed failures cluster into four causes:

1. Legacy handlers extract `Arc<crate::db::DB>` but are not consistently given the extension after the SeaORM cutover.
2. Next API proxies and page clients use incomplete route aliases or omit the authenticated tenant on backend requests.
3. Logout and feed WebSocket paths are not completing through the public web boundary.
4. AI/catalog code falls back to placeholder business data, and several assistant surfaces report `501 Not Implemented` without a stable status contract.

The implementation scope includes all page routes discovered from `src/ui/next/src/app`, all API paths those pages invoke, and the dynamic route variants exercised with real tenant/user IDs. Third-party providers remain opt-in: without credentials, their status and setup flows must be truthful, deterministic, and non-5xx; configured providers must be exercised in the browser where credentials are available.

## Architecture

### Database compatibility boundary

The production application continues to use SeaORM/HeatWave as the canonical persistence layer. A single compatibility adapter is added at the Axum router boundary for legacy handlers that still consume `Arc<DB>`; it is constructed from the selected database configuration and shares the same live connection lifecycle. This prevents individual routes from receiving a missing extension while avoiding per-handler mock stores or SQLite fallbacks. New code continues to use repository/SeaORM interfaces.

All route handlers that return lists, settings, metrics, tasks, orders, inventory, billing state, or help content must read the selected database or return a typed `configured: false`/`unavailable` response when an optional external provider is not configured. Placeholder records and fabricated success responses are prohibited.

### API contract and tenant propagation

The Next backend transport becomes the single proxy contract for authenticated browser calls. Each proxy derives `organization_id`, `user_id`, and bearer identity from the sealed session, adds the corresponding headers/query values required by the Rust handler, and maps backend timeout/unavailable errors to stable JSON with an explicit feature status. Missing aliases are either wired to the existing handler or removed from the client in favor of the canonical route. GET status endpoints must not mutate data.

### Realtime and authentication

Logout must clear the web session cookie even when backend revocation is unavailable, and its browser route must return `200 { ok: true }` for a trusted empty POST. The feed WebSocket must either complete an authenticated handshake through the ingress or expose a documented polling fallback without a browser-visible 502. Public routes remain limited to login, registration, verification, static framework assets, and explicitly reviewed marketing pages.

### OmniSolo rename and repository migration

OmniSolo becomes the canonical product, package, UI, documentation, and repository name. User-visible `OHC`/`One Human Corp` branding, titles, metadata, README text, generated help content, and release labels are renamed to OmniSolo. Technical compatibility identifiers that are part of a deployed contract (for example `OHC_DATABASE_URL`, legacy API paths, cookie names, and existing Kubernetes secret keys) remain accepted as deprecated aliases while new documentation and generated manifests use `OMNISOLO_*`/OmniSolo names. A source contract prevents new user-visible OHC branding and documents each retained compatibility alias.

The application remote is migrated to `git@github.com:omnisolo-llc/onehumancorp.git`. Existing deployment history is preserved; the current GitOps repository continues to pin immutable image digests until the new upstream repository publishes equivalent release artifacts.

## Verification design

### Unit and contract tests

Add focused tests before each implementation change:

- router construction supplies the legacy `DB` extension to documentation, walkthrough, assistant, and other legacy handlers;
- catalog generation rejects provider failure instead of returning placeholder title/description/price;
- authenticated transport injects tenant/user identity and maps timeout/unavailable states;
- logout accepts an empty trusted POST, clears the session, and remains idempotent when backend revocation fails;
- public landing/registration policy and OIDC provider status are explicit;
- every formerly `501` assistant endpoint has a stable status schema or a browser-hidden feature flag;
- route aliases and WebSocket ticket/origin handling are canonical.

Tests run with the existing Rust unit/contract targets and the Next/Vitest unit targets. No test may assert fabricated catalog or analytics rows as a substitute for a database.

### Playwright browser tests

Create a production-oriented Playwright suite that logs in through the real UI and verifies:

- login success, invalid credentials, logout, session gating, closed registration, email verification entry, and configured-provider visibility;
- dashboard, settings, onboarding, orders, inventory, products, analytics, assistant, payments, POS, integrations, help, API docs, and all generated/marketing feature pages;
- real catalog create/read and UI rendering of the persisted record;
- settings reads and safe disabled-provider states;
- QR generation, product generation failure semantics, route navigation, and feed/realtime fallback;
- a generated route inventory with dynamic IDs, no page-level 5xx, no console errors caused by application responses, and no hidden placeholder business data.

The browser suite uses only the live cluster and a real administrator account. It must not mock `fetch`, service workers, database responses, or provider responses. External side effects such as payment capture, SMS sending, or webhook delivery are verified through dry-run/validation paths unless a real provider credential is explicitly present.

## Rollout and acceptance

1. Add red unit/contract tests for one failure class at a time.
2. Implement the smallest root-cause fix and make the focused test green.
3. Build and publish immutable ARM64 backend/web images.
4. Update the GitOps pins, reconcile Flux, and wait for migrations and all deployments.
5. Run the complete Playwright live suite and direct API contract sweep.
6. Require zero unexpected page/API 4xx/5xx responses, zero placeholder data, successful real HeatWave CRUD, and clean browser console/network logs before declaring completion.

## Non-goals

- Removing PostgreSQL/SQLite support or legacy deployment manifests.
- Replacing HeatWave with an in-memory, mock, or local fallback.
- Claiming third-party provider execution without valid provider credentials.
- Rewriting API paths solely for branding when backward compatibility is required.
