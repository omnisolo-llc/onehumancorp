# Exact registration transaction proof

This fallback harness builds the real `src/server/auth/mod.rs` library and its
real tests as an isolated Cargo workspace. No transaction, store, authentication
handler or database is replaced. The existing registration tests use actual
SQLite databases and the production SeaORM registration transaction.

`prepare.py` derives dependencies from the production auth manifest and root
workspace versions. The runner checks registry versions/checksums against the
root lockfile and rejects source changes during validation. Generated manifests,
locks and fingerprints are ignored. The harness avoids unrelated whole-workspace
resolution while preserving the actual auth dependency closure.

Run `bash scripts/registration-proof-contract/run.sh --lib` for all auth library
tests. An explicit test filter is permitted for a recorded RED/GREEN iteration;
report the filtered count, and do not call that a complete auth or repository
pass. This gate does not replace full `make lint`, `make test`, PostgreSQL grant
contracts or mounted transport/browser verification.

The `portable_migration` integration target imports the actual portable migration,
connection, and entity modules. Its real SQLite databases verify old-schema upgrades
without guessed historical owners, new source binding, and transactional rollback.
Run it with `run.sh --test portable_migration`; no replacement store is provided.
It also applies the unchanged canonical SQLite tenant bootstrap before and after
portable migration, verifies all actual columns/defaults and preserves an existing
paid tenant's settings. SQL definitions are compared to the complete canonical
SQLite/MySQL/PostgreSQL bootstrap source; this does not claim a live MySQL run.

Generated manifests live in `.build`; the adjacent ignored `migrations` symlink
points only to the unchanged real server migrations, preserving the actual auth
test helper’s manifest-relative `sqlx::migrate!` input. No migration is copied or
replaced, and the entire SQL input tree is fingerprinted.

The separate `postgres_registration` target imports the same production portable
modules and registration store, plus the actual `tenants`/`users` DDL from the
canonical initial migration. It requires `OHC_REGISTRATION_TEST_DATABASE_URL` for
a dedicated loopback `ohc_*_test` database, rejects URL overrides, uses a fresh
private schema and verifies the existing system role before granting privileges
only on that fixture schema. It retains the actual user/tenant foreign key and
forced tenant RLS, and checks fresh ticket/OIDC namespaces, rollback, collisions,
consumed-ticket retries and preservation of an invited account’s existing paid
plan. Run `run.sh --test postgres_registration` only while you own the coordinated PostgreSQL/native slot. It makes no provider calls.

The runner uses a public fixture signing key and clears inherited global database,
Redis and secret-file settings. Repository connections come only from the actual
SQLite fixtures or the explicitly guarded PostgreSQL test URL. No personal auth
key is read or created by the contract.
