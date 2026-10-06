# Manual inventory adjustment boundary

The maintained `/inventory` screen and the `/api/v1/ui/inventory` and
`/api/v1/pos/inventory` handlers use recorded tenant-owned products. Empty data is
empty; unavailable or malformed data never supplies a fixture product or quantity.
The signed tenant boundary is retained. Browser display names cannot select a tenant.

GET adds `inventory_version`, an opaque digest of the actual product and its ordered
centralized stock rows. A manual POST is an array of at most 100 entries:
`{id, payload: {item_id, quantity_change, expected_version}}`. Optional legacy
`location_id` is retained in request identity but does not select a different stock
policy. Optional `is_sold_out` changes only that explicit field on PostgreSQL; absence
preserves it. Stable identities and observed versions are mandatory. Missing and
foreign products have the same blocked outcome and never cause catalog insertion.

Each entry retains the historical per-item transaction boundary, with an explicit
outcome. A saved receipt, compatibility marker, quantity changes and inventory-ledger
entries commit together. Receipt lookup is a read-only GET with `adjustment_id`. Exact replay returns the
original receipt even if the product was later deleted, including a DELETE queued
ahead of a retry while the original commit was pending. Product absence is evaluated
only after the final receipt recheck; a changed request
under the same ID or a legacy unverified marker remains unconfirmed. A database/commit error is
unconfirmed. Only the matching `acknowledged` receipt proves the stock mutation.
Receipts do not prove a sale, payment, provider action or downstream fulfillment.

Existing centralized rows form one shared pool: one deduction is distributed across
ordered available rows once, preserving committed quantities. Replenishment is placed
in the first existing row. No implicit location or opening balance is created. With
no centralized rows, the existing product is the stock source. Arithmetic is checked;
manual adjustments cannot remove reserved stock or exceed the supported integer range.
Offline sale/shortage reconciliation is a separate path and is unchanged.

PN-counter oversell debt (`P < N`) remains an explicit limitation: manual adjustments
are blocked with `inventory_debt_requires_reconciliation`. Existing offline paths
retain that debt, clamp visible stock to zero and queue a shortage for review
(`api/durable_sync.rs:764-785`, `api/terminal_offline_sync.rs:230-243`, and
`services/sync/offline_pos.rs:37-41`). No proven replenishment/debt-resolution policy
was found in that contract. This repair does not reset either counter, create stock
from a clamped zero, or clear the pending shortage; reconciliation is required first.

PostgreSQL uses migration 1041 and existing row-level tenant isolation. The historical
MySQL stock-only path now uses a transaction, observed version and persisted receipt;
its receipt table is added to MySQL initialization. MySQL requires its existing product
`updated_at` column plus its durable receipt count for version observation; a
second-resolution timestamp cannot revive a stale version after a manual +/− cycle. This change does not claim that the
legacy PostgreSQL-only POS binding now supports local SQLite operation. SQLite tests
execute the actual portable CAS/receipt SQL and are identified as SQL/storage tests.

The UI checks signed session identity before/after I/O, clears retired account data
on explicit identity mismatch or verification failure even without a browser event,
and retires 401/403 responses before trying to parse their bodies. It
waits for receipts rather than optimistic stock, and retains the exact pending request
in owner-scoped session storage before dispatch. Reload/unknown responses check that receipt without submitting a mutation. An
absent receipt keeps the hold; a separate explicit Retry same adjustment action
resubmits that exact identity. A historical receipt does not replace current stock. Unknown price/currency is shown as unavailable.

Verification commands:
- `node --test scripts/inventory-truth*.test.mjs`
- `npm --prefix src/ui/next test -- src/app/inventory/page.test.tsx`
- `npm --prefix src/ui/next run typecheck`
- `bash scripts/run-inventory-postgres.sh` with an explicitly isolated loopback
  `OHC_INVENTORY_TEST_DATABASE_URL` whose database is named `ohc_*_test`

The required hosted PostgreSQL step provisions an isolated database. Each test creates
and verifies a fresh NOSUPERUSER/NOBYPASSRLS/NOINHERIT role with FORCE RLS, and the runner
rejects missing/ignored cases or a changed source fingerprint. Local Node/source/SQLite
checks do not substitute for Rust compilation, PostgreSQL execution or repository
`make lint` / `make test` acceptance.
