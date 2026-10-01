# Growth editor preview boundaries

The coupon, streak, job board, and goal editors preserve their configured values. A preview does not establish a published destination, recorded referral progress, reward eligibility, or a completed claim.

- Coupon: no publication/verification workflow backs the invented `/unlock/my-business` URL. Copy and social controls remain unavailable; no numeric progress is asserted. The configured code is visible in the editor and is not a protected entitlement.
- Streak: no `viral-streak/embed` route is mounted. Embed and Claim Today controls remain unavailable. Day markers are configured examples, not recorded visits.
- Job board: roles are examples. Publication, application tracking, referral attribution and rewards are unavailable. Theme buttons update the actual preview and expose their exclusive selected state.
- Goal: the mounted `/api/v1/growth/viral-goal-tracker` embed remains copyable. Query fields are URL-encoded and then escaped for the HTML attribute. Its backend parameter is `hideBranding`. Clipboard feedback awaits the platform and retires stale completion. Individual referral progress and share-to-goal actions remain unavailable.

The configured tenant field comes from the existing editor configuration. It is not a newly verified owner identity. This repair preserves the existing Pro policy and does not authorize rewards, entitlement grants, social sends, or new tracking. Public embed entitlement enforcement and owner-bound configuration remain separate capability gaps.

Regression tests exercise real rendered components with isolated platform boundaries. Browser execution, provider verification, and a complete publication/reconciliation workflow remain separate release gates. This patch is reconstructed from the preserved recovery specification, with fresh source-bound tests; it is not claimed to be the lost original Git object.
