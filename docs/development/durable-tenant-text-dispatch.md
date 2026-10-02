# Durable tenant text dispatch

This path executes one explicitly configured, bounded text-analysis request. It
has no workspace, shell, tool catalog, agent swarm, consumer subscription relay,
or fallback model. It does not claim a business action was performed.

## HTTP contract

Authenticated owner/admin requests to `POST /api/v1/agents/workflows` and
`POST /api/v1/agents/hire` with a task require one `Idempotency-Key` header: a
canonical lowercase, non-nil UUID. Missing, malformed, duplicate or nil keys are
rejected before budget mutation, agent registration or provider dispatch. This
is an intentional API contract change. The maintained Next client creates the
key once for an intended submission, saves only that identity when acceptance
is uncertain, and never automatically sends another task. A new explicit
submission after a definitive rejection gets a new key. Keyless idle agent
registration remains supported because it creates no provider effect.

The deduplication identity is tenant + actor + request UUID. An identical replay
reads the original durable receipt, including after a new database connection or
provider outage. Changing text, model choice, workflow, name or hired-agent role
under the same key is a conflict. Replays do not register another agent, acquire
another execution capability or reserve another budget. The original provider,
model, payer, tariff and ceiling remain part of the immutable receipt payload.

`GET /api/v1/agents/workflows` preserves the `workflows` array of complete
records and adds `next_cursor`. Pages default to 20, accept `limit=1..20` and
`before=<created_at>:<canonical UUID>`, and fit within 1 MiB after JSON escaping.
An included receipt always retains all task/output text. Maximum accepted text
fits below 512 KiB per record; no valid record can cause a nonadvancing page.
Cursors use immutable timestamp/ID ordering, including ties, and need not refer
to an extant row. An exhausted page returns an empty array and null cursor.
The Next history UI offers older/newer pages and preserves full result text.

`GET /api/v1/agents/workflows/by-request/{request_uuid}` performs direct exact
lookup by current tenant and actor, even for requests older than every history
page. Another actor or tenant receives not-found; malformed, nil or noncanonical
UUIDs are rejected. Lost-ack recovery uses this endpoint and never scans history. `GET /api/v1/agents/workflows/{id}` reads one receipt, and
`POST /api/v1/agents/workflows/{id}/cancel` cancels an unclaimed request or records
an unknown outcome for a claimed request. All receipt endpoints revalidate current canonical
owner/admin authority. A caller-supplied tenant, actor or provider is never an
authorization source. A hired receipt includes its stable agent correlation ID.

The browser can recover a lost acknowledgement only when a current authenticated
receipt matches its saved UUID and actor. An unconfirmed marker does not retain
task text. Read failures or another actor's receipt keep the operation held.

## Operator configuration and spending authorization

The actual production constructor requires:

- An explicitly configured supported text provider/model/endpoint and credentials
  through the existing `OMNISOLO_LLM_*` configuration.
- `OMNISOLO_LLM_TENANT_ID`, naming the only tenant allowed to use the process
  credential. The existing explicit `OMNISOLO_BUILDER_TENANT_ID` is accepted only
  as the fallback when the LLM tenant setting is absent.
- `OMNISOLO_USAGE_PAYER`: `managed_api`, `byok_api`, or `local`. Local mode is
  restricted to the configured Ollama transport. Native subscription sessions
  are not relayed by this API.
- For managed API, the configured provider/model entry in
  `OMNISOLO_USAGE_RATE_CARDS` and a positive
  `OMNISOLO_USAGE_MAX_REQUEST_MICROS`. Rates are operator-approved integer USD
  micro-units per million tokens with an explicit revision; this implementation
  supplies no fabricated price or default commercial tariff.
- An already configured persisted usage account/spending limit. The existing
  owner-scoped usage spending-limit API creates that authorization explicitly.
  Inference never creates an allowance, adds payment credit or changes a plan.

Receipt storage and UsageLedger use the exact same configured SeaORM SQLx pool,
for both PostgreSQL and SQLite. PostgreSQL usage/receipt tables enforce forced
RLS; SQLite operations retain explicit tenant predicates and transactions.
Separate transactions form a conservative protocol rather than pretending to be
one atomic cross-module commit:

1. Persist the immutable queued receipt.
2. Reserve the request's bounded maximum in UsageLedger.
3. Commit one durable receipt claim.
4. Commit the usage record as in-flight.
5. Recheck authority and cancellation, then send one provider request.
6. Persist observed provider usage and finish the exact completion proof.

Managed admission prepares the exact normalized request once, fingerprints it,
and reserves every UTF-8 byte of its complete system and user text plus 4096
tokens for fixed protocol framing, together with every allowed
output token at the approved tariff. The request is denied if that bound exceeds
the configured request ceiling or the account's remaining authorization. The
immutable prepared request is carried through admission and sent without another
normalization. Its fingerprint and input bound are stored with the receipt and
reservation. The transport is restricted to one text request and explicit output limit;
history references, hosted tools, media and hidden additional calls are not
admitted. Provider-side pricing or usage outside that declared contract requires
reconciliation and is never replaced with an estimated debit. Responses are
bounded to 2 MiB before JSON parsing, including unknown-length/chunked bodies.
Redirects and automatic retries are disabled. Anthropic cache-write directives
are disabled because this path has no approved cache-write tariff.

BYOK and local records never charge customer-direct inference again. A real
provider response ID and observed token counts support settlement; absent,
invalid, over-reservation or conflicting observations remain uncharged pending
reconciliation. A conflicting provider receipt is preserved as evidence without
a second charge or silent refund. Usage records are separate from display text:
a completed text response does not assert that unknown billing quantities were
zero. The usage event ID is the workflow receipt ID. Ollama has no external
provider response ID; its internal correlation ID is never claimed as provider
evidence. Missing IDs retain reconciliation exposure. Failed/ambiguous accounting
and completion writes emit redacted operation/event diagnostics; stored holds
and leases remain available for conservative read-side reconciliation.

## Cancellation, revocation and restart

Workers are owned by a bounded tracked JoinSet. The admitted request is
non-cloneable, and a claimed receipt cannot obtain another provider attempt.
Authority, token revocation and registration are checked before the effect,
periodically while awaiting it, and at receipt completion. Canonical PostgreSQL
revocation writers and final receipt commit share the token fence documented in
the receipt persistence implementation.

Cancelling a queued receipt proves no claim can dispatch it and permits release
of a still-reserved hold. Cancelling a claimed receipt, losing transport,
revocation during I/O or losing the worker retains an unknown outcome. These
conditions never prove provider cancellation and never refund possibly consumed
exposure. Reads reconcile terminal accounting even when provider configuration
is unavailable. Queued records lost before claim become cancelled after 120
seconds; expired 120-second dispatch leases become unknown. Provider work has a
90-second deadline and no automatic retry. A restart does not resurrect an old
admission capability.

SQLite initialization installs its receipt schema on the configured portable
connection. New optional payload fields omit absent values, preserving existing
v1 receipt fingerprints; existing paid state and immutable rows are not rewritten.

## Verification boundaries

The focused workflow and PostgreSQL harnesses compile the actual handlers,
admission, worker, receipt, provider adapter and ledger implementation. Tests use
owned SQLite files/connections, a non-superuser forced-RLS PostgreSQL role and
loopback HTTP providers with explicit observed fixture responses. No live vendor
account or paid call is used. Run their locked source-fingerprint gates and the
maintained browser tests; focused evidence does not replace `make lint` and
`make test` on the fully integrated revision.

The historical `*.mock-contract.ts` swarm and portfolio-event examples are not
current production acceptance tests. They do not certify an automatic swarm,
portfolio business outcome or live MiniMax execution. The bounded authenticated
text dispatch contracts above are the supported behavior.
