# Shipping and fulfillment integrity checkpoint

**Goal:** Authenticate provider events, project only explicitly bound deliveries, and report persisted fulfillment evidence without inventing shipments or customers.

**Approved scope:** Repair existing mounted Shippo/DoorDash webhook, shipping-label, and fulfillment-queue paths. No live provider calls, payment admission engine, deployment, or historical account inference. Tests use synthetic credentials, disposable PostgreSQL/SQLite and loopback HTTP only.

## Design

- Official Shippo `shippo-auth-signature` authenticates timestamp, dot and raw bytes with SHA-256 HMAC. Reject absent/blank configuration, duplicate/malformed signature fields and timestamps outside five minutes. DoorDash Drive authenticates the exact configured Authorization header. Tenant, stable account namespace and mode come only from trusted server configuration.
- Forward migration1029 creates immutable explicit provider bindings; existing delivery records remain unbound. A new real Shippo SUCCESS/VALID transaction receipt can establish a binding in the same local transaction as LABEL_CREATED. It cannot fulfill an order. No provider account, carrier, tracking, transaction or mode values are invented.
- Tracking admission requires tenant/provider/account/mode plus the actual transaction ID, or a unique recorded carrier/tracking pair when transaction is absent. DoorDash requires its explicit external delivery identity. An order ID never stands in for a provider ID. Missing/ambiguous mappings, stale/replayed events and canceled/terminal orders do not mutate fulfillment.
- Real database reads replace queue fixtures. Known fulfillment modes are derived only from recorded provider associations. Unsupported actions return actionable errors rather than pretending to dispatch drivers or create labels. Existing shipping aliases use the same authenticated route implementation.

## Implementation and checks

- [x] Run RED regression cases against current complete production modules and actual SQL/loopback HTTP.
- [x] Add official signature parser and strict provider payload parsing; test malformed, replay-window, duplicate and optional fields.
- [x] Add forward binding schema and SQLite equivalent; no backfill. Test old data survives and stays unbound.
- [x] Implement atomic label/binding storage and tracking admission with tenant/row locks and monotonic event dates. Test cross-tenant/provider/account/mode, missing/duplicate mapping, replay/out-of-order, cancel/terminal and failure recovery.
- [x] Replace queue fixtures, consolidate authenticated aliases and correct misleading action results. Test real readback and missing auth.
- [x] Correct fulfillment UI errors/status presentation without optimistic completion; run UI tests/type checks.
- [x] Run whole-source focused tests/Clippy, actual parent mount checks, source-fingerprint verification and independent review. Full `make lint`/`make test` remain required aggregate acceptance.

## Required follow-on

Provider purchase admission, idempotency, durable unknown-outcome recovery and canonical owner/token authorization are not solved by this binding table. A provider purchase followed by failed persistence remains reconciliation-required and must not be described as a safely retryable failure. Global shipping destination configuration is not a real per-order address model. No provider-sandbox or real delivery certification is claimed.

Official references checked 2026-10-03:
- https://docs.goshippo.com/tracking/webhook-security
- https://docs.goshippo.com/tracking/webhooks
- https://developer.doordash.com/en-US/docs/drive/how_to/webhooks/

## Verification recorded at this checkpoint

- Initial whole-source RED: 9 existing passed, 7 regression cases failed for the intended defects.
- Outer owner-auth boundary RED: 36 passed, provider-only request failed401; corrected public mount then passed.
- Final source-bound CI gate: 37 passed, 0 failed/ignored/filtered; strict all-targets Clippy passed.
- Maintained Next: 13 focused UI tests passed, full typecheck passed, focused ESLint passed.
- Migration identity contract, YAML/Python syntax and diff checks passed.
- Independent Astra Ultra read-only review found no remaining blocking defect within this scope.
- Full application startup, full historical migration execution, aggregate make gates, offline acknowledgements, canonical store/owner authority and restart-safe external purchase admission remain unverified or required follow-ons as described in docs/development/shipping-integrity.md.
