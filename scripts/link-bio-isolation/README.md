# Private link-bio regression gate

Run `scripts/link-bio-isolation/run.sh` with `OHC_BIO_TEST_DATABASE_URL` pointing
only to an owned disposable UTF8 PostgreSQL database. Both the launcher and
Rust fixture validate a literal loopback IP and an `ohc_*_test` database name
before any connection. URL options and fragments are rejected. The launcher
also runs three prerequisite checks, including a no-connection rejection. The registered CI gate
requires all eleven Rust cases; no filtering or skipped cases certify it.

The generator copies six complete production items: both link-bio handlers,
three input/output types, and the existing growth auth compatibility middleware.
The HTTP fixture uses the real strict bearer middleware and Store with actual
signed member credentials. It extracts the canonical tenant/user/KV schemas,
forces KV row-level security under a real non-superuser role, and reuses one
connection across tenant changes. A second pool exercises explicit query
scoping even when the database owner can bypass RLS. Unused GrowthState fields
are omitted; all touched handlers use only its genuine PostgreSQL pool.

The tests cover own-member save/read, forged tenant input, cross-tenant reads,
missing/corrupt state, global alias overwrite, anonymous and unsigned fallback
identity, connection context reset and private response caching. No fixture
executes a provider or publishes a public profile. SQLite/MySQL, full server
composition and actual browser acceptance are separate gates.
