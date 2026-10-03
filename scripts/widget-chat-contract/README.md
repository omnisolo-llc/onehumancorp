# Widget chat boundary probe

Run `run.sh` only with an explicit loopback `OHC_WIDGET_TEST_DATABASE_URL` selecting
an ASCII `ohc_*_test` database. The shell and actual Rust fixture reject unsafe
prerequisites before connecting. Missing storage never skips tests.

The harness imports the complete actual widget router, handlers, repository and
canonical models, including the repository's three original struct tests (aligned
with canonical stored fields). It extracts the exact parent widget mount and real
pool-reset helper from production. DB is only an adapter holding the pool/store
fields those modules access. Actual signed Store/middleware handles requests;
there is no injected Claims/auth bypass. The whole application and unrelated
workers are not booted. Registry versions must match the root lock.

The first seven new cases cover startup/admission with an unavailable loopback
pool. The PostgreSQL cases prove actual owned conversation/message persistence,
readback, both parent relations, malformed legacy relations, normal16-hex and
opaque stored actor IDs, disabled/revoked/missing identities, repeated pool use,
deferred commit failure and explicit unsupported SQLite behavior. A real
restricted LOGIN/NOBYPASSRLS role uses SCRAM (a wrong password must fail); owner
pool variants ensure predicates also work when RLS can be bypassed. Successful
responses must be JSON; Axum text rejection bodies do not bypass status checks.

The API is authenticated chat for an existing account. Conversation POST requires
explicit inbox_id/contact_id already owned by the signed tenant. Message POST
accepts no sender, direction or actor fields; it derives agent kind and exact
opaque actor ID from current verified membership. Anonymous customer widget
capabilities, implicit parent creation and external delivery remain out of scope.

Migration1021 converts only polymorphic sender_id storage to text. Contact/bot
identities are not assumed to be user IDs. Actual SQLx migration tests preserve
UUID strings, null bot identities, message bytes, original233/1009 checksums, both
installation orders and dependent indexes. An incompatible synthetic FK must stop
migration atomically instead of being dropped. Canonical1009 fixture bytes are
from reviewed896a24d; when its active file exists in the paired checkout, the test
requires exact equality. Production233 and1009 bytes are never edited.

The trusted canonical ChatService retains its UUID input contract and converts
that bind to text. Its actual existing8-case fixture must separately apply1021;
this harness does not pretend to execute that service. The widget grants no new
contact/bot impersonation path. Unrelated legacy work-item/AI-draft methods are
unchanged, so this is not whole-omnichannel certification.

No provider, model, external database or business-process dispatch executes.
Isolated test child processes are used for process-global configuration. Stopped,
zero-discovery or ignored test runs are never a pass. Whole-main and hosted
acceptance remain separate from this focused gate.

Review follow-on: every widget response, including authentication/extractor errors,
is private,no-store. Transport bodies are limited to64KiB; message content must be
nonblank, NUL-free and at most16KiB of UTF-8. Opaque actor IDs are limited to4KiB.
History returns {messages,next_cursor}, defaults to50/max100 records, and caps
serialized pages at256KiB. Continuations bind tenant/conversation and stable
(created_at,id) order. Each page rechecks current parents; it is not a frozen
cross-page snapshot. The SQL projection bounds legacy variable-width fields and
refuses oversized stored values rather than returning truncated/empty messages.

Explicit READ COMMITTED transactions lock the conversation, then inbox and contact
with SHARE locks in separate ownership-checked statements. Conversation creation
locks inbox/contact before inserting. Locks survive until commit, fencing tenant
reassignment during the operation. Real database-observed races cover both lock
orders for both parents and all three mounted operations. A later reassignment
makes the old relation unavailable; this API does not transfer conversation data.
Other writers/cascades can still acquire locks in another order and deadlock;
those errors roll back and are never retried automatically.

Lock waits are capped at3s and statements at5s within each widget transaction;
stricter configured values remain effective. A real held-parent timeout regression
proves rollback/no receipt, later pool reuse, and retention of a100ms role default.


## Committed event publication

The gate also requires explicit `OHC_WIDGET_TEST_REDIS_URL` pointing to a
disposable loopback Redis server with an explicit port, no credentials/options,
and database0. The test runner clears production Redis configuration; only
isolated child processes receive the validated test URL. Connection-stall tests
inject failures only. Successful publications use actual Redis subscriptions,
not simulated acknowledgements. No Redis keys or global flush commands are used.

Invalid Redis configuration leaves the optional pool unavailable without a panic
or URL/credential logging; a fresh-process test exercises the actual helper.
The full production Redis pool, current configuration helper, outbox relay and
exact server startup block are included. Migration060 supplies the actual JSONB
queue and RLS policy, alongside the canonical chat migrations. A message and its
pending event are one transaction; failed queue writes or deferred commit errors
roll both back. The stable event identity is the canonical message ID. Redis
publishing runs only after commit and re-reads tenant-owned canonical rows.

The internal worker claims only `publish_chat_event` rows with SKIP LOCKED. Its
transaction holds the queue and canonical authority rows while a750ms bounded
connection/publication runs;
all database and network work has a5s outer bound. Stricter configured statement
and lock timeouts remain effective; authority rows stay locked through the local
publication attempt. A timeout is an unknown Redis outcome, not cancellation of
an already submitted command: late publication after lock release is possible. Process interruption cancels
the claim instead of leaving a permanent PROCESSING job. Failures and zero
subscribers retry with exponential delays, capped at256s and10attempts, then
retain FAILED intent. A successful Redis command with active subscribers stores
an explicit publication acknowledgement. This proves publication only, never
recipient receipt, reading, processing or customer delivery. Consumers must
deduplicate `event_id` and use authenticated history for catch-up: a lost database
acknowledgement can produce the same event again. No in-repository browser
subscriber or customer-delivery transport is introduced by this relay.

Real PostgreSQL/Redis regressions cover no phantom event on commit rejection,
atomic JSONB intent, explicit Redis publication metadata, zero-subscriber retry,
restart bootstrap, concurrent claims, foreign/malformed identities, reassigned
parents, connection outage, capped retries, cancellation and lost-database-ack
replay. The startup fixture retains the extracted shutdown sender until its
worker test completes; the ordinary HTTP fixture does not boot unrelated
application workers. Complete backend/server and hosted gates remain separate.
