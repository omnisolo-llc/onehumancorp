# Rust Authentication Authority Hardening Plan

> Execute this slice with test-driven development, then run specification and quality review before moving to HTTP routing.

**Goal:** Make the Rust auth store the portable, timing-resistant credential and fail-closed revocation authority used by every transport.

**Architecture:** Add a repository-level username-or-email lookup that preserves tenant isolation and distinguishes absence from infrastructure failure. Centralize credential verification in `Store`, using the same single bcrypt verification path for present, missing, inactive, and tenant-invalid users. Change revocation APIs to return errors, propagate repository/Redis failures through token validation, and keep deterministic in-memory revocation for standalone stores without a persistence backend.

**Scope:** `src/server/auth` only. HTTP login/logout routing, browser cookies, middleware, and UI changes remain separate slices.

---

## Task 1: Define portable identifier lookup

**Files:**
- Modify: `src/server/auth/user_repository.rs`
- Modify: `src/server/auth/sqlite_store.rs`
- Modify: `src/server/auth/postgres_store.rs`

1. Add failing SQLite tests proving an active user is found by exact username and exact email within the requested tenant, while missing, inactive, and cross-tenant identifiers return `Ok(None)`.
2. Add `get_by_login_identifier(identifier, org_id) -> Result<Option<User>, String>` to `UserRepository`.
3. Implement one parameterized tenant-scoped query per backend, including `active = true`; PostgreSQL must retain transaction-local tenant context/RLS.
4. Run the targeted SQLite tests and the PostgreSQL compile/test lane available locally.

## Task 2: Make credential denial constant-work and generic

**Files:**
- Modify: `src/server/auth/mod.rs`

1. Add failing unit tests around an injectable verification helper proving exactly one password verification for a valid user, a wrong password, no user, inactive user, and tenant mismatch.
2. Update `Store::authenticate` to accept a login identifier, use repository or in-memory username/email lookup, and route every denial through one valid dummy bcrypt hash.
3. Return only `invalid credentials` for credential denial. Preserve infrastructure errors as an availability failure after dummy verification.
4. Run `cargo test -p server_auth --lib`.

## Task 3: Propagate revocation failures fail-closed

**Files:**
- Modify: `src/server/auth/mod.rs`

1. Add failing tests proving invalid tenant IDs and configured backing-store failures return `Err`, and token validation rejects when revocation status cannot be established.
2. Change `Store::revoke_token` to `Result<(), String>` and `Store::is_revoked` to `Result<bool, String>`.
3. Propagate repository failures. If Redis is configured, propagate connection/command failures. Continue using the local map in standalone memory-only mode.
4. Update all internal callers. Token validation must convert revocation lookup failure to a stable generic availability error. Logout must revoke against the token claim's organization and surface failure instead of claiming success.
5. Run the auth crate tests and compilation for affected server callers.

## Task 4: Verify and review

1. Run `cargo fmt --check` for touched Rust files, `cargo test -p server_auth --lib`, `cargo check -p server`, the Bazel auth test where available, and `git diff --check`.
2. Perform a specification review against this plan and the approved auth design.
3. Perform a code-quality/security review. Resolve all Critical, Important, and Minor findings and repeat verification.
4. Commit this atomic slice before starting bounded HTTP login/logout routing.
