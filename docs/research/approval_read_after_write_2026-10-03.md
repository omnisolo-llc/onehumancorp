# Approval lists after a committed dismissal

## Observed failure and cause

Source: `71bb6f7f326752312210cf0678d04e5ba76ae1ea`, PR #39757.
The [hosted shard 4 job](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37157353341/job/111304390580)
finished with 149 passed and one failed. The sole failure was
`dashboard_audit_isolation.spec.ts`, “a persisted action on /action-center cannot
erase the next case's coverage”: the control inventory remained 16 after a
successful POST, completed response body, and real navigation/reload. The
unchanged assertion required fewer than 16 controls within its existing timeout.

The [shard artifact](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37157353341/artifacts/11287445032)
records the first list read around 22:25:12 UTC, POST success at 22:25:12.952,
the second list response at 22:25:13.239, and the failed inventory poll ending
at 22:25:17.834. Navigation completed and no response-body wait hung. The other
three persisted feed-route cases passed in that exact job.

Both approval list handlers cached responses for ten seconds and served stale
responses while revalidating. Approval decisions did not invalidate that cache.
The UI's reload therefore received its pre-decision snapshot. Polling the DOM
cannot repair this because the UI does not issue another list request during
that poll. This was reproduced against real PostgreSQL using the production
router and verbatim production list-query methods: a committed REJECTED row
still appeared in the primed pending endpoint, while primed activity omitted it.

## Repair and cost

Pending and activity approval reads now query the existing tenant-scoped
database methods directly and send `Cache-Control: private, no-store`.
Pagination, cursor direction, mobile payload stripping, response shapes, tenant
predicates and decision handling are preserved. Removing this cache avoids
stale in-flight revalidation and independent-server cache invalidation races.
Other application caches are unchanged.

The tradeoff is one existing database read transaction for each pending or
activity request, including requests previously served from the ten-second
cache. No throughput or hosted timing improvement is claimed. A future cache
would need a shared committed revision and correct in-flight read fencing.

The browser regression now additionally checks the POST `{success: true}`
receipt, the owned row's actual REJECTED database state, and absence of that ID
from the real reloaded GET body. The original reload, inventory-decrease,
missing-control and fresh-owner inventory equality assertions and timeouts are
unchanged. There are no runtime response substitutions.

## Executed evidence and limits

- RED: the focused PostgreSQL read probe ran seven cases: five failed for stale
  data or absent private/no-store headers; pagination/mobile and rejected-write
  preservation passed. No production fix was present during that run.
- GREEN: the complete `approvals-read-contract` gate passed 12/12, with zero
  failed, ignored, measured or filtered tests, in 1.14 seconds of test execution.
  Seven cases cover real PostgreSQL read contracts and five are existing
  imported cache/fixture-boundary checks. Strict Clippy for all probe targets
  passed with `-D warnings`.
- All 36 focused/ignored-gate Python tests and both existing ignored-CI
  guards passed. The two new guards were observed failing before wiring.
  Root Node script tests passed 950/950.
- E2E TypeScript and whole-repository ESLint passed using pinned Node 22.22.1.
  Browser discovery remains 1,803 cases in 461 files on this isolated base.

The final gate's source manifest SHA-256 is
`3bc66880a20e9c295e506860eefb2bef1d57adc3680ba535bd3ec8fab2fb2a02`.
The probe compiles the actual router and copies both complete production read
methods verbatim. An independent real database transaction supplies the writes.
Its decision and ledger adapters panic if called; neither returns fake success.
Claims are supplied at the router boundary, so authentication middleware is not
certified by this probe. The gate checks source hashes before and after the run
and verifies focused dependency identities against the workspace lockfile.

Ten root-library regression cases also use the real orchestrator decision
handler (five SQLite and five configured PostgreSQL cases). All ten are ordinary
required tests: native CI provisions their owned database before `make
test-backend`, and the checked-in local wrapper provisions an owned cluster for
the same selector. No new ignored cases are introduced, and the existing
fourteen ignored-test gates and guards are unchanged. These ten tests are added but
**unexecuted locally**: the shared whole-server build had been killed by SIGKILL,
and this repair deliberately used the bounded read probe rather than repeating
that resource-heavy build. The strengthened actual-browser case is also
**unexecuted locally**. Fresh hosted CI must verify the complete mutation →
database → reload behavior and full acceptance; local read-contract success is
not a browser pass. The existing provider/runtime positive gates remain open.

The action-center's optimistic card removal and execution wording are separate
existing UI concerns. This bounded repair does not claim to fix those, provider
dispatch, legacy decision side effects, or the existing read method's swallowed
database-read errors. No commit, push, merge or live provider dispatch was
performed in this lane.
