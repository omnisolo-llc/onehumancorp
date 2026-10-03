# Shipping and fulfillment integrity checkpoint

This checkpoint repairs existing webhook, label-recording and queue behavior.
It is not certification that paid shipping is safe end to end, nor evidence of a
live provider transaction or observed delivery.

## Recorded behavior

- Private shipping and fulfillment routes require an exact valid bearer and the
  current canonical owner/admin role. PostgreSQL startup proves that configured
  business and auth pools resolve the same actual database and relations, then
  uses the canonical pool. Matching tenant IDs, copied identity rows or equal
  URLs do not establish this relationship. An unavailable proof returns503.
- Local owner reads and mutations use the existing OwnerPgTransaction or
  OwnerSqliteTransaction boundary, which rechecks current authority at commit.
  Provider I/O holds no database locks. Shipping reauthorizes after the response;
  revoked authority after an external label response returns202 with its actual
  transaction ID and reconciliation required, without recording local success.
  This does not supply a durable purchase reservation or serialize external
  dispatch with revocation.

- Shippo uses its documented `shippo-auth-signature` header over timestamp,
  literal dot and exact raw body. Missing/blank secrets, malformed/duplicate
  fields, oversized bodies and timestamps outside ±300 seconds fail closed.
- DoorDash Drive uses the exact Authorization header configured at the provider.
  Owner bearer credentials, unsigned tenant headers and tenant fields in payloads
  cannot authenticate provider events or choose their account tenant.
- Provider scopes require explicit server-owned `SHIPPO_TENANT_ID`,
  `SHIPPO_ACCOUNT_NAMESPACE`, `SHIPPO_WEBHOOK_MODE` (`test` or `live`) and the
  corresponding `DOORDASH_*` values. Shippo additionally needs
  `SHIPPO_WEBHOOK_SECRET`; DoorDash needs `DOORDASH_WEBHOOK_AUTHORIZATION`.
  `SHIPPO_API_TOKEN` and webhook secret must belong to the configured account.
  Namespaces are stable, non-secret account identifiers configured by the
  operator. Never reuse one for a different account. No token/key is stored in
  the binding table.
- Forward migration1029 creates `delivery_provider_bindings` without changing or
  inferring identities for existing tasks. SQLite startup adds previously absent
  tracking columns and the equivalent binding table, without backfilling rows.
- A new confirmed Shippo transaction receipt and LABEL_CREATED are recorded in
  one local transaction. Actual transaction identity, mode, trusted label URL,
  optional carrier and tracking number are preserved. Label creation never
  changes the canonical order to shipped/fulfilled. A contradictory provider
  receipt is not success. Missing optional tracking/carrier stay unknown.
- A webhook must match tenant, provider, configured account namespace and mode.
  Shippo uses the actual transaction ID when present, with no contradictory known
  carrier/tracking; otherwise it requires one exact stored carrier/tracking pair.
  A receipt with initially missing tracking can be enriched only by an event that
  identifies its actual transaction. Ambiguous or unbound matches fail closed.
  DoorDash requires its explicitly stored external delivery identity. OHC order
  IDs are never provider-identity substitutes.
- The matching task, binding and canonical order are locked together. Replays,
  older/same-time events, canceled/fulfilled/returned orders and terminal delivery
  regressions do not mutate the projection. Failure-to-transit recovery remains
  possible. Only authenticated bound transit/delivery evidence projects an order
  as shipped/fulfilled.
- PostgreSQL queue reads join real tenant-scoped orders, customers, products and
  delivery tasks. Missing customer/mode data remains unknown; no sample orders
  are generated. Label receipts can be read through
  `GET /api/v1/shipping/label/{transaction_id}` with bearer authentication and the
  configured account scope. A404 does not prove that an external purchase never
  happened. The queue also exposes the stored label link when available.
- Fulfillment rate/label aliases reuse the authenticated shipping implementation.
  Provider webhooks are mounted after the global owner-authentication layers and
  rely on their own strict provider authentication. Manual readiness and handoff
  record actual local task state; handoff never claims delivery. Unconnected
  driver dispatch returns an explicit unsupported result.
- The UI displays confirmed statuses and errors, retains legitimate optional
  label fields, and removes special-order fixtures/fallback prices/tracking.
  Buying a label does not display Shipped. An uncertain purchase blocks another
  click in the current page and warns that reload is not proof of no purchase.

## Existing records and reconciliation

Existing tasks are preserved and intentionally unbound. A webhook for them
returns an explicit reconciliation-required conflict; configuring a current key
must never relabel historical rows as belonging to that key. Reconciliation must
retrieve the original transaction/delivery under the actual provider account,
compare its mode, order/customer evidence and tracking/carrier, then establish
one explicit mapping. This checkpoint has no automatic historical-mapping tool
or fabricated DoorDash task creator. Do not insert mappings from claimed order
IDs, guessed carriers or current credentials alone. Until verified reconciliation
is available, retain the old record and its blocked status.

## Required follow-ons and verification limits

1. Durable purchase admission/idempotency is not implemented here. Concurrent
   requests, process crashes, network uncertainty, cancellation/revocation around
   dispatch, or failed local recording after provider success still require a
   durable request/receipt engine before paid-shipping completion can be claimed.
   Unknown outcomes are reported as reconciliation-required; never automatically
   repeat the provider POST. The page-local retry block is not restart/cross-tab
   protection. Historical provider receipts are not created by this migration.
2. Real per-order shipping addresses, bounded expiring quote/cost authority and
   account-bound provider rate evidence remain required. Existing global
   destination configuration is not a per-customer address model.
3. PostgreSQL owns webhook/queue/manual-action runtime in this checkpoint.
   Canonical SQLite label purchase, recording/readback and schema repair are
   tested, including the production startup's shared-pool configuration. An
   independently opened SQLite business pool has no supported identity proof and
   fails closed. SQLite fulfillment queue/manual operations return503 explicitly;
   full SQLite fulfillment runtime remains unimplemented.
4. Offline manual intents can still be queued truthfully, but the existing
   fulfillment response is not the per-operation outcome protocol expected by
   SyncManager. Offline acknowledgement/authority remains a separate repair.
5. The focused probe imports whole changed production modules and real route
   boundaries, actual databases and loopback provider HTTP. It does not start the
   complete application or certify all historical migrations, all `make lint` /
   `make test` gates, provider sandbox behavior or real delivery.

Official protocol references checked 2026-10-03:
- https://docs.goshippo.com/tracking/webhook-security
- https://docs.goshippo.com/tracking/webhooks
- https://developer.doordash.com/en-US/docs/drive/how_to/webhooks/
- https://developer.doordash.com/en-US/docs/drive/reference/webhooks/
