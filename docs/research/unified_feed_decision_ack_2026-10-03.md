# Unified-feed decision acknowledgement, 2026-10-03

## Scope and source

This is a separate follow-on to `browser_ci_response_completion_2026-10-03.md`, based on published source `e90530e30d5d153f6ab1b5400c776bd7433c309c` plus that frozen browser patch. It changes the shared page rendered by `/unified-feed` and `/dashboard/unified-feed`. No browser discovery or HTTP/persisted-state assertions are weakened.

The original `handleAction` removed the card before the response arrived, ignored non-2xx status, and logged transport failures without restoring the decision. A card disappearing was therefore not evidence that a decision had been recorded.

## Implemented behavior

- Keep the card and edited text while the actual mutation is pending. Remove it only after HTTP 200 and a matching item ID, signed tenant and requested lifecycle state. Edited content also needs an unambiguous matching stored payload; contradictory draft fields are rejected.
- Display known HTTP rejection separately from an unconfirmed result. Network failures, 5xx responses, malformed/foreign/inconsistent receipts and missing edited content retain a persistent owner-bound hold. No ambiguous mutation is automatically replayed.
- Reverify identity before dispatch and send expected-user/tenant preconditions to the maintained authenticated proxy. Fence reads, response bodies and writes against owner changes, expiry, storage invalidation, unmount and React StrictMode effect retirement. A late old-owner result cannot mutate the new view.
- Use an immediate in-view guard, an origin Web Lock and a persistent per-owner/per-item marker. A stale competing browser view or same-owner reload must reconcile an existing marker instead of repeating the mutation. Lack of storage/lock support fails before dispatch. This is same-browser protection, not a claim of cross-device or server-side idempotency.
- `Refresh recorded decisions` performs authenticated GET readback only. Because the existing feed is cached and paginated, missing/pending records cannot clear a hold. Conflicting duplicate records are rejected. Successful empty history remains distinct from unresolved or unavailable history.
- The shared `readQueueOwner` helper accepts an optional AbortSignal. Existing callers retain their previous interface. A timed-out header/body verification releases its pending identity state, and a late reply cannot restore the cancelled lease. The feed uses that signal so a stalled identity request cannot retain its view lock indefinitely.

## Backend acknowledgement limitation remains OPEN

`src/server/api/agent_feed.rs::update_feed_item_state` returns HTTP 200 plus an updated row after state persistence. It currently ignores failures from payload updates, legacy approval synchronization and job enqueue. Repeated approval requests can enqueue work again. Browser-side protections do not repair that server contract or prevent duplicate actions across devices, storage clearing or other API clients.

The UI therefore says **Approval recorded. Execution or delivery is not verified by this decision.** It does not call an approval an executed action, a sent reply, a provider receipt or a fulfilled business outcome. Atomic payload/state/queue handling, durable dispatch acknowledgement and server-side idempotency require a separate backend repair. No Rust/backend code is changed here.

## Verification

Pinned Node 22.22.1 is used.

- Initial lifecycle regression run: 14 failures and one pass, reproducing premature disappearance and missing failure/owner/duplicate handling.
- Additional RED→GREEN cases cover unavailable refresh versus empty history, empty auth rejection, StrictMode retirement, stalled identity headers/body and contradictory edited receipts/readback.
- Focused final verification: 55/55 tests across four files, including all existing queue-identity tests. The added coverage includes owner swaps, expiry, late responses, same-owner reload, two competing views, known rejection, network/5xx uncertainty, edited-content retention, actual readback and bounded pre-dispatch cancellation.
- Final unchanged-source complete Next suite: **3,447/3,447 tests in 499 files**, 197.06 seconds, exit 0 (`npm --prefix src/ui/next test -- --maxWorkers=4`).
- Root script suite: **950/950**, zero skips, 54.06 seconds, exit 0 (`npm run test:scripts`).
- Final `npm run typecheck:web`, `npm run typecheck:e2e`, `npm run lint:node` and `git diff --check` passed. Complete browser discovery remains **1,803 tests in 461 files**.
- Independent read-only review approved the revised lifecycle after separately reproducing the StrictMode and identity-timeout issues. It did not claim browser/backend acceptance.
- Final source/test fingerprint, excluding this evidence document: SHA-256 `d47f51e8a7646a9158a9a1233d61bd6abaf6164e63b4769bba78b7f541ce4385` over sorted path-to-SHA256 JSON. Source hashes remained unchanged throughout final validation.
- An earlier incomplete exhaustive run was stopped to add reviewer-requested repairs. It is not certification; the complete unchanged-source run above supersedes it.

Fresh real-stack browser CI remains required. The original 21 configured provider/runtime acceptance cases plus the separately configured recorded-text-analysis case remain OPEN. This change neither configures providers nor certifies their outcomes.
