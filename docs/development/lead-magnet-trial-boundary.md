# Lead-magnet sharing and trial boundary

Source baseline: `cf18d44ed455dfb254186b1d67829cd6e138801d`. The maintained standalone builder and its live dashboard entry points are repaired together; historical `next_out` output is not a release source.

The builder advertises branding removal for a Pro plan. Its old share button advertised a free seven-day Pro trial, opened an X intent, then called the claim endpoint and treated any successful HTTP response as verified sharing. It persisted `has_pro=true` in browser storage. Neither opening a popup nor browser storage establishes posting, eligibility or an entitlement.

The mounted Rust `handle_trial_extension_claim` checks one `has_claimed_trial_extension` flag and writes `tier='pro'`, `plan_tier='pro'` and the claimed flag. It does not record a seven-day expiration or verify a social post. Its missing-tenant branch also ignores insertion errors before returning success. The Next claim route is an authenticated proxy to this handler. The separate modern trial page instead describes “Pro Activation,” so existing surfaces disagree about duration. A targeted search of repository research, plans and specifications found no normative duration/eligibility/verification policy resolving that conflict; existing tests repeat the implementation rather than establish such a policy.

This repair does not invoke or change that grant endpoint. Trial unlock remains explicitly unavailable. The optional share control only requests an X draft and never claims a post or reward. Existing signed-in session identity and `billing/my-plan` reads govern owner binding and the same Pro/Business preview eligibility used by the maintained `useProPlan` hook. The plan GET includes both expected-owner headers from the first verified identity; the authenticated proxy checks them against its actual session, covering an A→B→A session switch. If an opaque identity cannot round-trip as HTTP headers, plan eligibility stays held instead of sending an unbound request. Unverified identity holds embed generation; unverified plans keep branding. Canonical auth invalidation and expiry clear the binding. No local flag authorizes branding removal.

Generated snippets use the actual same-origin public lead-magnet route and its supported query names. URL construction and DOM attribute serialization preserve opaque tenant IDs and user text. Copy feedback waits for the clipboard promise. The builder's audit button and prompt editor are previews, not proof of AI processing or delivery.

Remaining capabilities and boundaries:

- A supported trial needs a specified duration, eligible actors, authoritative verification, expiry/revocation semantics and a transactional receipt. It is not implemented by this repair.
- The existing public Next embed still accepts its branding query without a tenant entitlement lookup; that server-side policy gap is separate from the repaired owner preview. Do not claim the preview check secures the public endpoint.
- The capture endpoint records a Hub event; AI report generation and email delivery are not certified by the builder.
- Local DOM tests execute the actual inline scripts with isolated identity/plan and clipboard boundaries. They make no external social request and do not certify a real post or grant. Hosted Playwright remains the real browser gate.
