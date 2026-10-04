# Quote acceptance durability gate

Set `OHC_QUOTE_TEST_DATABASE_URL` to a disposable PostgreSQL database whose test
administrator can create schemas and roles, then run:

```
bash scripts/quote-acceptance/run.sh
cargo clippy --locked --manifest-path scripts/quote-acceptance/Cargo.toml --all-targets -- -D warnings
```

The runner uses the repository's locked registry versions/checksums, records every
source input and rejects changes during the run. Generated absolute-path source,
the derived lock and source manifest are ignored. A real non-superuser,
NOBYPASSRLS LOGIN role uses an explicit disposable password; the test verifies
session/current identity before and after tenant-context resets. It works with
SCRAM PostgreSQL authentication and does not weaken database authentication.

The gate extracts the actual mounted accept/get/update/approve handler bodies,
request/model types and tenant predicates, imports the actual acceptance helper,
and applies the real PostgreSQL quote/invoice migrations. The actual Stripe
client creates checkout requests only against an ephemeral loopback HTTP server
using a public fixture key. That server records real idempotency headers and
validates the amount/currency/reference. No provider, credential or payment is
contacted. Original negative controls used the byte-exact old acceptance handler
with a recording provider boundary: repeat acceptance created two invoices, and
invoice failure rolled back SQL after the provider had already been called.

The existing quote API takes a PostgreSQL pool. In addition to handler oneshot
tests, a bounded Axum listener serves the actual extracted handlers over loopback
HTTP at `/api/v1/quotes`, using explicit verified-claims fixtures and real
PostgreSQL. The root-locked reqwest client performs GET, PUT, fresh readback,
stale/missing PUT/PATCH rejection and fresh approval/readback. Requests have
connection/response deadlines, bypass ambient proxies and refuse redirects; the
listener is gracefully stopped and joined before fixture cleanup. This verifies
actual quote HTTP transport, but it does not certify a
separate SQLite quote API, full server startup, JWT verification, browser UI,
webhooks, payment settlement or full workspace acceptance. Handler tests supply
verified-claims fixtures; source guards check the unchanged outer strict bearer
and tenant wiring. The booking-lock boundary in owner-update is a counting, failing
sentinel; rejection tests require zero calls. A rejected owner-update checkout
request would be recorded by the loopback listener and fail closed. These tests
do not certify successful new checkout creation or actual Redis reservation
while approving an unaccepted quote. Existing repository test declarations remain in place.

## Receipt contract

First acceptance, owner PUT and PATCH approval require the observed
`expected_updated_at` from GET in the JSON body. Missing/null or stale tokens
return 409 with `reason: reviewed_quote_version_required`; equality compares
exact instants (microseconds retained, equivalent UTC offsets accepted). The
precondition runs under the owned quote lock before writes, checkout or booking
locks. Existing accepted/invoiced immutability takes precedence. All three paths
share the same comparison, and owner reads/updates/approvals share acceptance's
owned-customer SHARE lock. A quote whose customer has moved to another tenant
now returns 404 on these routes rather than exposing or changing corrupt data.

PATCH approval and PUT transitions to SENT/APPROVED require a currently
DRAFT/SENT/APPROVED quote; DRAFTING and other ineligible states return 409 with
`reason: quote_not_open_for_approval`. Ordinary permitted edits retain their
existing behavior. Successful PUT preserves `success: true` and adds the final
`updated_at`, read after all line-item writes/triggers and before committing.
That token can be used for a subsequent approval; approval still returns `quote`.

The real-PG gate covers missing/stale tokens, exact precision/offset equivalence,
status/terms/line changes, same-version PUT/PUT, PATCH/PATCH and PUT/PATCH races,
zero SQL write attempts (including rolled-back attempts), zero provider/Redis
calls on version rejection, final trigger token/readback equality, save-to-approve,
and foreign/reassigned-customer rejection and lock retention.

GET retains this timestamp in mobile mode. Quote-line writes also
advance that token monotonically, including existing worker writes. The quote,
reviewed terms, exact signed invoice-line cents, one invoice and protected receipt
commit together before a checkout attempt. The existing owner total/deposit and
signed adjustments are preserved; this does not invent a new pricing policy.

Replay returns the same invoice. GET returns the receipt in `acceptance`:
`success`, `status: accepted`, `quote_id`, `invoice_id`, `invoice_status`,
`payment_status`, `stripe_payment_link`, `checkout_status`, and optional `reason`.
Invoice/payment statuses are read from current committed invoice data. Acceptance
alone never sets paid status or claims a booking reservation or message delivery.
The optional checkout URL is stored separately from `stripe_invoice_id`.

A committed claim precedes the single checkout request. The operation identity
is stable for the tenant and quote. A lost request/reply, cancellation, interrupted
claim or ambiguous persistence outcome remains reconciliation; ordinary replay
never automatically issues another request. A pending receipt can survive a
crash before the claim and needs explicit recovery work. Missing configuration
is reported as not_configured. Existing unbound checkout URLs are held rather
than adopted or replaced; pre-existing invoices without a verifiable receipt
require reconciliation. No historical payment state or link is backfilled.

Receipt-backed quote commercial terms and invoice lines are immutable through
both the mounted API and existing direct SQL writers. Recorded invoice payment
status can still advance. The database guard applies to new receipt-backed
acceptance, not every historical quote. Normal parent-deletion cascades are not
converted into an approval mechanism. A privileged-corruption fixture separately
checks that replay refuses mismatching quote/invoice data even if guards have
been administratively disabled; this is not a claim that SQL administrators are
untrusted or constrained by the application.

COMMIT transport errors are conservatively reported as unknown. Deferred database
rejection is tested to leave zero invoice/checkout effects, but the API does not
infer rollback from a generic failed COMMIT reply. There is no automatic checkout
recovery/retry tool in this patch. The pre-existing acceptance path does not enforce valid_until expiry; this
patch freezes that field but does not add a new expiry/repricing policy.
Actual provider reconciliation, checkout expiry,
account binding, payment delivery and settlement remain separate requirements.
