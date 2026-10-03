# Checkpoint Restore Safety Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Reject unscoped, missing, or wrong-task restores without mutation and preserve unrelated work during an authorized local restore.

**Architecture:** Add a task-scoped restore API and migrate its callers; retain the old signature as an explicit unsupported operation for real stores. Resolve and validate immutable Git targets before stashing; never clean/reset or remove an index lock. Scope PostgreSQL target lookup and history pruning within one transaction.

**Tech Stack:** Rust, Tokio, Git CLI, SQLx/PostgreSQL.

**Spec:** Parent repair assignment: validate task/checkpoint membership and target before mutation; preserve unrelated/untracked/ignored work and existing locks; keep raw runtime disabled. This is standalone safety hardening, not tenant-service acceptance.

## Global Constraints

- Work only in the isolated worktree based on db529fe95; parent owns integration/publication.
- Private temporary Git repositories and explicitly disposable PostgreSQL only.
- Coordinate one heavy Cargo invocation with the backend worker.
- No raw-runtime enablement, new endpoint, or unrelated feature expansion.

## Review Focus

- Sanitized tag collisions must not resolve another checkpoint's metadata.
- Ignored file and directory collisions must be rejected before stashing.
- Existing index locks, including linked worktrees, must remain untouched.
- Stash errors must abort without checkout/reset/clean; saved work must remain recoverable.
- Duplicate checkpoint IDs in separate threads must not delete the other thread's history.

## Task 1: Git and protocol safety

Files: src/agents/builtin/checkpointer.rs, agent_protocol.rs, agent.rs, gather_act_verify.rs.

- [x] Add real temporary-repository regression tests for missing/wrong-task IDs, snapshots of files/index/refs, index locks, ignored collisions and successful reversible restore.
- [x] Run regressions against original code and observe expected failures.
- [x] Add restore_checkpoint_for_thread(thread_id, checkpoint_id); old restore returns a clear scope-required error for real stores. Require explicit scoped adapter implementation; never delegate to a legacy unscoped method.
- [x] Resolve exact checkpoint identity to immutable commit, preflight ignored collisions and index lock, stash with checked exit status, then checkout a new branch using --no-overwrite-ignore. Never reset/clean. Include recovery stash ref in checkout failure.
- [x] Migrate all known callers to scoped API and make protocol reject wrong-task membership through its store. Configured-store rewind failures must abort without a success event.
- [x] Cover duplicate and sanitized-colliding IDs with task-specific refs outside the legacy tag namespace; retain nonoverwriting compatibility aliases and verify legacy target provenance with literal paths.
- [x] Run focused Git/protocol/caller tests and formatting.

## Task 2: PostgreSQL scope and test isolation

Files: src/agents/builtin/checkpointer.rs.

- [x] Add explicit disposable-DB regression for duplicate IDs, wrong-task/missing restore and correct history truncation.
- [x] Replace unsafe hardcoded public-table test mutations with private temporary tables on a single-connection fixture.
- [x] Observe failing duplicate-ID/wrong-task case on original store behavior.
- [x] Implement exact thread/checkpoint selection with row lock and scoped deletion in one transaction.
- [x] Wire the named disposable fixture and focused harness into native CI.
- [x] Run explicit PostgreSQL regressions, all focused checkpointer tests, affected caller tests, and Clippy if native slot permits.

## Integration notes

Task scope is not tenant authorization. Shared-workspace snapshot creation still has broader scope than a tenant service should permit; do not expose this runtime until that adapter, actor authority and workspace isolation are implemented and verified. Full make lint/make test remain parent-owned acceptance gates.

## Verification record (2026-10-03 UTC)

- Original-source RED reproduced missing-target workspace mutation, foreign index-lock deletion, and Git/PostgreSQL cross-task restore.
- Additional RED covered duplicate/sanitized-colliding IDs, crafted legacy alias and literal-path bypasses, both actual configured-store rewind false-success branches, and missing CI DB wiring.
- Final focused actual-source contract: 37 passed, 0 failed/ignored/filtered; strict Clippy passed.
- Actual builtin-agent test module compilation passed in 81 seconds; 32 store tests and 10 exact protocol/resume/rewind/checkpoint caller tests passed.
- CI fixture wiring regression and existing ignored-test CI contract passed; all verified source fingerprints remained unchanged during final execution.
- Independent static review found no remaining important issue. One post-review Git option interaction was caught by the stash tests: literal path handling is restricted to log operations; the complete final suite was rerun.
- Full repository make lint/make test, hosted CI, and tenant-runtime acceptance remain separate parent-owned gates.
