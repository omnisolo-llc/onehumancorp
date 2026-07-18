# Authentication and Dependency Hardening Design

## Objective

Close the confirmed Next.js authentication bypass and remediate the remaining fixable dependency, quality, and operational findings without weakening the UI, backend authorization, or previously verified production-agent behavior. Quality and security take priority over migration speed, performance, and diff size.

The current login page accepts any non-empty username, ignores the password, writes a display name to `localStorage`, and navigates directly to `/dashboard`. The Next.js application has no middleware authentication boundary, and the root end-to-end login fixture also opens `/dashboard` without a session. The Rust backend already performs password verification, tenant-scoped user lookup, JWT issuance, and token revocation. This project will connect the browser application to that real authority and make unauthenticated access fail closed.

## Chosen Approach

Use a locally verified, encrypted HttpOnly web session while retaining the Rust backend as the authorization authority for every backend operation.

The Next.js server will exchange credentials with the existing Rust login endpoint, place the returned backend access token and minimal session metadata in an encrypted cookie, and validate that cookie locally at the page/API boundary. Protected server-side requests will recover the token and forward it as a bearer credential. Backend endpoints will continue to validate the bearer token, including revocation and tenant authorization.

This hybrid gives page navigation a local cryptographic check without adding a backend round trip to every navigation. A revoked token might still pass the local page-shell check until its 24-hour expiry, but it cannot successfully read or mutate protected backend data because each backend operation remains authoritative. Logout will attempt backend revocation and clear the local cookie regardless of backend availability.

Rejected alternatives are:

- Checking only for cookie presence. This is forgeable and would preserve the bypass in another form.
- Introspecting the backend on every navigation. This provides immediate page-level revocation but adds latency and makes every page request depend on backend availability.
- Sharing the backend JWT signing secret with Next.js. This expands the blast radius of the backend signing key and couples two issuers unnecessarily.
- Migrating directly to Auth.js, Next.js 16, and React 19. That is a larger platform migration than the security repair requires and would mix framework compatibility risk with the authentication boundary.

## Security Boundary

### Protected by default

All rendered pages, route handlers, Server Actions, RSC/data requests, prefetches, and rewrites are protected unless the exact invocation matches a reviewed public allowlist. Unknown and newly added routes inherit protection automatically. A public page does not make its co-located Server Actions or data mutations public.

The public page classes are limited to:

- Login.
- Public onboarding and account-setup entry points.
- Help and public documentation.
- Explicit storefront, checkout, booking, bio, review, giveaway-entry, and embeddable widget experiences intended for unauthenticated customers.
- Framework assets and metadata required to render public pages.

The public API classes are limited to:

- The Next.js login endpoint.
- Public onboarding endpoints required before account creation.
- Health/readiness endpoints containing no tenant data.
- Explicit storefront, checkout, booking, referral, and widget/embed endpoints whose backend contract is public.
- Required third-party webhooks that perform their own signature or shared-secret verification.

Every public API entry declares its allowed methods, request/body limit, rate limit, tenant or object-capability source, idempotency/replay behavior, and response cache policy. Public checkout, booking, referral, and widget operations use opaque scoped capabilities or server-derived tenant context; they never trust a raw browser tenant/user header as authorization. State-changing GET requests are rejected.

Webhook entries additionally require a fail-closed configured secret or asymmetric verifier, signature verification over the exact raw body, a bounded timestamp window, replay detection, constant-time comparison where applicable, and idempotent processing. A webhook without a verified implementation remains protected and is not added to the allowlist.

The implementation plan must produce a machine-readable allowlist keyed by allowed HTTP method, canonical path matcher, invocation type, reason, and owner category. Broad prefix exemptions such as all of `/api/v1`, all growth routes, or all webhook routes are forbidden. Dynamic public paths must use anchored matchers and tests for near-miss protected paths.

Policy evaluates one canonical URL representation after rejecting ambiguous encodings. Tests cover route groups, configured base paths/locales, trailing slashes, case differences, encoded separators, dot segments, duplicate slashes, double encoding, RSC/data suffixes, and rewrite destinations. A URL that cannot be normalized unambiguously is protected or rejected; it is never made public by normalization disagreement between middleware and a route handler.

Static framework paths such as `/_next/static`, image optimization, icons, and robots metadata are excluded from session checks only to the extent required by Next.js. Public asset matching is separated from action, RSC, and route-handler matching. Development-only assets must not create a production bypass.

### Authentication responses

Unauthenticated browser page requests redirect to `/login?next=<validated-relative-path>`. The destination must be a same-origin relative path and must never accept an absolute URL, protocol-relative URL, control characters, or an encoded equivalent.

Unauthenticated API requests return a JSON `401` response and do not redirect to HTML. Authenticated users visiting `/login` are redirected to the validated destination or `/dashboard`.

Invalid, expired, undecryptable, or structurally incomplete sessions are identical to no session. The response clears the invalid cookie when the framework surface permits it. Authorization failures from the backend remain `403`; authentication failures remain `401` and cause the client to clear its local session or navigate to login as appropriate.

## Web Session Contract

### Cookie

Use a single host-only cookie with these properties:

- Name: `__Host-ohc_session` in every production deployment. A development-only name may be used only in an explicit loopback-bound local mode when HTTP prevents the `__Host-` contract; forwarded headers or request host values cannot enable that downgrade.
- `HttpOnly` so browser JavaScript cannot read the backend token.
- `Secure` outside an explicit local-development environment.
- `SameSite=Lax` to block ordinary cross-site form submission while preserving top-level return navigation.
- `Path=/` and no `Domain` attribute.
- `Max-Age` no longer than the backend token's reported expiry, currently 24 hours.
- Explicit deletion using the same path and security attributes.

Session material must never be written to `localStorage`, `sessionStorage`, client-readable cookies, URLs, logs, analytics, error messages, or rendered HTML.

### Cryptography and configuration

Use the maintained, Edge-compatible `jose` implementation with compact JWE, direct encryption (`alg=dir`), and AES-256-GCM (`enc=A256GCM`). The library generates a fresh cryptographically random 96-bit nonce for every session. The JWE protected header and validated payload bind the cookie name, format version, deployment environment/audience, and key ID so a token cannot be moved between environments or session purposes.

`OHC_WEB_SESSION_SECRET` is separate from the backend JWT signing key and decodes to exactly 32 random bytes. The implementation must not silently truncate, pad, repeat, or hash an empty/weak value into an accepted key. Secret values are read only by server/Edge code, use no `NEXT_PUBLIC_` name, and are never serialized into client bundles, source maps, build output, or logs; source and bundle regressions enforce this.

Production runtime and readiness behavior must fail closed when the web-session secret is absent, malformed, or below the entropy requirement. The secret must not be required during a static build step that could embed or expose it in an artifact. Tests may inject a deterministic test-only secret; explicit local development uses a separately named opt-in configuration and is accepted only while the server is loopback-bound. No production fallback or checked-in secret is allowed.

Rotation uses a bounded server-only key ring: one active key ID for issuance and at most one explicitly configured previous key for decryption during a period no longer than the maximum session lifetime. Unknown key IDs fail closed. Removing the previous key intentionally invalidates its sessions. Deployment documentation orders key provisioning, active-key switch, waiting or intentional invalidation, and old-key removal without logging key material.

The encrypted payload contains only:

- The backend access token.
- Numeric issued-at and expiry timestamps (`iat` and `exp`).
- User ID, username, roles, and organization ID needed for server-side display or request derivation.
- A web-session version used to reject incompatible formats.

It excludes passwords, password hashes, email unless required by an existing server-only UI contract, onboarding drafts, and mutable client-selected tenant data. Decryption validates the version, audience/environment, key ID, expected field types, non-empty user/organization identity, bounded clock skew, `iat <= now + skew`, `now < exp`, `exp > iat`, and `exp <= iat + configured maximum lifetime`. Issuance requires `exp` to be consistent with and no later than the backend-reported/token expiry. The plaintext, compact JWE, and final `Set-Cookie` value each have explicit byte ceilings below browser cookie limits; oversize is rejected and never truncated.

Roles and organization metadata in this local session are display and routing hints only. They may help derive legacy request headers, but no Next.js page, Server Action, route handler, or proxy helper may authorize tenant or role access from them. Backend `401`, `403`, revocation, and claim validation always prevail.

### Login

`POST /api/auth/login` accepts username, password, and optional organization ID, validates an exact JSON content type, enforces a platform-level body limit before parsing, validates bounded field lengths, and forwards the request server-to-server to `POST /api/v1/auth/login` on the Rust backend. The backend request has a short explicit timeout, cancellation propagation, a bounded response body, and redirect following disabled. It must not reveal whether a username, tenant, or password was the failing element.

Cloud/multitenant login must receive the organization from an explicit field or a trusted hostname-to-tenant lookup. The backend's current `e2e-tenant` fallback is permitted only in explicit test/local mode and must fail closed in production. A single-tenant production deployment may use an explicitly configured `OHC_DEFAULT_TENANT_ID`; it may not silently inherit a test identifier.

The Rust handler will perform a dummy password-hash verification when no matching active user exists so obvious response timing does not disclose account presence. Login attempts are rate limited at the server-facing boundary with bounded state, a deployment-aware trusted-client-IP policy, a generic `429`, and `Retry-After`. The limiter combines independent source and normalized-account buckets so an attacker cannot lock out an account indefinitely from one address or bypass limits by rotating only one dimension. Keys are privacy-preserving, credentials are never logged, entries expire/evict deterministically, and tests cover spoofed forwarding headers. Distributed deployments must use a shared limiter or explicitly document and enforce the capacity of a local per-instance layer plus an upstream limit.

Safe audit events record success, generic denial, throttling, configuration failure, and backend unavailability using correlation ID, coarse source classification, and hashed/rotating account key where operationally necessary. They exclude usernames, emails, passwords, tokens, cookies, bodies, and raw tenant identifiers.

On success, the route validates the backend response shape and expiry, encrypts the web session, sets the cookie, and returns only nonsensitive user metadata plus a validated navigation destination. It does not return the backend access token.

The login page becomes a controlled form with username and password state, native submit semantics, disabled duplicate submission, a contained generic error state, and accessible loading/error announcements. It navigates only after the session endpoint succeeds. Display-name convenience storage is removed or populated only from nonsensitive authenticated metadata after success; it is never an authentication signal.

### Logout

`POST /api/auth/logout` decrypts the current session, calls the backend logout/revocation endpoint with the bearer token when available, and always expires the local cookie. The Rust server does not currently expose HTTP logout, so the implementation adds a protected HTTP endpoint that validates the bearer token and records its JTI in the existing tenant-scoped revocation store. Both layers are idempotent. A backend outage may be reported through safe telemetry, but cannot prevent local logout.

State-changing session endpoints accept only requests matching a configured canonical origin in addition to `SameSite` cookie protection. The validator does not infer trust from attacker-controlled `Host`, `X-Forwarded-Host`, `Forwarded`, or same-site sibling subdomains unless a specific trusted-proxy/canonical-origin configuration authorizes them. It combines strict `Origin` validation with Fetch Metadata where supported and defines fail-closed behavior for missing/malformed headers.

The same validation applies to login (preventing login-CSRF/session swapping), logout, every cookie-authenticated Next.js API mutation, and every Server Action using an unsafe method, with narrowly documented webhook exceptions that authenticate their own raw request. State-changing GET routes are removed or return method-not-allowed. Tests cover cross-site, same-site cross-origin, opaque/null origin, missing origin, forged forwarding headers, and browser-valid same-origin requests.

Authentication and personalized responses use `Cache-Control: private, no-store` or an equivalent stricter framework guarantee and must never be stored in a shared cache. They never log request bodies or credentials.

## Server-Side Credential Propagation

Protected Next.js route handlers and rewrites must receive authentication from the verified server session, not from browser-supplied identity headers.

A shared server-only helper will:

1. Read and decrypt the session cookie.
2. Reject invalid or expired sessions.
3. Set `Authorization: Bearer <backend token>` on the outbound backend request.
4. Derive tenant and user headers only from authenticated session fields when a legacy backend contract still requires them.
5. Remove or overwrite browser-supplied `Authorization`, cookie, tenant, user, role, and SPIFFE identity headers at this trust boundary.
6. Resolve requests only against a configured backend origin; path/query input cannot replace the scheme, authority, or credentials.
7. Require HTTPS outside an explicit loopback local mode, disable automatic redirects, and accept a redirect only through a separately validated same-origin policy that never forwards credentials cross-origin.
8. Apply bounded connect/operation timeouts, cancellation, request/response byte limits, and safe content-type handling.
9. Strip hop-by-hop and sensitive backend response headers, including backend cookies, before constructing the browser response unless an exact route contract deliberately owns that header.

Forwarding the browser's raw cookie header to backend services is not an authentication mechanism. Routes that do not use the shared helper must either be explicitly public or prove an equivalent authenticated server boundary in a source-contract test.

The backend remains responsible for authorization, role checks, tenant isolation, token revocation, and sensitive-data filtering. The Next.js middleware check is defense in depth and page gating, not a replacement for backend authorization.

Backend-origin configuration is parsed once and fails closed on embedded credentials, fragments, unsupported schemes, non-loopback plaintext targets, or ambiguous hosts. Proxy tests cover absolute/path-derived URL injection, user-info syntax, DNS/IP variants, redirect chains, header-casing duplicates, response splitting attempts, timeout, oversize response, and client cancellation.

Protected pages, RSC/data responses, Server Actions, route handlers, rewrites, server-side fetches, and backend proxy responses are dynamic and `private, no-store` unless a reviewed tenant/user-aware cache contract proves isolation and revocation behavior. Tests request the same resource as two users and after logout/revocation to prove that CDN, Next.js, RSC, and fetch caches cannot replay authenticated content across identities or sessions.

## Onboarding Contract

Onboarding remains public so a new customer can prepare setup data. It does not grant dashboard access.

Completing or skipping onboarding without a valid authenticated session redirects to login with a validated `/dashboard` destination. If an onboarding endpoint later creates a real account and returns backend credentials, it must use the same server-side session issuance path; no special onboarding bypass may be introduced. Public onboarding state uses a cryptographically random, non-enumerable, scoped server-issued draft capability with explicit expiry and bounded storage rather than treating browser-selected `tenant_id`, `user_id`, or the `storefront` fallback as trusted authorization context. Draft rotation invalidates the replaced capability. Claiming a draft after login is an authenticated, tenant-bound, atomic, single-use operation so one account cannot attach another visitor's setup by guessing or replaying an identifier.

Existing onboarding endpoints will be classified as public-data collection or protected account mutation. Account mutations, business launch, billing, and tenant-owned reads must require authentication even if they live under an onboarding prefix.

## Dependency Remediation

The fresh Dependabot inventory at design time contains 52 open, fixable alerts: 36 npm, 12 Rust, and 4 Python; 14 are high, 27 medium, and 11 low. There are no open critical alerts in that inventory. Scanner output must be rechecked immediately before implementation because advisory state is time-sensitive. A committed disposition ledger records every alert ID, advisory/package, affected manifest and lockfile, vulnerable resolution, fixed resolution, production/development reachability, remediation commit, verification command, and status.

Dependency work is split into independently verified groups:

1. Upgrade the standalone UI from Next.js 14.2.35 to the current patched Next.js 15.5 backport (15.5.20 at design time), with the minimum compatible React/tooling adjustments. Do not combine this with Next.js 16 or React 19 migration. Recheck the backport tag and advisory floors immediately before lock generation.
2. Refresh root npm production dependencies and overrides for the reported `esbuild`, `vite`, and `ws` advisories.
3. Upgrade `pymdown-extensions` and regenerate the documentation lock consistently.
4. Upgrade direct Rust dependencies implicated by `jsonwebtoken` and OpenTelemetry advisories, then refresh transitive `rustls-webpki` and `glib` resolutions through compatible direct constraints.

Each group starts with a current advisory report, changes only the necessary manifests/locks, regenerates every supported npm and pnpm lock plus the Python and Rust locks consistently, and runs focused build/tests before the next group. Scanner versions and advisory databases are pinned or recorded in fail-closed CI so results are reproducible. An alert is complete only when the supported package-manager audit and authoritative advisory inventory no longer report the vulnerable resolution.

The acceptance target covers all 52 current alerts and any new alerts discovered before final verification, including development dependencies. A deferral is allowed only for a proven false positive or unreachable package with documented evidence, explicit owner, review date, expiry, and a CI-enforced exception; it is still shown as an unresolved risk rather than counted as fixed.

## Quality and Tooling Remediation

The implementation will address remaining formatter, Clippy, lint, and type-check failures in files touched by this project. Repository-wide cleanup may proceed in separate mechanical commits when the full command demonstrates an existing global failure and the change is reviewable. Behavior-changing lint fixes require focused tests; automatic mass rewrites must not obscure the authentication or dependency diffs.

No quality gate may be disabled, reduced to warnings, or narrowed merely to obtain a green result. If an unavailable external service prevents a test, the report must identify the exact missing prerequisite and preserve a fail-closed CI contract where production assurance depends on it.

## UI and Visual Integrity

The universal shell remains the single rendered shell contract. Authentication changes must not create a second shell, unstyled redirects, hydration replacement, horizontal overflow, or obscured controls.

Visual verification covers desktop and mobile states for:

- Empty login form.
- Keyboard submission.
- Loading and duplicate-submit prevention.
- Generic invalid-credential response.
- Successful redirect.
- Protected deep-link redirect and return.
- Expired/tampered session behavior.
- Onboarding completion and skip handoff to login.
- Logout and subsequent direct navigation denial.

The existing full page matrix is rerun in production mode. Protected pages use a real test session obtained through the login contract or a cryptographically equivalent test fixture; the audit may not bypass middleware by navigating directly. Public pages are checked without a session. Screenshots and console/hydration/page-error policy remain fail closed.

## Testing Strategy

Implementation follows test-driven development. Each behavior change begins with a focused failing test.

Required coverage includes:

1. Pure route-policy tests proving protected-by-default behavior across pages, handlers, actions, RSC/data, prefetch, and rewrites; method-specific exact public matches; canonicalization ambiguity rejection; dynamic boundaries; and near-miss rejection.
2. Session tests for JWE round trip and ciphertext confidentiality; nonce uniqueness; tampering; wrong/unknown/rotated key; environment/audience confusion; weak/missing configuration; `iat`/`exp` bounds and skew; backend-expiry consistency; malformed fields; byte ceilings; and version rejection.
3. Login route tests for content/body/field limits, cross-origin session swapping, backend timeout/cancellation/redirect/oversize/denial/outage, malformed backend success, cookie attributes, token non-disclosure, no-store responses, safe audit events, and safe destination handling.
4. Backend login tests for production tenant fail-closure, dummy-hash verification behavior, bounded dual-bucket rate limiting, deterministic eviction, retry metadata, and rejection of spoofed client-address headers.
5. Logout tests proving same-origin enforcement, idempotent cookie deletion, tenant-scoped backend revocation, and local success during backend failure.
6. Proxy tests proving credential injection from the session; removal of spoofed authorization, tenant, user, role, cookie, and SPIFFE headers; backend-origin and redirect confinement; header filtering; byte/time limits; and cancellation.
7. Middleware tests distinguishing browser redirect from API `401`, clearing invalid sessions, protecting unknown invocation types, validating canonical origin and Fetch Metadata for unsafe requests, and allowing only reviewed public method/path/handler combinations.
8. Public-route contract tests for capability scoping, method/body/rate/idempotency policy, webhook raw-body signature/timestamp/replay behavior, missing-secret fail-closure, and onboarding draft entropy/expiry/rotation/atomic claim.
9. Cache-isolation tests proving protected page, RSC, route-handler, server-fetch, and proxy output cannot cross users or survive logout/revocation through Next.js or simulated shared-cache behavior.
10. Source/bundle tests proving no Next code authorizes from local roles/organization metadata and no secret/token appears in client chunks, source maps, static output, or build logs.
11. Browser tests proving unauthenticated direct navigation and API calls fail, valid login succeeds, invalid login stays on the page, tampered/expired sessions fail, logout removes access, and public pages remain available.
12. Onboarding tests proving completion/skip cannot reach the dashboard without a session.
13. Existing Vitest, TypeScript, Next production build, Playwright, visual audit, package audits, focused Cargo tests, Rust advisory checks, Python dependency checks, Bazel UI tests, and any changed Rust Bazel targets.

Tests must not contain a helper named `loginAs` that merely navigates to a protected path. Shared browser fixtures must establish a session through the real login endpoint or an explicitly test-only cryptographic session helper that exercises the same validation contract.

## Performance and Token Efficiency

Middleware performs only bounded cookie parsing/decryption and route classification. It makes no backend request for a valid page navigation. Session work must not run for static framework assets. Cryptographic and matcher microbenchmarks are warranted only if measured page overhead is material; no latency claim is made without comparable evidence.

The authentication work adds no LLM prompts, tool schemas, or model calls. Dependency changes must preserve the previously verified agent request profile and token-efficiency regression. The final suite reruns the deterministic direct-adjacent token benchmark; quality assertions must stay unchanged or improve before any token result is accepted.

## Operational Security Actions

Some prior findings require authority outside the repository. The project will produce exact, non-secret runbook steps and evidence placeholders for:

- Revoking and rotating the previously tracked remote-cache/BES credentials.
- Assessing Git history, CI logs, remote-cache logs, and access logs for exposure or misuse.
- Rotating the production backend JWT secret if exposure cannot be excluded, with a rollout that intentionally invalidates existing sessions.
- Provisioning `OHC_WEB_SESSION_SECRET` through the deployment secret manager and documenting rotation behavior.
- Enabling authoritative dependency/security scanning in remote CI.

Repository changes may remove credentials and prevent recurrence, but must not claim an external credential was revoked, a secret was provisioned, or logs were reviewed without evidence from the owning system. These are reported as operational blockers until performed by an authorized operator.

## Delivery Sequence

1. Add failing route-policy and session regressions; implement fail-closed session primitives and configuration.
2. Add failing login/logout tests; implement server endpoints and the accessible login form.
3. Add failing middleware and proxy tests; enforce protected-by-default routing and server-derived credentials.
4. Classify public routes narrowly and repair onboarding/dashboard handoff.
5. Replace bypassing test fixtures and add authenticated browser/security coverage.
6. Upgrade Next.js to the patched 15.5 line and verify the complete UI/build/browser contract.
7. Remediate root npm, Python, and Rust dependency groups with focused verification after each.
8. Address safe quality/tooling debt and update the production review report.
9. Run full security, quality, token, and production visual verification.
10. Rebase onto the latest remote `main`, resolve conflicts without dropping upstream or user changes, rerun affected and final gates, then push reviewable commits.

Work remains directly on `main` as explicitly requested. Before any rebase or push, the tree and remote relationship are inspected. No force push is used unless separately authorized.

## Success Criteria

- A non-empty username can no longer bypass login.
- Every unknown/new page and API is protected by default.
- Pages, APIs, Server Actions, RSC/data requests, prefetches, and rewrites share one unambiguous protected-by-default policy.
- Public routes are method- and handler-specific, minimal, capability-scoped, justified, and regression tested.
- Browser JavaScript never receives or stores the backend access token.
- Tampered, malformed, weakly configured, and expired sessions fail closed.
- Protected backend operations use the server-recovered bearer token and reject spoofed identity headers.
- Bearer credentials cannot leave the configured backend origin, and authenticated content cannot cross users or survive logout through shared caches.
- All cookie-authenticated mutations and Server Actions enforce the configured canonical origin and Fetch Metadata policy.
- Next.js treats local role/organization metadata only as hints; backend authorization and revocation always win.
- Logout clears local access and attempts backend revocation.
- Onboarding cannot navigate an unauthenticated user into the dashboard.
- Every current and newly discovered dependency alert has a ledger disposition; supported production and development graphs have no open fixable alerts, and every exceptional deferral remains visible with evidence, owner, review date, and expiry.
- Existing UI shell, accessibility, responsive, hydration, TypeScript, build, browser, Cargo, Bazel, and agent-quality/token contracts pass.
- The production visual matrix is complete with no unexpected console, page, hydration, screenshot, shell, or overflow failures.
- External actions are either evidenced as completed or clearly reported as operational blockers.

## Non-Goals

- Migrating to Auth.js, Next.js 16, or React 19 in this security cycle.
- Replacing backend JWT authorization or its tenant/role model.
- Adding social login, password reset, MFA, remember-me, or long-lived refresh tokens.
- Treating middleware as sufficient authorization for backend data.
- Making public onboarding data authoritative for a tenant or user.
- Silencing scanners or relaxing existing gates to make the result appear clean.
