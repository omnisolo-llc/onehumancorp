# Persistent SMS verification contracts

No live SMS is sent by these tests. The Rust tests import the production SMS module and substitute only its Twilio trait boundary. PostgreSQL/SQLite authorization uses the configured canonical identity store. MySQL SMS is explicitly unavailable until it has a supported commit-authority implementation.

Commands:

- `node --test scripts/sms-verification.test.mjs`: production SQLite schema and exact query constraints, plus source wiring invariants.
- `python3 scripts/sms-verification-contract/schema_test.py`: direct SQLite SQL checks.
- `cargo test --locked -p omnisolo --lib sms_settings::tests`: real mounted SMS handlers, selected identity/database, negative OTP cases, replay, identity isolation, provider outcomes, current authority and dispatch receipts. Requires the native Rust prerequisites.
- `cd src/ui/next && npm test -- src/app/settings/sms.test.tsx`: frontend receipt and identity-boundary tests.

Provider acceptance is not delivery. An unknown provider result or lost acknowledgement never authorizes another send. Migration 1039 adds private PostgreSQL records with RLS; the normal portable initializer installs the same data schema on SQLite. Legacy global phone settings are not accepted as proof.

The Python SQL contract uses named numeric bindings to mirror SQLx's SQLite `$NNN` handling ([SQLx driver](https://github.com/launchbadge/sqlx/blob/v0.8.6/sqlx-sqlite/src/arguments.rs#L72-L108)). SQL checks do not replace compiling/executing the Rust authority tests, PostgreSQL RLS checks, browser QA or provider-sandbox verification.

## Required hosted gate

`OHC_SMS_TEST_DATABASE_URL` must name the explicitly owned loopback `ohc_sms_test` PostgreSQL database. Missing database access or setup failure is an error, never a passing skip. Run:

- `bash scripts/sms-verification-contract/fetch.sh`
- `python3 scripts/focused_ci_gate.py sms-verification-contract`

The gate requires at least 49 executed tests and every named PostgreSQL/worker regression. It rejects filtered, ignored, missing and failed tests, and checks the source fingerprint after execution. It uses a real non-superuser/non-BYPASSRLS application role, applies migration 1039 verbatim, and tests row-lock barriers against concurrent opt-out, demotion and deactivation. Two source-extracted billing regressions exercise actual dunning lookup/update/notifier code using SQLite without Redis prerequisites. Only unused state-wrapper fields are omitted; persistence code is unchanged.

An event-level receipt stores the original generated notification and full original audience, including an empty audience. Empty audiences are terminal no-ops, so business webhook acknowledgement does not become a retry error. Replays cannot subscribe future opt-ins or replace the first message. Volatile department dispatch never proves an order was committed; a webhook can read notification status only by referencing an existing canonical order receipt. New OTP requests supersede old generations; confirmation cannot replace an already verified phone or reset its saved preferences.

## Durable New Orders follow-on (migration 1043)

Canonical `orders` INSERTs now admit an event and its exact verified, active owner/admin audience into the existing SMS event/dispatch outbox in the same transaction. PostgreSQL uses an invoker trigger with current identity/role/proof/preference locks; SQLite uses its transaction write lock. The content is fixed server-authored text with a checked SHA-256 digest. A rollback rolls back the order and notification together. Existing historical orders are never backfilled, and order updates/duplicate inserts cannot admit another event.

The mounted `SmsService` worker discovers routing identities using the existing background discovery role and ends that transaction before any tenant content or provider effect. It rechecks and locks the tenant-owned persisted order plus the exact current actor/role/phone/proof/opt-in through the durable `sending` claim. Provider I/O occurs only after commit. Worker restarts retry `prepared` work; `sending`, `unknown`, `rejected` and accepted claims are never resent automatically. One failed recipient does not prevent processing other frozen recipients. A transient pre-provider outage leaves prepared work for a later poll. SQLite REPLACE replays cannot reset accepted receipts or rebuild an event audience. Large frozen audiences are processed in pages of 100, with terminal status based on the entire event. A durable 30-second retry schedule moves failed events behind untouched work, so one tenant cannot starve others.

The New Orders control becomes available only when the selected provider is configured, transactional admission is installed and the mounted worker has recently completed an outbox poll. During an outage a saved subscription stays visible and can be disabled. A department webhook can optionally include `order_id` to read its existing tenant-owned notification receipt; that read never creates an order or authorizes a send. Legacy event-only webhooks still cannot prove an order exists.

Additional local checks:

- `python3 scripts/sms-verification-contract/order_schema_test.py`: real SQLite transactions, file reopen, duplicate inserts, failed admission, historical/no-audience behavior and frozen tenant audience.
- `OHC_PGLITE_MODULE=/absolute/path/to/already/installed/pglite/dist/index.js node scripts/sms-verification-contract/order_pg_probe.mjs`: optional PostgreSQL-WASM execution of migration 1043 and exact production SQL, including forced RLS under an unprivileged fixture role. It does not replace native PostgreSQL multi-connection locking or Rust compilation.

The required native gate now executes at least 49 tests, with named new-order transaction, authority-race, source-lock, actual worker startup, restart and provider-outcome regressions. This gate must run with the documented owned PostgreSQL database and native dependencies; missing prerequisites are failures, never successful skips.

Scope and external configuration: no provider credentials, production roles or live accounts are created or changed by this patch. PostgreSQL worker discovery requires the application's already authorized `ohc_bypassrls` role; this patch adds no grants. Twilio configuration is still required, and provider acceptance remains distinct from delivery. Held unknown outcomes require provider reconciliation. The notification identity is the canonical order ID: this does not repair upstream business workflows that create separate orders for the same external checkout; their business-order idempotency remains a separate concern.
