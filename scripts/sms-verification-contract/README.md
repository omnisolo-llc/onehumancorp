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

The gate requires at least 29 executed tests and each of six named PostgreSQL tests. It rejects filtered, ignored, missing and failed tests, and checks the source fingerprint after execution. It uses a real non-superuser/non-BYPASSRLS application role, applies migration 1039 verbatim, and tests row-lock barriers against concurrent opt-out, demotion and deactivation. Two source-extracted billing regressions exercise actual dunning lookup/update/notifier code using SQLite without Redis prerequisites. Only unused state-wrapper fields are omitted; persistence code is unchanged.

An event-level receipt stores the original generated notification and full original audience, including an empty audience. Empty audiences are terminal no-ops, so business webhook acknowledgement does not become a retry error. Replays cannot subscribe future opt-ins or replace the first message. Volatile department dispatch never proves an order was committed and now returns explicit SMS-unavailable metadata until a persistent order receipt exists. New OTP requests supersede old generations; confirmation cannot replace an already verified phone or reset its saved preferences.
