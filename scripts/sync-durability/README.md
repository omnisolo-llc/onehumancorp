# Focused sync durability verification

This harness is for a separately provisioned, disposable PostgreSQL database. It
never starts the POS worker, calls a payment provider, or uses production data.

```sh
OHC_SYNC_TEST_DATABASE_URL=postgres://.../ohc_sync_test \
CARGO_TARGET_DIR=/path/to/owned/build-output \
scripts/sync-durability/run.sh
```

The supplied database user must be able to create isolated schemas and a
non-superuser role for the forced-RLS regression. The database is disposable;
tests retain their isolated schemas to support failure inspection. Run PostgreSQL
and this command in the same executor session where process lifetimes require it.

The script seeds the focused lockfile from the repository's Cargo.lock and rejects
any registry version/checksum drift. It compiles the exact mounted handler bodies,
request/response types, durable helper files and the actual server_auth crate,
including signed JWT validation and PostgreSQL revocation reads. Cache and mesh
transport endpoints are inert test plumbing. They are not evidence that Redis,
CDN, mesh delivery, downstream workers or providers operated successfully.

Production helper tests and all six maintained offline route regression names run
against real PostgreSQL. Additional HTTP tests mount the same handlers at their
production paths. This does not compile or exercise the complete server router,
all middleware, the monolithic server crate, or the full application/E2E suite.
The source fingerprint is checked again after execution. These focused tests do
not replace `make lint` or `make test` acceptance.

## Wire contract

The events, offline mutations, operation intents and terminal offline endpoint
return additive `outcomes: [{id, route, status, reason?}]`. Status is
`acknowledged`, `blocked`, or `reconciliation`. Existing count field names remain;
terminal `synced_count` counts acknowledged requests only. Durable shortage
records remain explicit reconciliation outcomes rather than inflating success.
An acknowledgment means the corresponding receipt and requested durable mutation
or queue insertion committed. It is not evidence of a completed payment or
completed downstream AI work. Lost COMMIT replies require reconciliation.

Order/product/appointment mutations require an observed `expected_updated_at`
and the expected fields they change (`expected_status`,
`expected_is_sold_out`, and `expected_notes` when changing notes). Missing or stale
tokens never receive a positive acknowledgment. Migration 234 advances the row's
updated_at on all updates, including writers that previously omitted the field.
`base_version` can be zero when unknown; it is never silently rebased. Event
acknowledgments also expose the committed `result_version`, including exact replay.
Legacy receipts with no stored request identity require reconciliation.

Terminal replay compares the immutable semantic request, including tenant,
terminal, amount, currency and parsed payload. The current non-cryptographic
`sig_` transport check is performed on every request; only that signature field
is excluded from semantic identity, and the original stored signature is never
rewritten. No cryptographic device attestation is claimed. Non-USD offline card
transactions are held because the existing downstream worker only supports USD.


## POS read-to-sync preconditions

The gate also extracts the complete POS order reader and inventory GET handler,
their signed-tenant helper, and the original SQLite order-query regression. Its
HTTP fixture uses the actual strict bearer middleware and PostgreSQL user store.
It reads microsecond row timestamps and stored sold-out state through the real
GET responses, then uses those exact fields to obtain committed product/order
sync acknowledgements while a forged tenant header cannot select another tenant.
A missing timestamp remains null; no version or row token is invented.

The POS orders path uses the actual in-process HybridCache with no Redis client.
The database-selection adapter supplies the disposable PostgreSQL pool; the
unchanged MySQL branch is compiled but is not exercised. This does not certify
MySQL or SQLite HTTP execution, the full outer router, or the Kitchen browser
journey. All pre-existing sync tests remain selected by run.sh and the existing
mandatory PostgreSQL CI step.
