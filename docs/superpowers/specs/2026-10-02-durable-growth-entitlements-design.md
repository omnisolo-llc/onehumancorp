# Durable growth entitlements and observed activity

## Goal and approved boundaries

Replace simulated growth rewards and unverified measurements with server-owned,
source-linked behavior. The safety checkpoint is `ff357ad` plus `1af40b3`; its
unavailable responses are containment, not completed trial or measurement features.
No deployment, live purchase, real account grant/revocation, commercial program
activation, credential change or provider request is part of this repair.

Existing paid/base `tenants.plan_tier` and `tier` values stay unchanged. Existing
Pro records with uncertain historical origin need reconciliation, not automatic
downgrade. A share action and a client-supplied conversion ID are never evidence
of a qualifying event. Programs are disabled by default and controlled by
platform/operator configuration, never by tenant owners granting themselves Pro.

## Source and data contracts

A configured program identifies a version, authoritative event type, grant tier,
duration, eligibility window and abuse caps. Verified registration is supported
as registration, never as a paid conversion: registration consumes a verified
registration ticket and creates a unique-email user in one existing transaction
(`auth/seaorm_store.rs::consume_ticket_and_create_user`). Eligibility must resolve
that persisted source inside the server; the caller cannot supply its truth.

Persist attribution before awarding a reward, reject self-referrals and duplicate
beneficiaries, and link attribution to the authenticated registering owner and
real referral record. Keep source event, program version, conversion/acceptance,
grant and ledger identities durable and unique. The conversion/acceptance, grant
and ledger write commit atomically; any failure rolls back all of them. Duplicate
or concurrent requests return the original receipt without extending its expiry.

Grant records overlay a base plan and contain starts/expires/revoked timestamps.
Every server-side effective-plan gate resolves the current overlay. A 300-second
process cache or 24-hour Redis cache must never preserve expired or revoked grant
access. Returning unknown/unavailable during a failed verification does not
rewrite the base plan. The existing customer/product `entitlements` table
(migration 1008) is a different scope and must not be mistaken for tenant-plan
ownership.

Observed activity responses identify tenant, observation time, bounded time
window, source and literal counting rules. Persisted records marked completed
are recorded states, not proof of revenue, customer acceptance or hours saved.
Unknown data differs from a verified zero. Measured savings remain unavailable
until a real baseline and measurement provenance exist; no fixed multiplier is
introduced. Referral revenue/rewards require actual ledger records rather than
conversion-count multipliers.

## Billing and transport prerequisites

The mounted gRPC interceptor verifies peer identity in cloud mode and replaces
claimed metadata. The growth service must consume verified request identity and
tests must reject metadata-only requests on the real mounted boundary. Standalone
loopback authentication remains a separately explicit mode.

Paid-conversion programs remain unavailable until actual checkout/customer
bindings and durable authenticated provider receipts exist. The current portal
customer fallback `cus_{tenant}` is not a provider identifier. The Stripe webhook
middleware's immediate acknowledgement before asynchronous persistence cannot be
used as grant evidence. Repair those defects without creating live sessions or
calling a provider during validation.

## Issue matrix and staged acceptance

| Issue | Source | Required result | Stage |
|---|---|---|---|
| Permanent share-to-Pro mutation | `api/growth.rs`, `services/growth/service.rs` | No fake grant; actual expiring/idempotent configured grants | Safety shipped; durable open |
| Hardcoded hours and inferred duration multipliers | Next savings route, old growth handler | Source-linked activity counts; hours unknown without measurement | Safety shipped; counts open |
| Missing pricing values shown as 0/∞ or Free | Next pricing page/card | Explicit unknown; zero and unlimited only from valid fields | Next |
| Local display name used as tenant; fake `ref123` link | Next referral builder | Verified identity, persisted referral link and honest preview | Next |
| Revenue/rewards inferred from conversion counts | HTTP and gRPC growth stats | Actual scoped records; unsupported values unknown | Next |
| Fixed cash/free-Pro/lifetime milestone promises | Growth milestone endpoint | Configured eligibility and durable actual grants only | Next |
| gRPC identity read from metadata only | Growth service | Verified transport identity required for authority | Durable gate |
| No attribution, source receipt, expiry or revocation | Growth/billing plan resolution | Atomic source→attribution→grant→ledger with caps | Durable gate |
| Existing tier caches outlive grants | `pricing/rate_limit.rs`, billing plan cache | No stale expired/revoked entitlement | Durable gate |
| Invoice download fabricates customer identity and a success receipt without a URL | `api/billing_api.rs::download_invoice_handler` | Verified provider customer and actual invoice URL, otherwise explicit unavailable | Billing gate |
| Portal uses fabricated Stripe customer identity | `api/billing_api.rs` | Persisted verified provider customer only | Billing gate |
| Webhook acknowledges before durable result | `api/billing_webhook.rs` | Persist before acknowledgement; retry-safe receipt | Billing gate |
| Legacy malformed SPIFFE test fixtures/conditional DB tests | Growth service tests | Valid identities and mandatory isolated-DB assertions | Test gate |

Use disposable loopback PostgreSQL and SQLite databases. Test two tenants,
unknown/zero data, invalid/missing evidence, disabled/unconfigured programs,
self-referral, duplicate/concurrent delivery, expiry, revocation, restart,
rollback on ledger failure, paid-tier preservation and forged transport identity.
No fixture, mock grant or fake provider ID may enter production code.
