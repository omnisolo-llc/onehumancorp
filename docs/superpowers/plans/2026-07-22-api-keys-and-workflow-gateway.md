# API Keys & Workflow Gateway Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement secure, token-based API keys allowing admins/members to execute Visual Workflows directly via a high-performance, secure HTTP REST gateway.

**Architecture:** Create a new migration for `api_keys` table. Implement Axum auth middleware hashing keys with SHA256 and matching in-db. Expose a unified public gateway endpoint `/api/v1/gateway/run`.

**Tech Stack:** Rust, Axum, PostgreSQL (sqlx), SHA256, Git, Bazel.

## Global Constraints
- Naming rules: Table names `api_keys`. Middleware name `api_key_auth_middleware`.
- Security rule: Enforce SHA256 hashing for storing keys (raw keys are never saved, only returned once during generation).
- Platform constraint: Ensure cross-compilation and local testing remain robust.

---

### Task 1: Create Database Migration for API Keys

**Files:**
- Create: `src/server/migrations/1006_api_keys_and_usage_logs.sql`
- Modify: `src/server/migrations/BUILD.bazel`

**Interfaces:**
- Produces: `api_keys` database table

- [ ] **Step 1: Write the SQL migration file**

Create `src/server/migrations/1006_api_keys_and_usage_logs.sql`:
```sql
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    member_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id VARCHAR(128) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE
);
CREATE INDEX IF NOT EXISTS idx_api_keys_hash ON api_keys(key_hash);
```

- [ ] **Step 2: Add the migration to `BUILD.bazel` filegroup**

Register the new SQL file inside the `srcs` of `filegroup` inside `src/server/migrations/BUILD.bazel`.

- [ ] **Step 3: Run the database schema contract tests to verify validity**

Run: `bazelisk test //src/server/migrations:sqlx_migration_contract_test`
Expected: PASS

- [ ] **Step 4: Commit changes**

```bash
git add src/server/migrations/1006_api_keys_and_usage_logs.sql src/server/migrations/BUILD.bazel
git commit -m "feat(db): add database migration for api_keys table"
```

---

### Task 2: Implement Key Generation API

**Files:**
- Modify: `src/server/auth/http.rs`
- Test: `src/server/auth/http.rs` (unit tests)

**Interfaces:**
- Produces: `POST /api/v1/settings/keys` JSON endpoint

- [ ] **Step 1: Write the failing TDD test for key generation**

Add a test in `src/server/auth/http.rs` verifying that a logged-in user can successfully generate a new key and get the plain-text token (returned exactly once).

- [ ] **Step 2: Run the test to verify it fails**

Run: `bazelisk test //src/server/auth:server_auth_unit_test`
Expected: FAIL

- [ ] **Step 3: Implement route and handler inside `http.rs`**

- Create `POST /api/v1/settings/keys` route.
- Implement the handler:
  - Generates a random 32-byte key, base64url-encodes it to return to the user.
  - Hashes it with SHA256, and stores it in the `key_hash` field of the `api_keys` table.

- [ ] **Step 4: Run test to verify it passes**

Run: `bazelisk test //src/server/auth:server_auth_unit_test`
Expected: PASS

- [ ] **Step 5: Commit changes**

```bash
git add src/server/auth/http.rs
git commit -m "feat(auth): implement token-based api key generation API"
```

---

### Task 3: Implement Gateway Middleware and Run Endpoint

**Files:**
- Modify: `src/server/auth/mod.rs`
- Modify: `src/server/api/onboarding/mod.rs`
- Test: `src/server/api/onboarding/mod.rs` (gateway test)

**Interfaces:**
- Produces: `api_key_auth_middleware` Axum middleware and `POST /api/v1/gateway/run` endpoint

- [ ] **Step 1: Write the failing TDD test for the gateway run endpoint**

Write a test in `src/server/api/onboarding/mod.rs` verifying that hit to `/api/v1/gateway/run` with `Authorization: Bearer <key>` successfully triggers workflow execution.

- [ ] **Step 2: Run the test to verify it fails**

Run: `bazelisk test //src/server/api:server_api_unit_test`
Expected: FAIL

- [ ] **Step 3: Implement `api_key_auth_middleware` and register route**

- Implement `api_key_auth_middleware` inside `src/server/auth/mod.rs`.
- Register the `/api/v1/gateway/run` endpoint inside `src/server/api/onboarding/mod.rs` using this middleware.

- [ ] **Step 4: Run test to verify it passes**

Run: `bazelisk test //src/server/api:server_api_unit_test`
Expected: PASS

- [ ] **Step 5: Commit changes**

```bash
git add src/server/auth/mod.rs src/server/api/onboarding/mod.rs
git commit -m "feat(gateway): implement gateway middleware and run endpoint"
```
