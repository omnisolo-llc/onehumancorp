# Department and explicit manual inbox delivery receipts (AW-07)

## Scope and evidence

This gate compiles the **exact production** department and manual inbox senders and Meta HTTP client/provider source. It uses selected SQLite pools, disposable loopback PostgreSQL with a non-superuser/non-BYPASSRLS application role, synthetic credentials, and owned local HTTP fixtures. Fixture acceptance is not Meta/Twilio sandbox or live-delivery evidence.

The harness currently contains 61 required Rust tests: the existing 33 department/provider tests plus 28 manual-request, ownership, recovery and owned-HTTP tests (including six additional PostgreSQL cases). No native test is considered passed until Cargo executes it. `prepare.py` fingerprints the full sources, migrations, producer/consumer wiring, lockfile and CI configuration; the runner rejects source changes and successful missing/skipped/filtered PostgreSQL coverage. The minimal fixture `DB` container holds only the production-used selected SQLx handles; no persistence, authorization query, provider method, state transition or receipt parser is replaced.

On an authorized hosted runner with Cargo and an owned database:

```sh
export OHC_DEPARTMENT_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ohc_department_test
bash scripts/department-delivery-contract/fetch.sh
python3 scripts/focused_ci_gate.py department-delivery-contract
```

`make lint` and `make test` remain the complete repository acceptance gates. Node source/SQLite regressions are `node --test scripts/department-delivery-receipts.test.mjs scripts/manual-inbox-receipts.test.mjs`; actual consumer logic is covered by `src/ui/next/src/lib/messageDeliveryStatus.test.ts`. Neither replaces native Rust/SQLx or real-stack execution.

## Implemented contract

- Preparation reads the tenant-owned canonical inbox recipient/channel and configured provider account, and freezes them in the pending reviewed action. Numeric process-local auto-approve limits are not standing messaging authority. Ambassador actions remain pending until owner review.
- Approved body bytes are used unchanged. A translation performed after approval no longer changes the outgoing text.
- Dispatch requires the canonical APPROVED feed row and HIGH review-request marker. A modern feed decision additionally requires its exact admitted job and snapshot in ATTEMPTING state; department triggers cannot bypass a cancelled/reconciliation-required modern decision.
- A transaction commits an unknown state and unique tenant/inbox attempt fence **before** any HTTP request. Failures to commit cause no HTTP. Concurrent/restarted/replayed actions never resend unknown/accepted attempts. Any inbox mirror with prior success/unknown or terminal/paused handling prevents a stale send.
- Provider acceptance requires a bounded, parseable receipt with the expected recipient and provider message ID. It is saved atomically with `provider_accepted`, never `delivered`. Transport loss, redirects, 5xx, malformed/mismatched receipts and post-send storage failure remain unknown. Missing/revoked/mismatched configuration is blocked; definite provider rejection is rejected.
- A fresh reviewed action can recover a proven blocked/rejected no-effect outcome. The old action is terminal; an unknown or accepted attempt must be reconciled, never retried automatically. There is no new automatic delivery webhook claim or reset/replay API.
- Dashboard/inbox label acceptance as delivery unconfirmed, unknown as reconcile-before-retry, and old sent/replied/auto_replied statuses as unverified legacy outcomes. Approval HTTP acknowledgements do not claim sending.

## Real external prerequisites and limits

Tenant `integration_credentials` must contain the correct provider record and active credentials: WhatsApp Cloud API uses `whatsapp_cloud_api`, token in `api_token`, numeric phone-number ID in `from_phone`; Twilio WhatsApp/SMS uses the existing `whatsapp`/`twilio` account SID, auth token, and sender fields; the existing Meta Facebook-login Instagram/Facebook path uses `meta`, its token, and explicit numeric sender/Page account ID in `from_phone`. Legacy token-only Meta entries have no reviewed sender identity and remain blocked until configured and reviewed again. No global token fallback is used.

Meta retains the existing Graph API route/version; this patch does not certify current account permissions, supported provider API version, app review, messaging-window/template eligibility or opt-in/consent. These must be verified in an authorized provider sandbox before rollout. Instagram-login tokens for `graph.instagram.com` are a different integration and are not silently sent to the Facebook-login endpoint. See [Meta's own WhatsApp API response example](https://www.postman.com/meta/whatsapp-business-platform/request/gnzu6lh/send-reply-to-contact-message) for message-ID acceptance. HTTP acceptance alone does not prove delivery; verified provider webhook reconciliation remains follow-on work.

The follow-on adds the mounted manual `/api/v1/ui/omni_inbox/action` route and its sole maintained send consumer, `src/ui/next/src/app/inbox/page.tsx`. Its Next proxy allowlists action/readback fields; the existing strict signed-bearer middleware supplies tenant and actor. User-authored manual replies get their own immutable `manual_inbox_requests` record, never a fabricated agent approval. The receipt and delivery adapters are shared with department dispatch. Quote/invoice and other non-Ambassador department effects remain outside this slice.

## Manual inbox request contract

- New UI submissions use a stable opaque request ID, prepare an immutable tenant/actor/inbox/body/channel/recipient/account binding, then explicitly claim that same request. Replacement retires only pending requests. Pending requests expire after five minutes. No navigation/auth/dismissal-stale UI owner can automatically continue a late preparation.
- The canonical inbox lock serializes preparation/claim/dismissal with departmental claims. New admitted identities atomically increment a durable inbox intent revision. Existing request replays, changed-body conflicts and rejected pre-admission requests do not advance it. Department preparation freezes the revision, so an older approval cannot revive after a newer manual intent even if its provider rejects; a fresh department review can recover a proven no-effect outcome.
- Claim commits the existing shared tenant/inbox unknown attempt fence before HTTP. Cross-tab requests, legacy retries, worker cancellation and restarts cannot create another unknown/accepted attempt. A new request ID is not a reset capability. An old rejected/blocked identity is also never replayed; only a fresh explicit request can recover a proven no-effect result.
- The handler awaits receipt-aware dispatch. Accepted requires a validated provider message ID and means delivery unconfirmed. There is no newly asserted delivered state without a verified delivery event. Rejection, blocked configuration, malformed response, network loss and finalization loss remain distinct. Draft bytes remain in the immutable request for authenticated readback.
- Dismissal/legacy close-only actions are durable and retire pending owners. They cannot recall already claimed HTTP. Late manual or departmental provider receipts preserve the closed inbox status while retaining provider evidence. Terminal state on either mirror is independent of historical/unknown fencing, including the mounted agent-feed alias that closes only the omni mirror. Dismissal/close readback includes the actor's earlier manual send receipt/body, so reload does not hide acceptance or unknown evidence.
- Legacy callers without `request_id` remain supported by a deterministic compatibility key over tenant, actor, inbox identity, exact body and approved flag. It contains no clock, generated row, status or provider field changed by claim. The first request freezes recipient/account. The same legacy payload can never be retried as a fresh effect after rejection; a caller must supply a new explicit request ID to request another attempt. Legacy `approved:true` without `edited_reply` still closes without sending. Empty provided replies are rejected, unsupported channels/configuration return blocked, and existing SMS-prefixed/WhatsApp recipients remain supported.
- There is no reset, resend-after-unknown, webhook reconciliation or automatic provider-delivery certification endpoint in this follow-on. Provider sandbox/live behavior and full browser/server deployment remain external verification prerequisites.

## Local follow-on evidence limits

Source regressions and exact production SQL can run independently. `manual_sqlite_probe.py` tests exact SQLite DDL/statements; it does not execute Rust business logic. The native runner requires an owned loopback PostgreSQL database and Cargo; it fails if missing, filters/skips exist, source fingerprints change, or any required named PostgreSQL/owned-HTTP test fails. Local SQL or mocked UI tests do not substitute for this mandatory native gate, full `make lint`/`make test`, provider sandbox verification, or delivery proof.
