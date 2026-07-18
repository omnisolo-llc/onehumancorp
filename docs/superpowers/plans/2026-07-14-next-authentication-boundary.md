# Next.js Authentication Boundary Implementation Plan

**Goal:** Replace the browser login bypass with a real Rust-backed encrypted web session, protect Next.js pages and APIs by default, and remove browser-selected identity from protected backend traffic.

**Architecture:** Next.js exchanges bounded credentials with Rust, stores the backend token only inside a compact encrypted HttpOnly cookie, and validates that cookie locally in middleware. A shared server-only transport recovers the token, overwrites identity headers, and confines credentials to one configured backend origin. Rust remains the authorization and revocation authority. Public routes stay method-, invocation-, and path-specific; everything else is protected.

## Task 1: Runtime, origin, and cookie policy

**Files:**

- Create `src/ui/next/src/lib/auth/runtimeConfig.ts`
- Create `src/ui/next/src/lib/auth/runtimeConfig.test.ts`
- Create `src/ui/next/src/lib/auth/origin.ts`
- Create `src/ui/next/src/lib/auth/origin.test.ts`
- Create `src/ui/next/src/lib/auth/sessionCookie.ts`
- Create `src/ui/next/src/lib/auth/sessionCookie.test.ts`

Add failing tests first for canonical web origin, backend origin, embedded credentials, fragments, unsupported schemes, plaintext non-loopback backends, explicit loopback local mode, and LAN HTTPS. Require `OHC_WEB_CANONICAL_ORIGIN` and `BACKEND_URL` outside explicit `OHC_WEB_LOCAL_DEV=true`; never derive trust from Host or forwarded headers. LAN access uses an explicitly configured HTTPS origin and the production `__Host-ohc_session` cookie. Only explicit HTTP loopback local mode may use `ohc_session` without `Secure`.

Add exact Origin plus Fetch Metadata tests for same-origin unsafe requests, cross-site/same-site cross-origin/null/malformed/missing origins, and forged forwarding headers. Add cookie helpers for consistent set/delete attributes, bounded max-age, JWE context, and invalid-session clearing.

## Task 2: Bounded Next login endpoint

**Files:**

- Create `src/ui/next/src/app/api/v1/auth/login/route.ts`
- Create `src/ui/next/src/app/api/v1/auth/login/route.test.ts`
- Update `src/ui/next/src/lib/auth/types.ts`
- Update `src/ui/next/src/lib/auth/publicRoutes.ts`
- Update `src/ui/next/src/lib/auth/publicRoutes.test.ts`

Start with failing tests for exact JSON media type, 4,096-byte streaming body ceiling, unknown fields, username XOR email, password/tenant bounds, safe return path, same-origin enforcement, backend timeout/cancellation/redirect/oversize handling, generic denial mapping, malformed success responses, token non-disclosure, no-store responses, and exact cookie attributes.

Call only the configured backend `POST /api/v1/auth/login`, with redirects disabled and bounded request/response work. Validate the Rust response and expiry, seal the existing compact JWE session, set the cookie, and return only safe user metadata and destination. Do not write authentication data to browser storage.

## Task 3: Accessible login form and idempotent logout

**Files:**

- Rewrite `src/ui/next/src/app/login/page.tsx`
- Create `src/ui/next/src/app/login/page.test.tsx`
- Create `src/ui/next/src/app/api/v1/auth/logout/route.ts`
- Create `src/ui/next/src/app/api/v1/auth/logout/route.test.ts`
- Add/update the shell logout control that currently owns sign-out navigation

Use a native form with controlled identifier, password, and optional organization fields; keyboard submission; disabled duplicate submit; generic contained errors; and accessible status announcements. Navigate only after `/api/v1/auth/login` succeeds.

Logout validates same-origin mutation headers, decrypts the cookie when possible, calls Rust `POST /api/v1/auth/logout` with the recovered bearer token, and always deletes the local cookie. It is idempotent and returns private no-store success even when backend revocation is unavailable, while recording only safe static telemetry.

## Task 4: Protected-by-default middleware

**Files:**

- Create `src/ui/next/src/middleware.ts`
- Create `src/ui/next/src/middleware.test.ts`
- Update `src/ui/next/src/lib/auth/publicRoutes.ts`
- Update `src/ui/next/src/lib/auth/publicRoutes.test.ts`
- Update `src/ui/next/src/lib/auth/types.ts`

Add failing tests that distinguish pages from route handlers, Server Actions, RSC/data requests, prefetches, rewrites, and framework assets. Prove unknown routes are protected, ambiguous paths fail closed, unauthenticated pages redirect with a validated relative `next`, APIs return JSON `401`, invalid cookies are deleted, protected output is private/no-store, authenticated `/login` redirects, and unsafe cookie-authenticated mutations enforce origin policy.

Middleware performs only route classification and bounded local JWE validation. It never calls the backend for ordinary navigation and never treats cookie presence, local roles, local tenant metadata, or browser headers as authorization.

## Task 5: Server-only authenticated backend transport

**Files:**

- Create `src/ui/next/src/lib/auth/serverSession.ts`
- Create `src/ui/next/src/lib/auth/backendTransport.ts`
- Create `src/ui/next/src/lib/auth/backendTransport.test.ts`
- Replace `src/ui/next/src/app/api/v1/ui/backendProxy.ts`
- Replace authentication-sensitive rewrites in `src/ui/next/next.config.mjs` with route handlers using the transport
- Migrate protected route handlers in bounded groups

Test session recovery, configured-origin confinement, path/query injection, redirect rejection, cancellation, timeouts, request/response ceilings, and response header filtering. Strip browser Authorization, Cookie, tenant, user, role, and SPIFFE headers; inject bearer and legacy identity hints only from the verified session. Map backend `401` and `403` without weakening them and never cache personalized results.

Add a source-contract test that inventories protected route handlers and fails if a handler forwards to the backend without the shared transport or an explicit reviewed public contract.

## Task 6: Remove browser identity bypasses

**Files:**

- Create a server-derived `/api/v1/auth/session` metadata endpoint if client display metadata is required
- Update client components currently reading `token`, `auth_token`, `tenant_id`, `tenant`, `user_id`, roles, or SPIFFE identity from `localStorage` for protected requests
- Update static HTML surfaces under `src/ui/next/public/` that use demo identities
- Add a residue/source-contract test

Remove client-generated bearer headers and test/demo tenant fallbacks from production code. Protected client requests call same-origin Next handlers; the server transport supplies credentials. Browser storage may retain non-authoritative UI preferences only. Public widget flows must use a reviewed opaque capability or explicit public contract, not a raw tenant fallback.

The residue test permits identity literals only in explicit test fixtures and documentation, and fails on new production `e2e-tenant`, default tenant, client bearer, or local-storage authentication patterns.

## Task 7: Public-route and onboarding completion

**Files:**

- Expand `src/ui/next/src/lib/auth/types.ts` and `publicRoutes.ts` only for reviewed contracts
- Update onboarding routes/pages and completion/skip handoff
- Add capability, webhook, and public-route tests beside each public handler

Classify every genuinely public route with exact method/invocation/matcher, body limit, rate policy, tenant/capability source, replay/idempotency policy, cache policy, reason, and owner. No broad `/api/v1`, growth, onboarding, webhook, or static prefix exemptions. Webhooks without verified raw-body authentication remain protected.

Onboarding data collection may remain public, but completion, launch, tenant reads, billing, and dashboard handoff require a valid session. Completion and skip redirect to validated login return paths instead of dashboard access.

## Task 8: Browser fixtures, cache isolation, and visual verification

**Files:**

- Replace bypassing Playwright `loginAs` fixtures
- Add browser auth/session/logout regressions
- Update `src/ui/next/scripts/visual-audit.mjs`

Browser fixtures establish a session through the real login contract or a cryptographically equivalent test-only helper that exercises the same cookie validator. Test unauthenticated deep links/APIs, invalid login, valid login, tampered/expired cookie, logout, onboarding handoff, two-user cache isolation, and revocation behavior.

Run the complete desktop/mobile page matrix with real protected sessions. Inspect login empty/loading/error/success states and every page for shell consistency, overflow, obscured controls, console errors, hydration errors, and screenshot regressions.

## Verification after each bounded slice

Run focused Vitest tests first, then:

```bash
cd src/ui/next
pnpm exec tsc --noEmit
pnpm exec vitest run
pnpm exec next build
```

Restore generated `tsconfig.tsbuildinfo`, run local Bazel with remote BES disabled, run relevant Playwright/visual lanes, complete independent spec and quality review, and commit only after all findings are resolved. Before final delivery, rebase the latest remote `main`, rerun the full Rust/Next/security/token-efficiency matrix, and push without force.
