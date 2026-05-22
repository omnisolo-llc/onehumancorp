<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: [integrations] Hybrid Rate Limiter MCP

## Problem Statement
OHC operates in both Cloud-native (multi-tenant) and Standalone (single-user) modes. When the swarm interacts with third-party APIs (e.g., GitHub, Jira, LLMs), strict rate limiting is essential to avoid throttling. Cloud deployments require a distributed rate limiter backed by Redis to synchronize limits across horizontal agent pods. In contrast, Standalone mode needs a zero-dependency local implementation (e.g., token bucket via SQLite or in-memory) to maintain the lightweight architecture. Currently, agents lack a unified MCP Tool for dynamic rate limiting across these environments.

## Research Report
Most existing agentic frameworks configure rate limits statically or rely strictly on a centralized infrastructure like Redis. This breaks down in hybrid architectures where an agent might be executing on a user's local machine without access to a distributed cache. By introducing a Hybrid Rate Limiter MCP, OHC agents can request tokens dynamically. The underlying implementation will route the request to Redis in Cloud mode, or to a local token bucket in Standalone mode, delivering an "Unfair Advantage" for smooth local-to-cloud handoffs without code changes.

## Design Doc
**Architecture:**
- Create a new package `src/server/lib/integrations/rate_limiter/`.
- Introduce a `RateLimiterManager` implementing the MCP Tool interface.
- Dynamically select the backend driver based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Utilize Redis (e.g., via `go-redis`) to implement a distributed token bucket or sliding window algorithm.
- **Standalone Mode:** Implement an in-memory or SQLite-backed token bucket algorithm.

**API Contracts:**
- `RequestTokens(ctx async context, bucket string, amount int) (bool, error)`
- `GetRateLimitStatus(ctx async context, bucket string) (RateLimitInfo, error)`

**Security:**
- Ensure `organization_id` prefixes are rigorously applied to bucket keys in Cloud mode to enforce cross-tenant isolation.

## Implementation Prompt
"Implement the Hybrid Rate Limiter MCP tool in `src/server/lib/integrations/rate_limiter/`.
1. Create `rate_limiter.rs` defining the `RateLimiterManager` and its MCP capabilities (`RequestTokens`, `GetRateLimitStatus`).
2. Implement environment-agnostic logic. To determine if the connection is Cloud, check: `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`.
3. For Cloud mode, implement a Redis-backed token bucket algorithm ensuring `organization_id` is used as part of the cache key.
4. For Standalone mode, implement a robust in-memory or SQLite token bucket.
5. Create comprehensive tests in `rate_limiter_test.rs`, mocking Redis and validating the Standalone local fallback. Ensure 100% test coverage.
6. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies."

## Priority
P2

## Estimated Scope
Medium

</div>
