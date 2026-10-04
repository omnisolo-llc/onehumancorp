# Queued catalog dispatch: canonical local commit

## Bounded repair

This is a separate follow-on to frozen feed-dispatch tree `79170c9217c540323a1c2562da7ef6159785f962`. It repairs only queued `create_product` local persistence. No live provider/customer work, privilege workaround, RLS disabling, deployment or publication is involved.

The baseline durably claimed an attempt and released its tenant/owner/token context before invoking `catalog::handle_create_product` on a bare pool. A normal restricted PostgreSQL role could not insert under the actual products policy. On a privileged local fixture, a token revoked after the claim but before the product write still allowed the product to commit. Both failures were reproduced before implementation.

## Authority and transaction boundary

- Startup binds the catalog capability after constructing the real canonical authentication Store. Existing `canonical_pg_data_pool` verifies the configured business and canonical identity namespaces/objects, including products, feed items, decisions and queue. The capability stores the canonical pool; no fallback to a privileged pool exists.
- The existing worker constructor and public pool-based catalog handler remain available. An unbound worker holds catalog work without a product mutation. Failed catalog binding does not disable unrelated generic jobs or remove their existing claim/current-owner checks.
- The durable `ATTEMPTING` fence still commits before execution. The local branch then uses one fresh READ COMMITTED transaction, transaction-local tenant context and the canonical pool. Blank, padded and system tenant identities fail closed regardless of standalone configuration.
- Current active owner roles, approved decision/item, exact queue/attempt identity and immutable payload are rechecked. Owner, action, item, receipt and queue locks follow the existing order. The actual catalog insert is shared with the legacy handler through a connection-capable executor function.
- Product, `DISPATCH_RETURNED` receipt with product UUID, return timestamp and queue status commit together. Immediately before COMMIT, the transaction takes the existing canonical token-revocation fence and checks current owner/token state again. Owner row locks and the token fence remain held through deferred database checks and the actual commit.
- A revocation committed while the INSERT is blocked causes rollback. A canonical owner/token change that arrives after the final commit fence waits until the local transaction finishes. Natural bearer expiry does not cancel previously admitted work; an explicit revocation row does, even after its expiry timestamp.
- Local success does not call the generic separate acknowledgement path. Failed/ambiguous commit recovery preserves an already committed local receipt and product UUID. Restart/redelivery never performs a second insert. Failed local attempts remain held for reconciliation under the existing single-attempt semantics.

## Verification scope

The focused harness imports the actual production handler, worker, local authority module, canonical authentication, migrations, queue and decision code. It compiles the exact startup block from `lib.rs`, checks actual module registration and imports exact production pool-reset hooks. No test-only persistence or authorization implementation substitutes for these paths.

The corrected inventory is 57 executed tests: 47 explicit PostgreSQL/HTTP/queue contracts, the inherited PostgreSQL repository lifecycle, five SQLite cases, two worker/router cases, one source contract, and one presentation-boundary case. The runner creates a unique schema in its validated disposable database, binds `OMNISOLO_DATABASE_URL` to it and requires the inherited lifecycle test's durable APPROVED row. A silent connection failure or omitted lifecycle execution makes the runner fail.

Regression coverage includes production products RLS under a NOSUPERUSER/NOBYPASSRLS role without bypass-role membership; foreign tenant/payload isolation and context reset; unavailable/split canonical authority; owner role loss/deactivation/foreign role and approval withdrawal after claim; explicit revocation during INSERT; natural expiry; atomic receipt/deferred-commit failures; concurrent/duplicate/restarted dispatch; and authority changes waiting behind an actual deferred COMMIT. A controlled local incident test covers unchanged generic effect/acknowledgement behavior when catalog binding fails or a generic acknowledgement fails.

The lost-acknowledgement test materializes a committed product/receipt and invokes error/restart recovery as if the caller lost its acknowledgement. It does not inject a network disconnect into PostgreSQL COMMIT. Global queue dequeue/recovery still uses its existing system role; restricted-role assertions cover `process_job` and local catalog writes. Standalone feed behavior is preserved, not expanded into SQLite dispatch. The legacy bare-pool catalog API and unrelated provider handlers retain their earlier contracts and are not newly certified for tenant authority or delivery.

Exact frozen source hashes, commands, test results and full acceptance limitations accompany the patch. Focused passes do not certify `make lint`, `make test`, the whole server, desktop/browser behavior or external fulfillment.

## Inherited lifecycle accounting correction

The initial frozen catalog artifact `e7e266dbfacc7a8a7e8581366e8ca6f419743027` reported 56 passes but explicitly disclosed that `repository::tests::test_agent_feed_repo_lifecycle` returned early. Its prerequisite is a reachable PostgreSQL `OMNISOLO_DATABASE_URL` with the feed repository schema; the prior runner supplied `sqlite::memory:` instead. This historical artifact remains unchanged.

Supplying the owned schema exposed a real dormant test failure at `mobile_item.context_payload.is_none()`. The current repository intentionally returns full canonical rows for either presentation hint. The existing `api/agent_feed.rs` and both `api/work_triage.rs` mobile branches separately project those rows into compact DTOs with payload fields omitted. Maintained actionable feed clients request the standard endpoint and depend on canonical context/proposal fields. Restoring payload stripping in the repository would change that storage contract merely to satisfy an outdated expectation.

The correction changes only test expectations and runner/evidence wiring: the inherited lifecycle now positively verifies canonical payload, identity and state retention, while a separate test executes exact source-extracted API/work-triage projection closures and verifies their serialized summary fields and omitted payloads. All existing assertions for create, get, update and normal/mobile list execution remain. No production repository or mobile response behavior changes, and no test is deleted or filtered from the acceptance gate.
