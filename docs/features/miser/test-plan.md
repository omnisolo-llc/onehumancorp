# Miser Test Plan

## Automated Tests
- `src/server/pricing/prompt_caching.rs`: Verify SHA-256 consistency and cache hits.
- `src/server/storage/local_provider.rs`: Verify images are smaller after optimization.
- `src/server/pricing/quota_test.rs`: Verify soft limits trigger upgrade prompts.

## Manual Verification
- Verify the "My Plan" screen renders correctly on mobile (375px width).
- Verify the "Upgrade" button links to Stripe Billing.
