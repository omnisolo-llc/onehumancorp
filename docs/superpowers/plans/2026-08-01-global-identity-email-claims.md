# Global Identity Email Claims Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enforce one normalized email to one persisted user across password, OIDC, migration, and bootstrap paths.

**Architecture:** A portable `identity_email_claims` primary-key table is the serialization point. Every runtime user-creation path inserts the claim in the same transaction as the user, while migration validates and backfills existing users and bootstrap reconciles only the same owner.

**Tech Stack:** Rust, SeaORM, SQLite/MySQL/PostgreSQL, Tokio tests

---

### Task 1: Registration concurrency regressions

**Files:**
- Modify: `src/server/auth/seaorm_store.rs`

- [ ] **Step 1: Write failing multi-connection tests**

Create file-backed SQLite repositories on independent connections. Assert concurrent verification issues one ticket, concurrent consumption of one ticket creates one user, separate valid password tickets for case-variant email create one user, and separate OIDC subjects for the same normalized email create one user and one external identity.

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test -p server_auth atomic_registration_tests -- --test-threads=1 --nocapture`

Expected: FAIL because `identity_email_claim` and transactional claim behavior do not exist.

- [ ] **Step 3: Add the portable entity and atomic claim helper**

Define `identity_email_claim` with `normalized_email` as its non-autoincrement primary key, `user_id`, and `claimed_at`. Add a helper that inserts a claim through a `DatabaseTransaction` and maps unique conflicts to a caller-supplied public denial.

- [ ] **Step 4: Claim password and OIDC emails transactionally**

Normalize the candidate email inside each repository method. Insert the claim after conditional ticket/invitation acquisition and before inserting the user. Move registration-mode checks, invitation selection/consumption, existing-email checks, user insert, and external-identity insert into the OIDC transaction.

- [ ] **Step 5: Run tests to verify passing behavior**

Run: `cargo test -p server_auth atomic_registration_tests -- --test-threads=1 --nocapture`

Expected: all atomic registration tests PASS.

### Task 2: Migration and deterministic backfill

**Files:**
- Modify: `src/server/persistence/migration.rs`
- Modify: `src/server/persistence_commands_test.rs`

- [ ] **Step 1: Write failing migration tests**

Assert migration creates claims for existing normalized user emails and rejects two different users whose emails normalize to the same value, with the normalized email present in the error.

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test -p ohc-mono persistence_commands_test -- --test-threads=1 --nocapture`

Expected: FAIL because migration neither creates nor backfills identity claims.

- [ ] **Step 3: Create and backfill the claim table**

Create `identity_email_claims` with SeaORM schema generation. Read all users, normalize in Rust, sort by normalized email and user ID, reject different-owner collisions before writes, then insert missing claims transactionally without overwriting an existing owner.

- [ ] **Step 4: Run migration tests**

Run: `cargo test -p ohc-mono persistence_commands_test -- --test-threads=1 --nocapture`

Expected: migration backfill tests PASS.

### Task 3: Bootstrap reconciliation

**Files:**
- Modify: `src/server/persistence/commands.rs`
- Modify: `src/server/persistence_commands_test.rs`

- [ ] **Step 1: Write failing bootstrap tests**

Assert bootstrap inserts an absent claim, accepts a claim already owned by the same admin, and rejects a claim owned by another user without changing either row.

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test -p ohc-mono persistence_commands_test -- --test-threads=1 --nocapture`

Expected: FAIL because bootstrap updates/inserts users without reconciling claims.

- [ ] **Step 3: Reconcile in one transaction**

Normalize the email, begin a transaction, resolve the target admin user, insert a missing claim or verify its `user_id`, then update or insert the admin and commit. Return an error for a genuine ownership collision.

- [ ] **Step 4: Run bootstrap tests**

Run: `cargo test -p ohc-mono persistence_commands_test -- --test-threads=1 --nocapture`

Expected: all persistence command tests PASS.

### Task 4: Full verification and commit

**Files:**
- Modify: `src/server/auth/http.rs` only if test schema setup requires the new entity.

- [ ] **Step 1: Run registration and full auth suites**

Run: `cargo test -p server_auth registration -- --test-threads=1`

Run: `cargo test -p server_auth -- --test-threads=1`

Expected: all tests PASS.

- [ ] **Step 2: Check the exact diff**

Run: `git diff --check`

Run: `git diff --stat HEAD`

Expected: no whitespace errors and only identity-claim/auth registration changes.

- [ ] **Step 3: Commit**

```bash
git add src/server/auth/seaorm_store.rs src/server/auth/mod.rs src/server/auth/http.rs src/server/persistence/migration.rs src/server/persistence/commands.rs src/server/persistence_commands_test.rs
git commit -m "security: atomically claim identity emails"
```
