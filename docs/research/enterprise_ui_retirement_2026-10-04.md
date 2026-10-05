# Enterprise UI audit and bounded static retirement

The audit inventoried 216 Next page routes and 310 shipped HTML paths at `40754bf82a9667025b0d440dad9b6bd4a533825c`. Six reviewers inspected desktop and narrow browser outcomes with a real isolated Rust/PostgreSQL/Valkey tenant. Target-page evidence covers 523 paths; `/pos/terminal` showed a PIN gate, `/ui/bio.html` an access gate, and `/share-card` a redirect. Initial/loading/error states are explicitly qualified. Eight legacy paths depended on unavailable external Tailwind CSS. This is not all-state, provider or enterprise-readiness certification.

## This change

Retire six redundant Next public HTML implementations, preserving authenticated compatibility URLs:

- `/integrations.html` and `/ui/integrations.html` → `/integrations`.
- `/api-docs.html`, `/ui/api-docs.html`, `/api/ui/api-docs.html`, `/api/v1/ui/api-docs.html` → `/api-docs`.

The legacy integrations bundle requested missing compiled assets. The legacy API reference requested missing relative Swagger assets. Their maintained Next replacements fetch the real `/api/v1/integrations` and `/api/v1/api-docs-spec` APIs. Provider connection controls distinguish unavailable, configured and verified states; this change does not perform or certify external provider verification.

Keep authentication before compatibility redirects, including anonymous API-shaped aliases returning401. Update shipped Help/dashboard references. Preserve canonical tooltip keyboard behavior instead of discarding the legacy accessibility assertions. Shared helpers and historical Tauri sources remain; Tauri's release entry is `src/ui/tauri/bootstrap/index.html` and its maintained application is the fresh Next standalone package.

The first classification found 132 HTML paths with same-name Next routes and178 without one. A name match, duplicate bytes, or lack of literal references does not prove functional obsolescence. Only the six reviewed paths above are removed. Production traffic/referrer evidence was unavailable.

## Retained access contracts

No legacy `.html` page is anonymously allowlisted by Next. Keep `/login`, `/register`, `/verify-email`, `/healthz`, reviewed authentication/bootstrap methods and framework assets. Preserve exact anonymous publication GETs under `/api/v1/public/sites/{publication_uuid}`, reviewed `/pages/{path}`, and selected `/products/{product_uuid}`, with eligibility, withdrawal, no-store and security-header enforcement. Do not expose every widget or `/ui` path publicly. Provider callbacks retain their independent verification boundaries.

## Remaining implementation work, in priority order

These are bounded follow-up scopes, not claims of completion. Source-only observations require reproduction before being reported as observed effects.

| Priority | Gap and evidence | Real implementation required | Acceptance |
| --- | --- | --- | --- |
| P1 | Affiliate screen rendered0affiliates but48referrals/$1,240 from missing-field fallbacks despite a real zero-valued API response. | Align UI/backend contracts; expose only tenant-scoped computed metrics and mark absent data unavailable. | Empty tenant shows real zeros/unknown values; failures never inject fixture metrics; persisted records reconcile displayed totals. |
| P1 | Source paths in finance, assistant, bookings and static POS claim paid/running/sent/success without corresponding confirmed outcomes. | Bind UI state to durable command/payment/task receipts. Keep accepted, executing, failed, unknown and settled distinct. | Real rejected requests, timeouts and disconnects do not produce success; duplicate/restart cases reconcile persisted outcomes. |
| P1 | Calendar renders confirmed booking status asPaid and future bookings as appointments today; AI scheduling switch only changes local state. | Use actual payment and scheduling-authority records, correct date filtering, and a persisted setting where implemented. | Confirmed unpaid booking never readsPaid; tomorrow is not today; a switch persists and changes real behavior or is clearly unavailable. |
| P1 | Quote/approval/settings source contracts can discard edited terms or show password/action completion without the intended backend operation. | Map every consequential form field into validated backend requests; echo accepted versions/terms and retain errors. | Edited amounts, split/refund selections and authority limits reach the server; failed saves preserve user input; password changes use the real API. |
| P1 | Query-only legacy success pages can displayDepositPaid; generated customer/share URLs may hit owner login. | Receipt-backed confirmation and reviewed capability/document routes with explicit public eligibility. | URL flags cannot create payment claims; unpublished/revoked resources remain unavailable; intended recipients reach only authorized content. |
| P1 | Chromium found26unnamed narrow-navigation links; dialog samples retained background focus and ignoredEscape. | Accessible responsive navigation and shared dialog primitives. | Nonempty accessible names, visible focus, keyboard reachability, initial/contained/returned focus and safeEscape behavior. |
| P2 | Legacy POS measured1089px width at390px; assistant fixed columns crush the composer. | Review workflow parity before redirecting to canonical apps; then retire duplicates and repair canonical responsive behavior as needed. | Real core workflows remain usable at390px/320px, with no page overflow or offscreen payment/composer controls. |
| P2 | Dashboard aliases contain three different HTML bodies; billing landing dispatch differs byquery. | Compare capabilities and map explicit route ownership. `cost-dashboard.html` defaults to`/plan`, while`?view=cost-dashboard` targets`/cost-dashboard`. | Preserve reviewed terms, IDs, selected views and standalone/embed contracts; no broad name-based replacement. |

## Regression and verification contract

The new browser spec uses repository fixtures, real authentication and actual backend responses. It covers six aliases at two viewport sizes, anonymous boundaries, retained public entry points, an unavailable publication, and real shipped navigation. It attaches canonical screenshots and checks required asset failures. It does not fabricate API success or trigger paid/provider actions.

Source/DOM coverage retains the historical standalone API-reference tooltip contract and replaces retired-copy assertions with file absence, stale-link, canonical tooltip and browser navigation checks. `*.mock-contract.ts` files do not count as browser E2E runs. Required `make lint` and `make test` remain acceptance gates; unavailable native prerequisites or other failures must be reported, never treated as passed. No CI optimization changes, merge or deployment belong to this branch.
