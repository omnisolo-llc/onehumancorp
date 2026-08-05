# Admin Console & Member Analytics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the new `user_usage_logs` table in PostgreSQL, expose a secure Axum aggregation endpoint `/api/v1/ui/admin/usage`, and build a beautiful Member Analytics dashboard tab in our Next.js admin interface.

**Architecture:** Create SQL migration `1007_user_usage_logs.sql`. Implement SQL aggregates in a new Axum handler inside `src/server/auth/http.rs`. Create a Member Analytics view in `src/ui/next` showing detailed usage.

**Tech Stack:** Rust, Axum, React, TypeScript, TailwindCSS, PostgreSQL.

## Global Constraints
- Naming rules: Table name `user_usage_logs`. Endpoint path `GET /api/v1/ui/admin/usage`.
- Security rule: Endpoint must require `ROLE_ADMIN` check to prevent non-admins from viewing cost data.

---

### Task 1: Create Database Migration for User Usage Logs

**Files:**
- Create: `src/server/migrations/1007_user_usage_logs.sql`
- Modify: `src/server/migrations/BUILD.bazel`

**Interfaces:**
- Produces: `user_usage_logs` database table

- [x] **Step 1: Write the SQL migration file**

Create `src/server/migrations/1007_user_usage_logs.sql`:
```sql
CREATE TABLE IF NOT EXISTS user_usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id VARCHAR(128) NOT NULL,
    feature VARCHAR(128) NOT NULL,
    tokens_used INTEGER NOT NULL DEFAULT 0,
    computed_cost NUMERIC(10, 4) NOT NULL DEFAULT 0.0000,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_user_usage_logs_user ON user_usage_logs(user_id);
```

- [x] **Step 2: Run the database schema contract tests to verify validity**

Run: `bazelisk test //src/server/migrations:sqlx_migration_contract_test`
Expected: PASS

- [x] **Step 3: Commit changes**

```bash
git add src/server/migrations/1007_user_usage_logs.sql
git commit -m "feat(db): add user_usage_logs database schema migration"
```

---

### Task 2: Implement Usage Aggregation API

**Files:**
- Modify: `src/server/auth/http.rs`
- Test: `src/server/auth/http.rs` (unit tests)

**Interfaces:**
- Produces: `GET /api/v1/ui/admin/usage` endpoint requiring `ROLE_ADMIN`

- [x] **Step 1: Write the failing TDD test for the analytics API**

Add a test in `src/server/auth/http.rs` verifying that fetching `/api/v1/ui/admin/usage` returns aggregated usage rows and correctly restricts access to `ROLE_ADMIN`.

- [x] **Step 2: Run the test to verify it fails**

Run: `bazelisk test //src/server/auth:server_auth_unit_test`
Expected: FAIL

- [x] **Step 3: Implement route and handler inside `http.rs`**

- Create `GET /api/v1/ui/admin/usage` route.
- Implement the handler which runs a database GROUP BY aggregation of `user_usage_logs` joined with `users` to display usernames, features, token sums, and cost sums.

- [x] **Step 4: Run test to verify it passes**

Run: `bazelisk test //src/server/auth:server_auth_unit_test`
Expected: PASS

- [x] **Step 5: Commit changes**

```bash
git add src/server/auth/http.rs
git commit -m "feat(auth): implement user usage log aggregation API"
```

---

### Task 3: Build Member Analytics Panel UI

**Files:**
- Create: `src/ui/next/src/app/settings/MemberAnalytics.tsx`
- Modify: `src/ui/next/src/app/settings/page.tsx`

**Interfaces:**
- Produces: "Member Analytics" settings panel tab

- [x] **Step 1: Create the MemberAnalytics React component**

Develop `src/ui/next/src/app/settings/MemberAnalytics.tsx` displaying a beautiful, clean Tailwind-styled analytics table showing each member's username, features accessed, total tokens, and computed cost.

- [x] **Step 2: Integrate into the workspace settings page**

Mount the `MemberAnalytics` component as a new tab inside the `/settings` page, protecting it to be visible only to logged-in workspace administrators.

- [x] **Step 3: Verify the UI compiles and unit tests pass**

- [x] **Step 4: Write Playwright E2E tests for the new UI**

Run: `bazelisk test //src/e2e:playwright_spec_coverage`
Expected: PASS

Run: `pnpm exec tsc --noEmit` and `pnpm exec vitest run`
Expected: PASS

- [ ] **Step 5: Run UI E2E tests**

Run: `bazelisk test //src/e2e:playwright --local_test_jobs="$(nproc)"`
Expected: PASS

- [x] **Step 5: Run UI E2E tests**

Run: `bazelisk test //src/e2e:playwright --local_test_jobs="$(nproc)"`
Expected: PASS

- [x] **Step 6: Fix remaining E2E test failures on main**

Run: `bazelisk test //src/e2e:playwright_spec_coverage`
Expected: PASS

- [x] **Step 7: Revert changes to e2e test files**

Run `git checkout src/e2e/test_subscriptions.spec.ts src/e2e/whatsapp_cloud_api_settings.spec.ts src/ui/next/src/e2e/member-analytics.spec.ts src/e2e/autonomous_booking_system.spec.ts src/e2e/playwright/nora-proposal-intake.spec.ts src/e2e/terminal_edge_sync.spec.ts src/e2e/triage-unified-inbox-instagram.spec.ts src/ui/next/src/e2e/dashboard-triage-edit.spec.ts`
Expected: Reverts files changed.

- [x] **Step 7: Commit changes**

```bash
git add src/ui/next/src/app/settings/MemberAnalytics.tsx src/ui/next/src/app/settings/page.tsx src/ui/next/src/e2e/member-analytics.spec.ts src/e2e/BUILD.bazel
git commit -m "feat(ui): add member analytics panel"
```
