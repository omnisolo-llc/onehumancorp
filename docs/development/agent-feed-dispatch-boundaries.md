# Agent-feed decision and dispatch boundaries

## Confirmed baseline defects and scope

Source inspected from combined tree `a879c1255ded461535c5968ebaa2246d19504e15`. This repair is a separate checkpoint; this document does not certify the current frozen aggregate.

The mounted `PUT /api/v1/agent-feed/{id}` and `/state` call `update_feed_item_state` in `src/server/api/agent_feed.rs`. The original route separately saves payloads, state and legacy status, ignores payload/synchronization/enqueue failures, and re-enqueues every `APPROVED` request. Editing without a `proposed_action` replaces its metadata. The background `AgentActionWorker` takes a temporary Redis lock, dispatches without re-reading approval or authority, and retries errors/timeouts even when work may already have occurred.

The repair scope is atomic decision admission and persistent single-attempt dispatch fencing. A recorded decision is distinct from a durable queued dispatch; a handler return is distinct from external execution/delivery. A provider outcome cannot be deduced from HTTP 200, an inserted queue row, a log message, or an internal handler returning `Ok(())`.

The existing standalone repository read/write methods remain available. This HTTP queue workflow already uses PostgreSQL; canonical PostgreSQL owner identity and business objects must match before the repaired route can admit work. A configuration with no canonical identity authority fails closed. This does not claim SQLite dispatch support.

## Reachable downstream gaps, not repaired here

All paths below are reachable from `AgentActionWorker::process_job` through `domain::action_router::dispatch_action`, after an owner approves a feed payload whose `feature_type` matches. No live provider call was used to verify these source findings.

- `supply_order`: `action_router.rs` logs approval/dispatch and, when `draft_message` exists, logs a simulated outbound vendor send. It returns `Ok(())` without an acknowledgement.
- `social_post_draft`: the same router logs scheduling and contains a comment that real Ayrshare buffering is absent.
- `lead_recovery`: logs a recovered lead and sent reply when `draft_reply` exists; no provider call/receipt.
- `dispute_resolution`: logs response delivery and a simulated refund based on `refund_amount`; `operational_action` is only logged. These logs must not be interpreted as a refund or external resolution.
- Unknown `feature_type`: the router warns, then returns `Ok(())`. A worker return is not evidence that a supported action existed.
- `invoice_followup` → `domain/invoice.rs::handle_invoice_action`: after checking invoice state/deduplication, inserts a draft communication, marks it `sent`, logs a simulated email, then marks it `delivered` without provider evidence. For example, an eligible unpaid invoice plus `generated_response` reaches these false communication states.
- `ambassador_reply` / `instagram_dm` → `domain/inbox.rs::handle_inbox_action`: with `inbox_message_id`, marks inbox records `replied` / `sent` before delivery. WhatsApp, Meta and SMS paths spawn detached provider calls and return before acknowledgement; errors are only logged. Missing credentials or an absent inbox ID can also return `Ok(())` with no external effect.
- `booking_draft` / `autonomous_quote` → `domain/booking.rs`: can mark an omni-inbox reply `sent` through ignored database results, without a delivery acknowledgement. Quote/booking handlers need their own durable output and authority audits.
- `quote_draft` → `domain/quotes.rs::handle_quote_action`: marks a quote `SENT` before the rest of its invoice/payment-link work. This path can call Stripe and must be tested only with an explicitly controlled provider boundary in subsequent repairs.
- `create_product` → `domain/catalog.rs::handle_create_product`: creates a new random product ID on every invocation. This is the local database action used to prove that worker redelivery and uncertain acknowledgement must not call dispatch twice.
- Incident dispatch → `domain/incidents.rs::handle_incident_resolution`: returns `Ok(())` even when `incident_id` is absent or an UPDATE changes zero rows. A dispatch return therefore still does not certify incident resolution.

There is also a separate legacy UI approval path embedded in `src/server/lib.rs` that directly updates feed/approval state and invokes business actions. It is not the agent-feed PUT/queue route and is not claimed repaired by this checkpoint. Inventory its callers before claiming product-wide approval integrity.

## Verification status

The focused contract imports production decision/store/queue/worker source and executes disposable loopback PostgreSQL requests. The actual production router and local `create_product` database handler are imported unchanged; other provider handlers are explicit panic boundaries, never fake successful implementations. Cache/WebSocket notifications are inert in the focused harness and are not certified by it. Real production migrations and canonical auth implementations are imported, not replacement persistence or authority implementations.

Exact commands, counts, source fingerprints and outstanding full acceptance gates will accompany the completed patch. Test preparation alone is not a passing test result.

## Implemented decision contract

- Both existing PUT aliases use the exact signed bearer and canonical current-owner transaction. Optional expected-user/tenant headers must match that bearer; absent or mismatched canonical storage fails closed.
- The canonical feed item, edited existing content, legacy approval/request synchronization, durable decision record and a single PostgreSQL queue admission commit together. Failed or ambiguous commits are not acknowledged as successful.
- Replays retain the original job identity. Approved content cannot be replaced by a replay. An explicit fresh-authority approval can reactivate a cancelled, never-attempted job under the same identity. Attempted or uncertain work cannot be reopened this way.
- `GET /api/v1/agent-feed/{id}/decision` reads the owner-scoped durable state without cache. `PENDING`, `ATTEMPTING`, `DISPATCH_RETURNED`, `RECONCILIATION_REQUIRED`, `CANCELLED` and `NOT_REQUESTED` describe the dispatch record, not customer delivery. Historical approvals without provenance and missing/terminal queue admissions are held for reconciliation.
- Worker claims recheck current owner roles, explicit bearer revocation and the canonical approval. A durable attempt timestamp fences redelivery and survives process loss. Safe failures before that timestamp can retry; failures/timeouts or lost acknowledgements afterward never automatically resubmit.
- Schema protections prevent retargeting an admitted job, clearing an attempt/return timestamp, altering an attempted snapshot, or deleting attempted/uncertain evidence. Restart recovery holds abandoned attempts; a successful handler return is labelled only `DISPATCH_RETURNED`.
- Legacy inbox/invoice approvals no longer manufacture a `sent` state in the decision transaction. Existing order processing/cancellation and legacy approval/request decisions remain represented. Canonicalized legacy records are suppressed from duplicate feed union arms.

## Exact local evidence and remaining limits

The initial HTTP/DB/worker run had 19 expected failures. Subsequent review reproduced three edge failures (fresh-authority reapproval, actual queue redelivery, duplicate return acknowledgement) and two stale queue-readback failures. Separate PostgreSQL tests reproduced mutable/deletable attempt evidence before the schema fence was added. Final focused inventory is 41 reported passes, including 32 explicit HTTP/PostgreSQL/queue contracts; strict focused Clippy and changed-source formatting pass. One inherited optional repository PostgreSQL test returns early under the harness's dummy SQLite configuration. Its reported `ok` is not additional PostgreSQL evidence. The five inherited standalone repository tests execute their actual SQLite assertions.

The local side-effect tests run the real `create_product` implementation on an owned privileged fixture solely to count actual inserts and prove the dispatch fence. A separate non-BYPASSRLS test installs the production product RLS policy and demonstrates the existing bare-pool handler failure: no product appears and the attempted dispatch is honestly held for reconciliation. Thus this checkpoint does not certify production-RLS catalog fulfillment. That narrow handler repair is a separate follow-on; it must preserve canonical authority and tenant context, not bypass RLS.

The terminal-queue case materializes the actual `FAILED` queue boundary directly; it does not certify all generic dead-letter cleanup prerequisites. The broad `make lint` and `make test` commands were attempted in the isolated checkout and both stopped at the web-build prerequisite (`next: not found`, exit 127 from Next, exit 2 from Make). Full workspace/native desktop/browser acceptance remains outstanding and must not be inferred from the focused contracts. No live provider, customer or business dispatch was performed.
