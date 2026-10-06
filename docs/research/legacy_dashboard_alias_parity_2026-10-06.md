# Legacy dashboard alias parity and bounded repair

Source base: `4c56263f640726370e1e73f4e7d9b8b40b79c6d1` (tree `1b4a7e26778ac5d973635b9442d615796b60cdc9`).

## Exact dispositions

| Browser path | Disposition | Reason |
| --- | --- | --- |
| `/api/ui/dashboard.html` | Authenticated ordinary GET/HEAD redirects to `/dashboard`; obsolete HTML removed | Older dashboard subset has maintained replacements below |
| `/api/v1/ui/dashboard.html` | Same exact redirect and removal | Same older implementation |
| `/dashboard.html` | Retained; receipt, readiness and invitation fixes | Larger standalone-tool navigation and interactions lack fully reviewed parity |
| `/ui/dashboard.html` | Retained; identical fixes | Same larger maintained compatibility surface |
| `src/ui/tauri/src/ui/dashboard.html` | Historical source retained with matching fixes | Not a current packaged frontend; preserve standalone compatibility |

Retirement uses the existing exact proxy map after authentication. Query order, duplicate parameters and encoding are preserved. Browser fragments remain browser-owned. RSC/prefetch, unsafe methods, anonymous API-path401, expired sessions, near misses and shared assets keep the existing boundary. Navigation in other API-prefixed HTML files now points directly to `/dashboard`.

## Older API-dashboard capability map

| Existing capability | Maintained destination or preservation |
| --- | --- |
| Unified agent feed and owner decisions | `/dashboard` → `UnifiedAgentFeed`; durable decision receipts, no delivery claim |
| Recorded time savings and sharing | `AiTimeSavingsWidget`; distinguishes unavailable measurement and preserves an X share intent for the current verified recorded estimate, without a trial/reward claim |
| Order milestones and invitation sharing | `SuccessMilestoneWidget` / `RecordedOrderMilestone` and `DashboardViralInviteWidget` |
| Invitation creation, copy and X sharing | Existing owner-bound `useCloudInvitation`; failures cannot manufacture links |
| WhatsApp invitation sharing | Preserved on `DashboardViralInviteWidget`, only after a verified receipt |
| Automation / trial setup | `SoftPaywallWidget`; verified plan/availability, no invented entitlement or activation |
| Agent Card navigation | Explicit `/agent-card.html` link retained on `/dashboard`; target HTML is not deleted |
| Certificate Generator navigation | Explicit `/viral-certificate-generator.html` link; target HTML is not deleted |
| Giveaway navigation | Existing `/giveaway` dashboard card |
| Help, walkthrough and workspace search | Maintained shell/widgets/Omnibox |

This is a navigation and feature-reuse map, not a claim that every destination's broader business capability is implemented or provider-verified. In particular, truthful unavailable trial/automation states are not trial granting or activation.

## Retained dashboard repairs

- Single and batch decisions require an HTTP200, `decision_recorded:true`, matching tenant/item/state and matching edited draft when sent. Legacy nested receipts also require `success:true` and a valid saved-edit field.
- Pending body reads do not acknowledge a decision. Invalid/error/contradictory receipts retain the card and draft. Partial batches report recorded counts and retain unconfirmed work. The UI never calls a recorded decision provider execution.
- Before dispatch, an actor/tenant/item/state/edit-bound unknown marker is persisted under an origin lock. Unavailable storage or locking prevents a write. Signed session identity is read and bound through expected-owner headers.
- Unknown markers survive reload. Reload and the explicit recovery action read `/api/v1/agent-feed/{id}/decision` or `/api/v1/ui/triage/decisions/{id}`; no mutation is automatically replayed. A missing/malformed receipt retains the original hold and draft. A different actor cannot release it. Known server rejections with `decision_recorded:false` permit a corrected explicit request.
- Batch progress retains each confirmed receipt and reads it on later reconciliation rather than posting again. Duplicate clicks are fenced; changed intent is held until the original outcome is resolved. Before saving a confirmed/rejected phase or enabling controls, the captured identity epoch and a fresh verified same-owner read must still match. A late GET or write acknowledgement after retirement leaves the original hold unknown and controls disabled.
- Every actual card render, including tab changes, websocket updates and group reconstruction, synchronously reapplies unknown holds. Exact held drafts are shown only after the original owner is freshly verified; another actor cannot read them from a restored card. Group controls remain held while any member outcome is unknown.
- Unavailable feeds expose retry plus the maintained dashboard link; neither error nor empty/loading cards declare a completed business/storefront.
- Milestone/referral sharing reads the existing invitation bridge's current owner-bound receipt. No timestamp, tenant-derived invitation or fallback URL is constructed. All invitation mirror fields and dependent controls retire together on authentication, storage, pagehide and expiry. Clipboard failures and late identity changes do not report success or revive a retired control. Simple milestone sharing remains available without claiming an invitation or reward.
- Shared HTML modules, agent cards, certificates and historical Tauri sources remain available. No unrelated pages were deleted.

## Verification boundary

`legacy-dashboard-decisions.test.mjs` automatically discovers shipped dashboard files and requires the reviewed two-retired/two-retained classification. It compares retained handler/module copies, rejects constructed invitation URLs, checks canonical link migration and requires explicit real-stack retirement cases.

`legacy-dashboard-retirement.spec.ts` adds six real-backend browser cases: both aliases at1440px and390px with authenticated redirect/query/fragment/history checks, an actual seeded owner decision with API receipt and database readback, and both anonymous access boundaries. No network substitution is used. These cases have been discovered but have not executed in this VM: the native runner stops on the missing source-built `target/debug/server`. Cargo, Docker and permitted Chromium sockets are also unavailable. Full release acceptance and visual verification remain pending on a supported native environment.

Focused script, component, middleware and type checks are separate from real-stack/provider evidence. This bounded patch does not certify all remaining legacy dashboard widgets or the repository-wide mock-remediation backlog.

## Follow-up review correction and backend contract

The first patch revision used only an in-memory in-flight set, which would have released unknown writes after a failed response. Review rejected that as insufficient. The revised contract above persists exact original intents, reconciles with GET only and retains partial-batch receipts. The narrow new legacy GET handler shares canonical owner authorization and stored-receipt decoding with POST replay, but does not call the mutation path or execute effects. Missing receipts never authorize automatic retry.

Six new backend contract cases (three each for real SQLite and PostgreSQL) cover no-record/readback without re-execution, foreign/member/demoted owner denial, and corrupt receipts. They run in the existing guarded database gate; its required inventory floor is now 109. Source preparation succeeds, but native compilation/database execution is unverified in this VM.

Open outside this bounded repair: legacy dashboard search result clicks still show a navigation alert instead of navigating. Other retained growth widgets and backend dashboard error/empty-state projection gaps remain part of the broader audit backlog. Retaining those pages is not a claim that all mock behavior has been eliminated.


## Independent-review corrections

The first independent review of candidate `4ccdf2aeedeb8ba1b62bd2969c67cafde1732878f39adbe887c5be16579bd18b` reproduced two defects with production renderer/consumer code: a rerender re-enabled an unknown decision and replaced its held draft, and a secondary invitation input retained the former owner's invitation after invalidation. It also found that retiring the API dashboard removed legitimate measured-savings X sharing along with the unverified reward claim. The revised candidate addresses all three. Regression tests exercise actual card/group renders, restored foreign-owner drafts, actual referral/milestone consumers, every invitation retirement signal, late clipboard success/failure, and truthful current-owner measured-savings sharing. These DOM/component checks do not replace the still-unrun native/database/browser release gates.
