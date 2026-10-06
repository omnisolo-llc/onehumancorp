# Department message delivery receipts (AW-07)

## Scope and evidence

This gate compiles the **exact production** department sender and Meta HTTP client/provider source. It uses selected SQLite pools, disposable loopback PostgreSQL with a non-superuser/non-BYPASSRLS application role, synthetic credentials, and owned local HTTP fixtures. Fixture acceptance is not Meta/Twilio sandbox or live-delivery evidence.

The harness currently contains 33 required Rust tests: 16 SQLite dispatch tests, 5 PostgreSQL tests, 8 Meta client tests, and 4 Meta provider tests. No native test is considered passed until Cargo executes it. `prepare.py` fingerprints the full sources, migrations, producer/consumer wiring, lockfile and CI configuration; the runner rejects source changes and successful missing/skipped/filtered PostgreSQL coverage. The minimal fixture `DB` container holds only the production-used selected SQLx handles; no persistence, authorization query, provider method, state transition or receipt parser is replaced.

On an authorized hosted runner with Cargo and an owned database:

```sh
export OHC_DEPARTMENT_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ohc_department_test
bash scripts/department-delivery-contract/fetch.sh
python3 scripts/focused_ci_gate.py department-delivery-contract
```

`make lint` and `make test` remain the complete repository acceptance gates. Node source regressions are `node --test scripts/department-delivery-receipts.test.mjs`; actual consumer logic is covered by `src/ui/next/src/lib/messageDeliveryStatus.test.ts`. Neither replaces native Rust/SQLx or real-stack execution.

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

This slice repairs department approval and the modern admitted action-worker route. The separate manual `/ui/omni_inbox/action` handler in `src/server/lib.rs` retains legacy detached/optimistic behavior and needs its own explicit-user-action/idempotency repair. Its caller is `src/ui/next/src/app/inbox/page.tsx`; departmental dispatch respects its closed/dismissed inbox state, but this patch does not certify that manual send path. Quote/invoice and non-Ambassador department effects are outside this slice.
