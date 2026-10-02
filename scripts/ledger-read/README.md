# Recorded ledger read contract

Run `bash scripts/ledger-read/run.sh` with `OHC_LEDGER_TEST_DATABASE_URL` pointing
to a disposable PostgreSQL database with schema/role creation permission. This
gate performs no payment or external provider calls.

The harness extracts the complete production `get_entries` handler, its response
types and the actual PostgreSQL pool reset hooks. It mounts the actual
`server_auth::strict_bearer_auth_middleware` and uses its real token validation
and PostgreSQL user/revocation repository. Database selection is the only local
adapter; the MySQL branch is compiled but is not exercised here.

Each test applies the actual ledger migration, seeds distinct tenants and
currencies, and creates a disposable restricted LOGIN role. The read pool is
limited to SELECT and one connection; both session/current user and
NOSUPERUSER/NOBYPASSRLS are verified. Ledger tables use forced RLS. The local
execution also used SCRAM authentication for these roles. Do not replace that
with a superuser connection plus SET ROLE when claiming the same coverage.

The tests cover missing bearer rejection despite forged identity headers,
signed tenant isolation, sequential pool reuse, unchanged stored numeric units,
real empty results, database errors and an undecodable stored row. Successful
reads must not silently discard rows or fabricate a currency. These are recorded
entries, not a verified balance, spendable amount or payment-settlement proof.

Generated source and registry dependencies are fingerprinted against the current
repository inputs and Cargo.lock. The gate does not certify the full main
server, MySQL/SQLite execution, a browser journey, live provider behavior or the
separate legacy balance/safe-to-spend endpoints. Next component tests cover the
mounted response shape and honest unavailable states; the Playwright journey
uses the real authenticated fixture and waits for the actual entries response.
