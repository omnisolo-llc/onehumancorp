# Cost Optimization (Miser)

## Implemented Features
1. **Prompt Caching**: `src/server/pricing/prompt_caching.rs` implements a TTL-based cache to bypass LLM generation costs for repeated requests.
2. **Storage Compression**: `src/server/pricing/compression.rs` provides `optimize_image` (WebP conversion) and string compression techniques to reduce storage and CDN transit costs. A `bandwidth_savings` calculator in `src/server/pricing/calculator.rs` measures the savings.
3. **Transaction Fee Routing**: `src/server/pricing/payment_routing.rs` routes payments strategically to minimize Stripe transfer fees (e.g. batching small payouts, using ACH for large amounts).
4. **Rate Limiting & Quotas**: `src/server/pricing/rate_limit.rs` and `src/server/pricing/budget.rs` provide Redis-backed soft rate limits and storage quotas per tenant plan tier.
5. **Cost Transparency Dashboard**: `src/server/api/billing_api.rs` and frontend implementations provide detailed, accurate cost tracking, including `Network Cost` and `Bandwidth Savings`.

## Resolved Blockers
1. The Playwright end-to-end tests for the cost dashboard were previously blocked universally in the CI sandbox environment by a Docker daemon/containerd issue (`failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`) when attempting to run `pgvector/pgvector:pg16` or `postgres:16-alpine`.
   - **Resolution**: Switched the `deploy/docker-compose.e2e.yml` to use `pgvector/pgvector:pg15` and `redis:7` images and added `seccomp:unconfined`, `apparmor:unconfined` and capabilities to `security_opt` and `cap_add`. Full UI tests now run in CI and locally through the Bazel Playwright aggregate.
