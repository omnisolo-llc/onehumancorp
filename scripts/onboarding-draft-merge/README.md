# Concurrent first onboarding draft saves

Set `OHC_DRAFT_TEST_DATABASE_URL` to a disposable PostgreSQL database with schema
creation permission, then run `bash scripts/onboarding-draft-merge/run.sh`.
The gate extracts the actual three save methods and their real input sanitizers.
It applies the real onboarding table DDL and preparation-receipt migration in
isolated schemas. A real PostgreSQL advisory-lock barrier makes both initial
INSERTs reach the database before either proceeds; this reproduced loss of one
permitted field before the fix. No timing-only sleep is used to assume the race.

The fix materializes the row within the same transaction before SELECT FOR
UPDATE. A concurrent initial writer then reads the first committed state and
uses the existing shallow merge and maximum-step behavior. Placeholder creation
rolls back with a failed save. Authenticated tenant/user keys remain separate;
user input cannot replace protected system fields or the separate receipt.
There is no new draft version, compare-and-swap, deep merge or receipt contract.
The existing version column is not an authoritative save revision and is unchanged.

The gate also covers separate users/tenants, failure rollback and retry, untouched
protected receipt bytes, and retained shallow nested-object replacement. Only
cache invalidation is a recording sentinel; this gate does not certify Redis,
outer authentication, UI session ownership, or full server/workspace behavior.
Production API response shapes are unchanged: empty 200 for /draft and empty 204
for /state after the committed save. Lost responses/COMMIT ambiguity still require
the client's conservative hold; neither GET timestamps nor this serialization
prove which unknown request completed.

Registry versions/checksums must match the root Cargo.lock. Generated source,
derived lock and source manifest are local artifacts. A before/after fingerprint
rejects changing inputs during the run. Use scoped Clippy with:

```
cargo clippy --locked --manifest-path scripts/onboarding-draft-merge/Cargo.toml --all-targets -- -D warnings
```
