# Share-card landing document startup lifecycle

## Exact evidence

Published baseline: `492245385a64a268e13a2198c3ebf6c6a48f1fbe`, PR #39757,
[CI run 37165371560](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37165371560).
Browser shard 3, job `111328017956`, ended with 150 passed and one failed.
Browser shard 4, job `111328017954`, also ended with 150 passed and one failed.
Both failures occur before click discovery at the unchanged five-second
baseline read-settling assertion for `/share-card`.

The shard 3 diagnostic identifies exactly five unfinished GET fetches, all
started in the `/share-card` document at `00:47:32.028–029Z`: `/api/v1/help`,
`/api/v1/videos`, `/api/v1/auth/session-identity` twice, and `/api/v1/tooltips`.
At failure they are 5,467–5,468 ms old, with no response headers or failure event.
Shard 4 independently reports the same five paths, ordering and source document,
started at `00:53:07.125Z`, aged 5,524 ms. The final page is `/onboarding`; its
initial reads and visible setup readiness have completed. Later destination RSC
requests have terminal events, including completed responses and aborted reads.

[Shard 3 artifact 11288734084](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37165371560/artifacts/11288734084)
retains the failure, screenshots and report. The diagnostics establish stranded
old-document startup requests, not an HTTP 500, authentication rejection or
backend-outage explanation. They do not establish the underlying browser/network
reason that no terminal event was emitted for those old requests.

## Root cause and bounded repair

The redirect-only landing document mounts the full application frame before
`RedirectAfterHydration` replaces that document. `HelpWidget` starts help and
video reads; `NetworkStatusIndicator` and `SyncManagerInitializer` independently
start canonical queue identity reads; the outer `TooltipProvider` starts its
tooltip read. This work belongs to the application destination and races the
landing document's immediate replacement.

The new test renders the actual `RootLayout` provider and widget tree with the
actual share page. Before the repair it reproduces the same five request paths
in exactly the hosted order, failing the no-landing-startup assertion. The test
uses transport fixtures only; no widget, queue manager, identity reader or frame
implementation is substituted. Identity replies explicitly deny authentication
instead of inventing a verified owner or executing queue work.

The repair renders only the share page in `PublicAwareApplicationFrame` for the
exact `/share-card` pathname and defers the outer tooltip read on that pathname.
The actual destination starts all normal widgets and tooltips. Tests explicitly
cover `/onboarding`, `/dashboard` and the distinct `/share-cards` application page,
including queue readiness and notification display. The share page remains
protected for document, RSC and prefetch requests. Authentication policy is not
modified. Metadata, target normalization, fallback link and single hydrated
document replacement are unchanged.

The existing real browser navigation case additionally requires no landing
startup reads and all four normal service paths on the destination. Its existing
server metadata, canonical identity, single navigation and document-stability
assertions remain. No audit fixture, request inclusion rule, five-second deadline,
readiness condition, inventory or owner isolation assertion is modified.

## Verification and limits

- RED: the actual-root-tree regression produced 1 failed / 6 passed. The failure
  lists precisely the five hosted startup requests.
- GREEN: 83 tests passed in 7 files, including the actual root tree, share page
  metadata/normalization, redirect replay guard, application frame, tooltip
  behavior, audit request lifecycle and audit navigation.
- Changed-file ESLint passed with zero warnings.
- Next TypeScript passed.
- E2E TypeScript passed. Whitespace checks passed; the frozen patch is checked
  for clean application to the unchanged published baseline.

The focused tests prove the source lifecycle change. They do not reproduce
Chromium's missing terminal events. Real browser execution of this repair and
the full required `make lint` / `make test` acceptance gates remain pending hosted
CI. No full suite or native build was launched concurrently with the other
repair lanes; focused verification follows the coordinator's resource limit.
