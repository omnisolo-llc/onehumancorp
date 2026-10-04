# Field Tenant Boundaries Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Make the mounted field operations authenticate current owners, isolate records by tenant, reject stale/terminal mutations, and report only committed persistence.

**Architecture:** Reuse signed bearer middleware and canonical commit_authority. Read existing appointments and preserve optimizer/delay preparation; persist authorized route/status updates atomically with exact operation replay and observed record timestamps.

**Tech Stack:** Rust, Axum, PostgreSQL/SQLx, current server_auth.

**Spec:** Parent repair assignment and current AGENTS.md/RESEARCH.md; concrete mounted findings at baseline e90530e30.

## Global Constraints
- Preserve existing features and standalone compatibility; no live business actions or dispatch.
- No changes to shared index or publication; parent owns integration and Cargo scheduling.
- Identity and business writes must use the same proven canonical pool.
- Never claim full acceptance from a focused contract.

## Review Focus
- Forged headers and missing/expired/revoked/downgraded bearer identity fail before effects.
- Cross-tenant identifiers and malformed legacy relationships never expose or mutate foreign records.
- Concurrent same-version edits, terminal reopening, and changed idempotency payloads conflict.
- Database/commit errors cannot yield success or fabricated completion events.
- Preview leaves storage unchanged; accepted route persistence saves actual owned records atomically.

## Task 1: Mounted request admission
- [x] Write and run five anonymous HTTP regression cases against whole imported handlers and exact field mounts (RED).
- [x] Reuse strict bearer middleware/current owner verification in field routers.
- [x] Run signed identity/current role/tenant mismatch cases (GREEN).

## Task 2: Fenced field persistence
- [x] Add PostgreSQL cases for tenant ownership, observed timestamp CAS, terminal states, receipt replay/conflict, and rollback before implementation.
- [x] Add minimal shared records/authority helpers and additive operation receipts; return persisted rows.
- [x] Bind canonical field pool in exact mounted routing seam using existing database identity proof.
- [x] Keep optimizer preview and delay proposal explicitly uncommitted; commit requested routes from owned persisted appointment snapshots.
- [x] Execute real restricted-role PostgreSQL tests and recheck source fingerprint.

## Task 3: Integration handoff
- [x] Document matched frontend expected_updated_at and truthful receipt/preview contract for parent caller repair.
- [x] Run formatting, lock consistency, focused tests, and review exact diff.
- [x] Report precise verification and outstanding whole-repo gates; parent publishes.

## Review decisions and remaining acceptance

Independent review found three Important issues: foreign customer-ID projection,
legacy completed-case duplicate task, and terminal route mapping. Each was
reproduced RED and repaired. Caller review also reproduced historical route order
mixing after a second save; reload now selects the newest current-date prepared or
active route. A final recorder regression preserves the flat status-event interface
and proves no appointment event on replay or rollback.

The frozen focused checkpoint executes41 field/cache cases and preserves11
appointment-read cases, with strict Clippy for both harnesses. It does not certify
whole-application compilation, make lint/test, browser journeys, in-flight token
revocation/expiry in this field harness, or routing-job receipt contention. Those
remain explicit integration/follow-on checks. The generic canonical commit fence
is reused without weakening it. Parent owns publication and full acceptance.

Ruling: the global route relationship cache was replaced with two scoped reads in
one transaction, because stale/global cache entries cannot prove current tenant
ownership; the public read interface and mobile projection remain. The cost is
one bounded database read per request rather than a process-global cached answer.

Ruling: existing legacy/malformed records are preserved and fail closed or project
only owned relations; no destructive cleanup, default staff or geographic origin
is introduced. Current PostgreSQL field endpoints retain their prior backend
scope; this checkpoint does not claim new SQLite field fulfillment support.
