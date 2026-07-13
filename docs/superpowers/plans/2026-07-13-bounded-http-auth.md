# Bounded Rust HTTP Authentication Plan

**Goal:** Replace the inline database-specific HTTP login with bounded, portable handlers backed exclusively by `auth::Store`, and add protected idempotent HTTP logout.

**Architecture:** Keep transport code in `src/server/auth/http.rs` beside the authority. A small shared state owns the `Store`, bounded privacy-preserving dual-bucket limiter, and an explicit trusted-peer policy. Handlers manually enforce media type and byte limits before deserialization, map typed authority outcomes to generic HTTP responses, and never accept tenant or identity fallbacks. The root router only constructs the repository/store once, mounts the auth router, and serves with peer connection information.

## Task 1: Bounded HTTP contract

- Add failing router tests for exact `application/json`, body/field bounds, unknown fields, generic invalid credentials, email login, missing tenant fail-closure, safe no-store responses, and backend-unavailable mapping.
- Implement bounded request parsing with `serde(deny_unknown_fields)` and generic safe error bodies.
- Call `Store::authenticate`; do not issue SQL from the HTTP layer.

## Task 2: Dual-bucket login limiter and trusted peers

- Add deterministic clock-driven tests for independent source/account buckets, retry metadata, expiry, bounded eviction, normalized-account hashing, and spoofed forwarding headers.
- Use direct peer IP by default. Honor forwarding headers only when the direct peer exactly matches a bounded configured trusted-proxy IP list; reject malformed/ambiguous forwarded values.
- Keep only keyed hashes in limiter state, cap entries deterministically, and return generic `429` plus `Retry-After`.

## Task 3: Idempotent HTTP logout

- Add failing tests for missing/malformed/invalid bearer credentials, successful tenant-scoped revocation, repeated logout, and fail-closed revocation errors.
- Factor signed-claims validation plus revocation into a transport-neutral Store operation that ignores prior revocation only for the logout operation.
- Return no-store success without exposing token or claims.

## Task 4: Root wiring and removal

- Mount the auth router at `/api/v1/auth/login` and `/api/v1/auth/logout` with one repository-backed Store.
- Remove `HttpLogin*`, `http_login_handler`, `db_for_login`, raw login SQL, `OHC_DEFAULT_TENANT_ID`, and `e2e-tenant` fallback.
- Serve Axum with `SocketAddr` connect information and update Cargo/Bazel source/dependency declarations.

## Task 5: Verification and review

- Run focused red/green tests, all `server_auth` tests, strict auth clippy, root cargo check/tests affected by routing, local Bazel auth/server targets, and `git diff --check`.
- Complete specification review, then security/quality review; resolve every Critical, Important, and Minor finding before committing.
