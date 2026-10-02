# Team invitation boundary

The mounted team widget uses the same `useCloudInvitation` hook as the referrals page. A verified, unexpired session identity and the existing paired expected-owner headers bind the request to the authenticated account. Local display names no longer select invitation authority. The existing generic team-link value `pending-invite` remains the invitee field; creating a link is not delivery or joining.

The shared origin Web Lock and per-owner metadata key are identical to the maintained React and legacy dashboard callers. Metadata is written before dispatch, and an uncertain outcome stays held across reopening. Only the mounted HTTP 200 receipt with its actual valid invitation URL is displayed. Lifecycle retirement removes the previous account's link and invalidates pending clipboard feedback. No live invitation is created by the local tests.

The widget no longer announces a fixed tenth order or supplies a fabricated default-team milestone card/link. It explicitly requires a verified business order count. The storefront embed remains a preview based on existing local configuration and now states that identity and referral rewards are unverified; its clipboard operation still awaits the real platform result.

## Remaining capabilities

The browser coordination does not establish server-side or cross-device idempotency, scoped invitation listing/reconciliation, redemption, or a reconciliation UI. Those remain separate incomplete capabilities. The public embed requires verified business binding and entitlement enforcement before it can be represented as an authoritative business publication. Real milestones still need tenant-scoped, source-defined order semantics; no count, payment, revenue, currency conversion, or reward grant is inferred by this change.

Unit coverage mounts the real component/hook and uses explicit HTTP/storage/lock test boundaries. It covers identity readiness, expected-owner headers, contradictory receipts, late auth/storage/pagehide responses, unknown outcomes after reopening, shared callers, and truthful milestone/embed copy. Existing native clipboard promise tests remain. Hosted browser acceptance is still required; focused tests are not full repository certification.
