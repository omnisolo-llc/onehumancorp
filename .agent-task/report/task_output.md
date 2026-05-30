issue_title: "Research: Redis Token Bucket Rate Limiting for Background Job Queue"
issue_description: |
  # Research Report: Redis Token Bucket Rate Limiting

  ## Problem Statement
  As OneHumanCorp (OHC) scales, the background job queue (currently relying on Postgres `SKIP LOCKED`) needs robust traffic regulation. We require a distributed rate-limiting architecture to prevent "noisy neighbor" scenarios where a single tenant's massive job spike (e.g., 10,000 AI operations or webhooks) starves other tenants across the multi-tenant SaaS platform. Simple per-process rate limits fail in a distributed K8s environment, and adding this burden to Postgres causes lock contention and degrades performance.

  ## Research Findings
  - **Current State:** The backend uses `SKIP LOCKED` in Postgres for dequeuing jobs, but lacks distributed tenant-level rate limiting for background workers (AI Agents, webhooks, etc.).
  - **Industry Standard:** Distributed platforms (Stripe, Shopify, Cloudflare) implement generic Token Bucket or Sliding Window algorithms in Redis for millisecond-level precision without database overload.
  - **Token Bucket algorithm:** Perfect for background job queues because it allows bursts of activity up to the bucket capacity while strictly enforcing a steady refill rate.
  - **Redis Implementation:** Utilizing Redis Lua scripts guarantees atomic operations for check-and-decrement (taking a token), which is critical for concurrent worker nodes.
  - **Current OHC Codebase:** The platform already uses Redis for rate limiting (e.g., `src/server/pricing/rate_limit.rs`), but it primarily tracks monthly action limits, storage quotas, and product counts. It does NOT implement a generic Token Bucket for high-frequency job throttling.

  ### Competitive Analysis
  - **Shopify:** Utilizes Redis token buckets for their GraphQL Admin API and internal background job throttling per shop.
  - **Stripe:** Implements multi-layered Redis rate limiters for API requests and webhook delivery retries to ensure fairness and prevent system collapse.
  - **OHC:** Needs a per-tenant, per-resource token bucket in Redis (e.g., `rate_limit_bucket:{tenant_id}:{job_type}`).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Worker1[AI Agent Worker Node 1] --> |Take Token| RedisCluster[(Redis: Token Bucket via Lua)]
      Worker2[AI Agent Worker Node 2] --> |Take Token| RedisCluster
      RedisCluster -.-> |Token Granted| Worker1
      RedisCluster -.-> |Limit Exceeded| Worker2
      Worker1 --> |Dequeue| DB[(PostgreSQL `SKIP LOCKED`)]
      Worker2 -.-> |Backoff / Yield| QueueLoop
  ```

  ### Implementation Strategy
  1.  **New Token Bucket Component:** Add a `TokenBucketRateLimiter` to the existing rate limiting infrastructure (`src/server/pricing/rate_limit.rs` or a new module `src/server/utils/token_bucket.rs`).
  2.  **Lua Script:** Write a Redis Lua script to handle the token bucket logic atomically:
      - Parameters: `key`, `capacity`, `refill_rate_per_second`, `now` (timestamp).
      - Logic: Calculate time elapsed since last refill, add new tokens (up to capacity), check if 1 token is available. If yes, decrement and allow. If no, deny.
  3.  **Worker Integration:** In the background worker loops (`src/server/queue.rs` or similar job processors), before attempting to dequeue a job for a specific tenant, check the token bucket.
  4.  **Fallback Strategy:** If Redis is down, fail-open (allow the job to process) to prevent complete system halting, but log a critical alert.

  ### Mobile UX & AI Impact
  - **Mobile UX:** Invisible to the user, but ensures the OHC mobile app remains snappy for all users, as background tasks won't drag down the shared database.
  - **AI Agents:** Prevents one user's massive catalog import (triggering thousands of AI tasks) from delaying another user's single AI quote generation.

  ## Implementation Prompt
  Implement a Redis-backed Token Bucket Rate Limiter for background job processing.
  1.  Create a `TokenBucketRateLimiter` struct in Rust.
  2.  Implement a Lua script within this struct to atomically check and consume tokens (parameters: key, capacity, refill rate).
  3.  Provide a method `async fn check_limit(&self, tenant_id: &str, resource_type: &str, capacity: u32, refill_rate: u32) -> Result<bool, String>`.
  4.  Ensure comprehensive unit tests verifying the token bucket logic (bursts, steady refill, denial when empty).
  5.  (Optional for this initial PR depending on scope) Integrate this check into the main job dequeue loop to yield processing if a tenant is rate-limited.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
