# Dashboard click-audit isolation

The `/` and `/dashboard` click contracts use the same canonical PostgreSQL fixture
as the native runner. Every control is clicked once against a newly persisted
case-owned dataset and a new authenticated browser context. A page reload alone
cannot restore a proposal that an earlier real click approved or dismissed.

## Baseline and ownership

- `src/e2e/e2e-seed.sql` marks its two data-only sections with
  `audit-fixture-data:start/end`. Schema, triggers, policies and RLS changes stay
  in the native runner's initial setup.
- `scripts/ui-audit-fixture.cjs` gives every seeded tenant, user, record key,
  unique email and nested foreign reference a fresh UUID namespace. Native UUID
  keys receive deterministic case-specific UUIDs with a reverse canonical map.
  The canonical source is fingerprinted. Upserts are removed: collisions must
  fail and roll back, never overwrite another test's data.
- The transaction still requires `verifiedFixtureDatabaseUrl`: the runner's
  private proof file, live Docker container ID/name/label and exact loopback
  PostgreSQL binding. The fixture cannot run against a general database URL.
- Persistence checks verify the normalized admin role and the canonical graph:
  five case-owned plan tenants, eight main-tenant feed records, two approvals,
  one inbox item, two products and two opportunities. All remaining canonical
  booking, quote, subscription and milestone data is retained too. The eight
  feed records explicitly include the three operations UI scenarios and daily
  checklist as test data; the audit never waits for invented production tasks.
- Login uses the real HTTP endpoint. Session identity must match the exact new
  user and tenant. Cookies, local storage, delayed callbacks and entitlement
  changes cannot carry over from the previous context.

## Complete target coverage

The first settled case produces the baseline `discoveredKeys`. The audit requires
exactly that inventory before every click and after the final reset. Missing,
additional or duplicate keys fail; disappearing controls are never dropped.
The same attachment records each case's verified owner IDs and complete key
snapshot under `isolation.cases`, plus the seed digest, navigation receipts,
observations and phase timings.

Target keys include record test-ID ancestry for isolated cases. Generated
namespace and UUID portions map back to the canonical record identity. Thus two
identically labeled Dismiss buttons remain distinct even when row order changes.
Every observation still uses a trusted browser click and the existing effect
oracle; no response stubbing or production simulation endpoint is introduced.

`dashboard_audit_isolation.spec.ts` exercises a real dismissal, verifies its
persisted state, proves that the shared document lost a required control, then
checks the full inventory in a fresh case and clicks the alternative control.
Both records' distinct final database states are asserted.

## Timing and evidence

The owned audit has a finite inventory instead of an unbounded rediscovery loop.
Each case, including its reset, has a 30-second step limit. The overall allowance
is `(inventory size + 2) × 30 seconds`, with a 120-second minimum, covering initial
setup and final verification. Individual lookup and gesture limits remain
unchanged. This is a bounded execution allowance, not a measured performance
claim. The original 90-second non-convergence cutoff remains on other routes.
Actual wall time and all target observations must come from the source-bound
hosted run. Local unit tests cannot certify PostgreSQL/browser acceptance.

## Finite API-documentation inventory

The API documentation page's default view is a finite inventory of operation
expansion controls and the normal app-shell controls. The previous shared90s
cutoff failed after31of35controls even though discovery had already converged.
It now freezes the complete discovered key set, checks that same set before each
click and after the final reset, and runs each control exactly once in its own
bounded30s lookup/action/reset step. The route budget is derived from the actual
inventory, not a larger arbitrary convergence cutoff. Existing timing receipts
remain mandatory; `finiteInventory` records the count and per-case budget. This
is a coverage repair, not a performance certification; actual hosted timings
must be read before claiming the new full inventory is accepted.

The dynamic order page uses the real canonical `e2e-seeded-record` order and the
load contract requires its Order Summary and exact record ID. It does not accept
an invented `e2e-id` or whitelist a missing-order error.
