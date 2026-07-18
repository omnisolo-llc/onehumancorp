# Secure Admin Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a fail-closed, one-time admin bootstrap that lets real deployment smoke tests authenticate normally.

**Architecture:** A narrowly scoped Axum setup router owns setup-token validation and transactional tenant/admin creation. Deployment scripts supply ephemeral credentials, call setup, log in, then use the issued JWT for protected `/api/v1` requests.

**Tech Stack:** Rust, Axum, SQLx/PostgreSQL/SQLite, Bash, Docker Compose, Helm/Kind, Bazel.

---

### Task 1: Lock the setup security contract

**Files:**
- Modify: `src/server/api/setup.rs`
- Test: `src/server/api/setup.rs`

- [ ] Add failing tests proving a missing, short, or incorrect `OHC_SETUP_TOKEN` cannot create an admin.
- [ ] Run `bazel test //src/server:server_lib_test --test_filter=setup --nocache_test_results --test_output=errors` and confirm the new tests fail for the missing behavior.
- [ ] Replace the client-controlled role with fixed `ADMIN`, require username/email/password/organization ID bounds, and validate `Authorization: Bearer` against the configured token.
- [ ] Run the focused Rust test and confirm all setup tests pass.

### Task 2: Make first-admin creation backend-correct and one-time

**Files:**
- Modify: `src/server/api/setup.rs`
- Test: `src/server/api/setup.rs`

- [ ] Add failing tests for tenant creation, first-admin creation, and conflict after the first admin.
- [ ] Run the focused Rust test and confirm the one-time behavior test fails.
- [ ] Use backend-specific SQL inside a transaction, store PostgreSQL roles as an array and SQLite roles as JSON, and commit only after tenant and admin creation succeed.
- [ ] Run the focused Rust test and confirm the setup suite passes.

### Task 3: Mount only the versioned setup route

**Files:**
- Modify: `src/server/lib.rs`
- Test: `src/server/lib.rs`

- [ ] Add a failing router test that expects `/api/v1/setup/admin` to reach the token gate and `/api/setup/admin` to return not found.
- [ ] Merge `api::setup::router(db.clone())` under `/api/v1/setup` outside the normal bearer middleware while retaining the setup router's own token gate.
- [ ] Run the focused server tests and confirm both route assertions pass.

### Task 4: Make deployment bootstrap fail closed

**Files:**
- Modify: `deploy/docker/server-init/bootstrap-admin.sh`
- Modify: `deploy/docker-compose.yml`
- Test: `deploy/tests/e2e_ci_execution_contract_test.sh`

- [ ] Add failing shell-contract assertions for `/api/v1/setup/admin`, the setup bearer header, and marker creation only on success.
- [ ] Run `bazel test //deploy:e2e_ci_execution_contract_test --nocache_test_results --test_output=errors` and confirm failure.
- [ ] Pass `OHC_SETUP_TOKEN` to server and server-init, send fixed ADMIN input to the versioned endpoint, and exit nonzero without a marker for unexpected status.
- [ ] Re-run the shell contract and `bash -n` until both pass.

### Task 5: Authenticate real Compose and Kind smoke requests

**Files:**
- Modify: `deploy/tests/docker_compose_e2e_test.sh`
- Modify: `deploy/tests/kind_e2e_test.sh`
- Test: `deploy/tests/e2e_ci_execution_contract_test.sh`

- [ ] Add failing contract checks requiring setup, login, token extraction, and bearer headers in both smoke scripts.
- [ ] Supply ephemeral setup/admin credentials, call setup after readiness, log in with the correct organization ID, and centralize authenticated curl headers.
- [ ] Run the contract, then `//deploy:kind_e2e_test` and `//deploy:docker_compose_e2e_test` with local execution and uncached test results.

### Task 6: Final verification

**Files:**
- Verify all modified files.

- [ ] Run `git diff --check` and all repository static contracts.
- [ ] Run the exact `bazel test //... --nocache_test_results --test_output=errors` command.
- [ ] Inspect complete output and report only results supported by the fresh run.
