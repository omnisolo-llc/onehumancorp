# Field operation authority and persistence

The mounted `/api/v1/field-ops` and `/api/v1/field-service-routing` APIs require
actual signed bearer authentication. Mutation/planning and route reads additionally
verify current owner/admin authority against the canonical identity store. Field
writes use the same proven PostgreSQL pool as identity and recheck authority at
commit with the existing token fence. Startup compares actual database object and
lock namespaces through `canonical_pg_data_pool`; matching URLs or copied rows are
insufficient. Unproven private appointment storage is unavailable, not an alternate
source. Existing signed-tenant appointment read permissions are preserved.

## Caller contract

- Appointment reads include nullable `updated_at`. A legacy null timestamp cannot
  authorize a blind mutation; reload or repair the underlying record separately.
- Appointment POST requires `id`, a supported `status` and `expected_updated_at`.
  Optional `notes`, coordinate pair and start/end times are persisted, and the
  response returns the actual stored values plus the new timestamp. Omitted/null
  optionals retain existing values. End time cannot precede start time.
- Both appointment and routing-job status writes compare the exact observed
  timestamp and reject reopening terminal states. Missing preconditions return428;
  stale versions or changed-key replays409; absent/foreign records404. Committing
  an update advances timestamps monotonically, including within one clock tick.
- `Idempotency-Key` is recommended for status edits and mandatory when saving a
  route. Migration1030 stores the exact committed response with tenant/operation/
  key and a digest binding actor and request. Replays return that receipt; changed
  requests conflict. Receipts and mutations commit or roll back together.
- Optimize-route uses the IDs and timestamps supplied by the caller, then loads
  actual owned appointment data. `commit:false` returns a preview. `commit:true`
  saves a `prepared` route and its jobs atomically, returning `committed:true` and
  `routeId`. Appointment reload uses the most recently prepared/active route for
  the current UTC date, rather than mixing stop positions from historical routes.
  Legacy supplied `tenantId` implies persistence unless explicitly
  overridden by `commit:false`; any supplied tenant must equal the signed tenant.
- Running-late is a30-minute proposal from stored owned records. It retains the
  observed timestamps and returns `committed:false`. Persist each approved change
  with the original timestamp, and keep partial failure visible. Neither endpoint
  sends notifications or claims staff accepted an assignment.

Supported appointment statuses remain Requested, Pending, Scheduled, Confirmed,
En-Route, In-Progress, Completed and Cancelled. Routing-job statuses remain pending,
en_route, on_site, done, completed and cancelled. An owner-reported completion is
not independent verification of physical work. The existing department task is
created only after a real transition to Completed and committed in the same
transaction; exact replay does not create it again.

Route reads join both appointment and route ownership inside one transaction and
return failure rather than success-shaped empty data on persistence errors.
Missing scheduled times remain null. No default tenant, staff identity, geographic
origin or synthetic scheduled time is substituted. Existing invalid relationships
fail closed without destructive cleanup or foreign-data disclosure.

## Verification

Use `scripts/field-boundary-contract/run.sh` against an owned loopback database.
It checks a frozen source manifest and runs complete actual-handler regression
inventory. The existing `operations-appointments` read gate remains separately
required. Focused checks do not replace `make lint`, `make test`, full application
compilation, browser journeys, or provider-sandbox verification.

## Offline appointment status events

`/api/v1/sync/events` uses the server's canonical authentication Store. Appointment
items additionally require current owner authority over the same proven canonical
appointment, task, conflict and receipt relations. Each item obtains a fresh owner
transaction; exact receipt replay also rechecks authority before acknowledgement.
Receipt, observed-state CAS, schedule/notes mutation and department task are atomic.
Terminal records cannot reopen. Case-only normalization of a completed record does
not enqueue completion work again. The existing per-item receipt shape is unchanged.

Payload schedule fields are `scheduled_start_time` and `scheduled_end_time`, with
RFC3339 values. Missing/null schedule values preserve the stored endpoint; supplied
values merge with the stored counterpart and require end >= start. Missing notes
preserve notes, explicit null clears them, and any notes write needs expected_notes.
All writes need the captured expected_updated_at and expected_status. Reconciliation
and blocked outcomes must stay in the client queue for review; only an explicit
matching acknowledgement proves commit. A receipt proves local persistence, not
customer delivery or physical-work completion.

No migration or provider dispatch is introduced. The mounted event route and field
handlers remain PostgreSQL-only. A signed SQLite identity receives a blocked item
when PostgreSQL authority cannot be established; supported local SQLite authority
transactions remain independently usable and are covered by a compatibility test.


### Offline quote and operation-intent children

`/api/v1/sync/offline` and `/api/v1/sync/operation-intents` now admit signed tokens
through that same canonical Store and require a fresh current owner/admin
transaction per item. Each route proves its actual receipt and business relations
against the configured data pool; equal URLs or copied token/user tables are not
proof. Receipt replay checks current authority, and expired/revoked writes roll
back before acknowledgement. Existing inventory, quote task and intent SQL effects
remain unchanged and share the receipt commit. No migration or live dispatch is
part of this repair.

The field draft-quote child must provide `payload: {notes: <captured notes>}` to the
operation-intents route as well as the existing raw notes string to the offline
mutation route. Its stable child ID has one independent receipt per route. Both
receipts establish local persistence only. The separate invoice-generation route
still requires approved line items; this work does not issue an invoice, invent
prices, or automatically retry a previously held client envelope.
