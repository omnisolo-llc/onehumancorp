# Static retirement and factual UI follow-up

The owner requested continued removal of obsolete/mock pages and real feature implementation, using six Astra ultra workers. The first six-route cleanup is draft PR #40124, including its documentation-test repair at `7d4fb9b92447b91d343d446e2563e1ecd6e0144f`. This separate integration branch starts there. On 2026-10-04 at 18:08:45 UTC the owner explicitly approved these eight aliases, the shared affiliate endpoint's HTTP 500 behavior, the specified browser-test repairs and a stacked draft PR. No merge or deployment is authorized.

Six nonoverlapping family assignments cover the remaining 304 shipped HTML paths. Classification distinguishes verified redundant implementations, real legacy functions, material mock/false-success behavior, and unresolved parity; aliases and same-name Next candidates are separate facts. Mixed pages with useful real functions are not blanket deletion candidates.

## Bounded implementation

- Retire two trial-extension interstitials in favor of `/trial-extension`, preserving actual account lookup and the truthful unavailable-grant state. No trial grant is invented.
- Retire four changelog aliases in favor of `/changelog`, preserving the real changelog API and adding distinct loading, validated empty, content, and error/retry states.
- Retire two hardcoded unified-feed pages in favor of `/unified-feed`. Preserve real feed reads and persisted approval decisions. Copy must distinguish recorded approval from provider execution; a decision without dispatch metadata must never imply a send.
- Replace fabricated affiliate growth metrics with validated tenant-scoped API data. Missing referral data is unavailable. Commission cents are total commission, not paid commission. Backend query errors return an error instead of fabricated zero totals.

This batch does not implement trial grants, active-referral accounting, payout reconciliation or provider execution after feed approval. Their unavailable states are interim truthful reporting, not completion of those capabilities. The unchanged dashboard affiliate widget and two legacy affiliate-dashboard pages also still retain zero displays after statistics errors; correcting those callers is separately tracked. The owner's broader request to implement every mock capability remains open beyond this bounded batch.

Compatibility redirects follow the first batch's exact, authenticated GET/HEAD rules. Preserve query strings, browser fragments, anonymous boundaries, unsafe-method handling, RSC/prefetch behavior, standalone historical sources, reviewed public documents, and protected APIs. Root owns the shared route map and shipped-link migration. Workers own separate feature changes and tests.

## Acceptance

Use failing focused regressions before implementation. Prove deleted files and stale shipped links are absent; retained historical contracts remain tested. Browser cases must use the real disposable backend and repository authentication, with desktop/narrow screenshots and actual receipt/readback checks. No fulfilled API responses, production writes, or provider actions.

Run final lint/types, component/script/CLI/desktop contracts, source-bound backend/web build, and real E2E. Attempt required `make lint`/`make test`; missing native prerequisites remain blocked acceptance, never a pass. Publish only a draft with exact evidence and remaining larger capability gaps.
