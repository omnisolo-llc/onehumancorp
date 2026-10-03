# Site Publication Implementation Plan

> For implementation: use the executing-plans workflow, with independent source review after each bounded stage. Continue existing CI monitoring in parallel.

**Goal:** Publish an explicitly reviewed owner site to a real public route with durable status and no draft or cross-tenant disclosure.

**Architecture:** A publication receipt is also its durable pending/leased job. A transaction saves the reviewed draft and immutable snapshot, while an operation UUID prevents duplicate acceptance. A deterministic worker commits a fenced public version; public reads validate current authority and membership before using rendered-byte caches.

**Tech stack:** Rust/Axum/SQLx PostgreSQL, existing auth/common crates, Next TypeScript, source-bound PostgreSQL harnesses and hosted Playwright. No dependency upgrades or live provider calls.

**Spec:** `docs/development/site-publication-receipts-design.md`

## Global constraints

- No synthetic claims or legacy `published_at` authorization
- Explicit signed OWNER/ADMIN action, checked against the current active user and tenant
- Editing drafts never implicitly publishes them
- Renderer performs no model, payment, DNS or CDN deployment calls
- Snapshot/product eligibility and current publication authority are checked before every public cache read
- Intermediaries cannot reuse a response after an authority change; use `no-store` until purge/edge authorization has its own verified contract
- Maintain the existing string auth tenant versus UUID builder-tenant mapping explicitly; never silently derive another user's authority
- Keep all failed/skipped/unrun checks separate from passing evidence

## Review focus

- Two concurrent identical submissions return one receipt/site; conflicting reuse changes no rows
- Owner removal or tenant reassignment blocks a waiting worker and a public read, including cached content
- An expired older worker cannot overwrite a newer publication
- Literal markup, URL schemes and `</script>` strings stay inert in the final public document
- Reload/account switching preserves the correct operation and cannot show another owner's receipt or resend an unknown request

## Stage 1: Atomic receipt and reviewed snapshot

Files: new `src/server/migrations/1019_site_publication_receipts.sql`, `src/server/builder/publication_store.rs`, `scripts/site-publication/{Cargo.toml,prepare.py,test.rs,run.sh,verify_lock.py,README.md}`, narrow `builder/mod.rs` export.

Interface: `PublicationActor { user_id: String, tenant_id: String }`; `SiteSnapshot { domain: Option<String>, pages: Vec<PublishedPage> }`; `PublishedPage { path: String, title: String, seo_metadata: Value, blocks: Vec<PublishedBlock> }`; `PublishedBlock { block_type: String, content: Value, sort_order: i32 }`. `submit_publication(pool, actor, operation_id: Uuid, site_id: Option<Uuid>, snapshot) -> Result<PublicationReceipt, PublicationError>`. A receipt exposes a server-generated publication UUID, caller operation UUID, site, tenant-bound status, version and digest; it exposes a public URL only after publication.

- [ ] Add failing real-PG tests for nonexistent receipt storage, duplicate/concurrent operation reuse, changed snapshot conflict, inactive/foreign actor, foreign product ID and deferred commit rejection
- [ ] Implement additive receipt/job storage and per-site monotonically increasing publication version; canonicalize and bound snapshot input before any write
- [ ] Save site/pages/blocks and receipt in one transaction; explicitly switch the existing tenant context only for the schema's string/UUID representation, and test both forms
- [ ] Preserve the exact immutable snapshot/product membership; leave draft edits and legacy timestamps without a public grant
- [ ] Run the whole-source focused harness, strict Clippy, migration identity checks; freeze source hashes and obtain review before committing

## Stage 2: Deterministic rendering and fenced worker

Files: new `src/server/builder/publication_render.rs`, `publication_worker.rs`; narrow extraction from `builder/edge.rs`; extend the same harness.

Interfaces: `render_snapshot(snapshot: &SiteSnapshot) -> Result<RenderedSite, PublicationError>` where `RenderedSite` contains path-to-HTML bytes and their digest. `claim_publication(pool, now) -> Result<Option<PublicationClaim>, PublicationError>`; `finish_publication(pool, claim, rendered) -> Result<PublicationReceipt, PublicationError>`. Claims carry operation ID, lease token and site version. `run_publication_worker(pool, shutdown)` is the mounted consumer.

- [ ] Add renderer negatives for text/attributes, disallowed URL schemes, JSON-LD script termination and malformed block data
- [ ] Extract a pure renderer using actual maintained block rendering; preserve configured content while keeping unsupported actions explicitly unavailable
- [ ] Add failing real-PG tests for restart recovery, lease expiry, late worker fencing, newer-version precedence and owner revocation between claim and commit
- [ ] Implement provider-free rendering and an idempotent fenced commit; check current active owner/tenant in the final transaction
- [ ] Mount the worker only in the existing supported PostgreSQL runtime, preserving other workers; run all focused cases and review the complete startup hunk

## Stage 3: Authenticated actions and guarded public projection

Files: `builder/api.rs`, new `builder/publication_http.rs`, narrow `lib.rs` mounts, `api/storefront_delivery.rs`; same harness plus source-wiring guards.

Interfaces: authenticated POST publication uses `{operation_id, site_id?, snapshot}`; authenticated GET receipt uses operation UUID and current owner. A dedicated public GET resolves a stable server-generated site UUID/path to its current committed publication snapshot; authenticated recovery instead uses the owner-scoped operation UUID. Existing product GET requires that product's membership in a current eligible published snapshot.

- [ ] Add actual-router red cases proving synthetic claims, implicit edit publication and unauthenticated mutation are rejected
- [ ] Remove synthetic fallback and automatic publish calls from ordinary edits; preserve owned draft editing and authenticated previews
- [ ] Route explicit publication through Stage1 and return a receipt reflecting only committed state
- [ ] Add public read tests for unpublished/legacy/missing/revoked/foreign references and stale internal cache entries; verify invalidation and all mutations stay protected
- [ ] Read the eligible snapshot and current authority before internal cache access; apply safe response headers and correct404/error behavior
- [ ] Register the source-bound gate in mandatory PostgreSQL CI and retain nonzero discovery/log evidence; review before checkpoint

## Stage 4: Builder status and recovery UI

Files: `src/ui/next/src/app/builder/ownedDraft.ts`, its tests, `website-builder/page.tsx`, `storefront-builder/page.tsx`, `builder/page.tsx`, associated component tests and authenticated BFF contracts.

- [ ] Add failing tests for one durable operation UUID, real pending/published receipt transitions, reload recovery and conflicting payload reuse
- [ ] Pass the existing locally persisted operation UUID to the server; hold unknown mutations and use read-only receipt recovery without automatic POST retry
- [ ] Migrate all three active publication callers; remove the legacy caller's invented cloud URL and unsupported default referral offer
- [ ] Render saved/pending/published/reconciliation separately; display only the receipt's verified public URL
- [ ] Add owner-retirement and delayed-body regressions, including a returning owner and a later edited draft
- [ ] Run focused tests, full Next types/lint and a combined one-worker frontend aggregate when the shared window is available; review before checkpoint

## Stage 5: Real owner-to-public browser journey

Files: `src/e2e/playwright/edge_seo_agent.spec.ts`, focused publication E2E and release documentation.

- [ ] Keep real product creation, observed owner identity, exact cents/JSON-LD, cache invalidation and cross-tenant denial assertions
- [ ] Select the actual product in the reviewed publication before anonymous access; await the real durable published receipt and open its URL
- [ ] Verify later draft edits do not change public content until an explicit new publication; verify revocation blocks cached reads
- [ ] Execute the complete hosted suite on the exact pushed head, record all failures and total runtime, and retain artifacts durably
- [ ] Do not mark site publication or release readiness complete until the mounted worker/public route/browser path all pass
