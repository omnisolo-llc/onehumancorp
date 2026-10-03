# Durable Growth Entitlements Implementation Plan

> For agentic workers: execute the approved staged plan with test-driven development and independent review. No live billing operations.

**Goal:** Replace the remaining simulated entitlement, pricing and referral data with durable authoritative behavior.

**Architecture:** Keep base plans immutable and resolve grants as an expiring, revocable overlay. Reuse the real registration, authentication, database and ledger boundaries; programs are explicitly operator-configured and disabled by default. Observed counts report literal persisted states separately from business outcomes and time-saving estimates.

**Tech Stack:** Rust, SQLx/PostgreSQL/SQLite, Axum/Tonic, Next/React, Vitest and Playwright.

**Spec:** `docs/superpowers/specs/2026-10-02-durable-growth-entitlements-design.md`.

## Global constraints

- Do not change live accounts, billing credentials, subscriptions or commercial programs.
- Preserve existing paid/base plans and uncertain legacy Pro records.
- No client click/share or user-supplied success flag is authoritative evidence.
- No concurrent large native graph; coordinate one cached-only Cargo slot, one build job and a 900 MiB free-disk floor.
- `make lint` and `make test` remain final acceptance; focused gates never replace them.

## Review focus

- Missing numeric fields must never become verified zero or unlimited.
- Stale owner/plan responses cannot enable another account's controls.
- Concurrent claims cannot duplicate attribution, extend grants or bypass caps.
- Expiry/revocation must be enforced despite caches and restart.
- A signed provider notification is not complete until its durable result commits.

## Stage 1: Truthful account and referral presentation

Files: Next pricing page/card/tests; referral builder and its rendered/browser contracts; HTTP/gRPC referral stats and milestones.

- [ ] Write failing rendered tests for missing/invalid plan/usage fields, explicit zero and unlimited, and retry recovery.
- [ ] Parse valid plan and metric fields; render unknown independently and disable plan-dependent actions when identity/plan is unverified.
- [ ] Replace local-identity/fake referral links with actual persisted signed-owner link creation and encoded embed data; retain explicit preview status.
- [ ] Replace fabricated reward/revenue fields with source-linked scoped records or explicit unavailable values.
- [ ] Run complete affected rendered/route/browser-contract/type/lint checks; commit a self-contained stage.

## Stage 2: Authoritative grants and attribution storage

Files: focused tenant-entitlement store/schema, existing registration/referral source readers and ledger integration; disposable PostgreSQL/SQLite contract harness.

Interfaces: `resolve_effective_plan(owner, now)` returns verified base/current plan and active grant receipts; `claim_configured_event(owner, program_id, source_id, request_id)` resolves source/attribution and atomically returns the stable existing/new grant receipt; `revoke_grant(platform_authority, grant_id)` records revocation without rewriting base plan.

- [ ] Test disabled programs, source provenance, tenant mismatch and self-referral before implementation.
- [ ] Add versioned programs, immutable attribution/source identity and expiring/revocable grants with database uniqueness and abuse caps.
- [ ] Use one transaction for attribution/conversion, grant and ledger; force a ledger failure to prove rollback.
- [ ] Test concurrent duplicates, conflicting idempotency keys, cap exhaustion, restart and exact original expiry preservation.
- [ ] Test PostgreSQL and SQLite migrations against actual canonical tenant/user/ledger schemas; commit verified storage and source binding.

## Stage 3: Effective-plan and transport enforcement

Files: pricing rate limiter, billing current-plan handlers, every direct Pro-feature read, growth HTTP/gRPC boundaries and Next useProPlan.

- [ ] Reproduce expired/revoked stale-cache access and metadata-only gRPC identity acceptance in real boundary tests.
- [ ] Resolve the server-owned overlay at every gate; keep base plan values untouched and avoid caching grant authority past revocation/expiry.
- [ ] Return source-bound availability/grant receipts and make UI transitions depend on verified current account state.
- [ ] Test Free/Starter/Pro/Business base preservation, late prior-owner responses and nonterminal receipts; commit the mounted flow.

## Stage 4: Observed activity and billing evidence

Files: growth activity/metrics readers, savings widget, pricing summaries, billing portal creation and webhook receipt handling.

- [ ] Test real empty/scoped/time-windowed activity records and explicit unavailable dependencies.
- [ ] Return literal counts with observation/source information; leave measured hours unknown without baseline provenance.
- [ ] Require persisted provider customer IDs for portal creation; reject fabricated/fallback IDs without provider I/O.
- [ ] Persist authenticated webhook identity and processing outcome before acknowledgement, and test duplicate/restart/failure replay with local fixtures.
- [ ] Keep paid-conversion grants disabled until the actual provider receipt and customer binding path is verified.

## Stage 5: Integration acceptance

- [ ] Independent source/security review and issue-matrix reconciliation.
- [ ] Complete Next/Node suites, strict type/lint, source-bound native database/transport contracts and no skipped fixture tests.
- [ ] Fresh full hosted `make lint`/`make test` and all real-stack browser shards against the exact integrated commit.
- [ ] Report remaining external prerequisites and measured-savings/paid-conversion gaps explicitly; never call containment full functionality.
