# Canonical tenant branding regressions

Run `bash scripts/growth-embed-regressions/run.sh` with
`OHC_GROWTH_TEST_DATABASE_URL` pointing to an isolated local `ohc_*` PostgreSQL
test database. PostgreSQL is required; unavailable prerequisites fail explicitly.

The small harness compiles byte-exact complete production embed handlers and
query structs from `src/server/api/growth.rs`. Only the unused Hub/tracker fields
of their state container are omitted. SQLx calls and all HTML rendering are real.
Four positive tests require unbranded HTML for the selected Pro tenant; additional
cases preserve exact case-insensitive Pro policy, explicit hide requests,
free/unknown/error defaults, per-tenant reads and unchanged persisted plans.

The SQLite test executes those same SQL strings against the canonical tenants
DDL extracted from the standalone branch in `db.rs`. It verifies the shared SQL
contract, not a PostgreSQL-only handler running on SQLite. PostgreSQL uses the
canonical migration 008 table in a connection-local temporary table.

Dependency versions/checksums are checked against root Cargo.lock. Input and
selected-item hashes are recorded, and the runner rejects source changes during
validation. Generated files and the derived lock are ignored. This focused lane
does not replace `make lint`, `make test`, or fresh browser E2E verification.

The goal-tracker regression also seeds real referral records for multiple users
and tenants. The public widget has no authenticated viewer or referral-record
binding, so those totals must not become personal progress or reward eligibility.
It must show tracking unavailable, preserve configured target/reward, and leave
referral rows and tenant plans unchanged. This is an explicit capability gap;
these tests do not claim that individual referral tracking or rewards work.

The context regression requires Python 3 and Node. It parses actual rendered HTML
for customer-referral, viral-goal and viral-widget embeds, then executes only the
static listener in an isolated VM with a recording window.open and no network.
Eight tenant inputs per handler cover quotes, ampersands, Unicode, encoded-looking
text and script-looking text. The URL and attribution links must round-trip the
raw tenant; no tenant data may enter JavaScript. This is parser/listener coverage,
not a real-browser exploit or external referral request. PostgreSQL also verifies
raw tenant identity remains separate from an HTML-encoded lookalike's Pro plan.
