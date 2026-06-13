issue_title: "Implement Edge-Cached Dynamic Storefront Content & SEO Rendering"
issue_description: |
  # Edge-Cached Dynamic Storefront Content & SEO Rendering

  ## Problem Statement
  Currently, the OHC platform lacks a high-scale mechanism for generating edge-cached dynamic storefront content that is instantly visible to search engines and extremely fast for mobile consumers. Maya the baker and Carlos the handyman need their portfolios, pricing, and custom offers to load instantaneously on customer phones, while remaining deeply indexable by search engines (SEO) to capture organic local demand. If the storefront relies exclusively on centralized databases during traffic spikes, it will suffer from poor SEO and high latency on slow mobile networks. The lack of an edge-cached rendering strategy and autonomous invalidation limits the growth and reliability for OHC users.

  ## Research Report
  - **Market Context:** Non-technical SMB owners rely on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Search engines also struggle to index slow, client-side rendered dynamic content, reducing organic discoverability.
  - **Competitor Analysis:**
    - **Shopify:** Offers strong edge network capabilities (via Cloudflare) for fast global delivery of storefronts.
    - **Vercel / Next.js:** Defines the modern standard for developers (ISR, Edge computing), but is inaccessible to non-technical users.
    - **Wix / Squarespace:** Provide easier SEO tools but still require manual configuration and lack autonomous instant scalability during unexpected spikes.
  - **OHC Approach:** OHC's differentiation is being **invisible and autonomous**. It requires Universal Edge Caching where all storefront reads hit a global edge cache automatically. More importantly, it requires **Agentic Cache Invalidation** (when the Operations Agent updates inventory, it instantly purges the cache key) and **Agentic SEO Pre-rendering** (when the Marketing Agent updates the website, it autonomously triggers pre-rendering of static HTML to the edge).

  ## Design Doc
  - **Architecture:**
    - **Storefront Engine & Edge Cache Layer:** A storefront rendering layer that outputs static HTML/JSON with appropriate cache-control headers (e.g., `s-maxage`, `stale-while-revalidate`) integrated with a global CDN (Cloudflare/Fastly).
    - **Message Bus Integration:** Leverages the existing `OHCJobQueue` and PostgreSQL SKIP LOCKED pattern for background job dequeue.
    - **Invalidation Worker:** A background worker (e.g., `CacheInvalidationWorker`) in Go that subscribes to inventory/storefront update events via the job queue and issues purge requests to the CDN.
  - **Mobile UX Flow:**
    - Customer taps an Instagram link.
    - Request hits Edge CDN and returns pre-rendered HTML in under 100ms.
    - Customer views the 375px mobile storefront instantly with no loading spinners.
  - **AI Agent Integration Points:**
    - The Operations/Commerce Agent handles inventory updates and automatically queues cache invalidation tasks without owner intervention.
  - **Key Design Decisions:**
    - Implement a "stale-while-revalidate" strategy.
    - Ensure multi-tenant isolation using cache keys like `storefront:{tenant_id}:{resource_id}` or `storefront:{tenant_id}:{path}`.
  - **Architecture Diagram:**
    ```mermaid
    sequenceDiagram
      participant Owner
      participant Agent as Operations Agent
      participant Queue as Job Queue
      participant Worker as Invalidation Worker
      participant CDN as Edge CDN

      Owner->>Agent: Mark "Vegan Cake" out of stock
      Agent->>Agent: Update Inventory DB
      Agent->>Queue: Enqueue Invalidation Event
      Queue->>Worker: Dequeue Event
      Worker->>CDN: Purge Cache Key (storefront:tenant_id:items)
      CDN-->>Worker: Ack Purge
    ```

  ## Implementation Prompt
  - **Objective:** Implement the backend caching strategy, cache headers, and the invalidation worker for the Universal Edge-Cached Storefront.
  - **CUJ:** As an OHC owner (Maya), when my AI assistant marks an item as "out of stock", my public storefront immediately updates for new visitors via an edge cache invalidation, without requiring me to manually publish or rebuild the website.
  - **Acceptance Criteria:**
    1. Create a `CacheInvalidationWorker` (or equivalent Go queue consumer) that listens to inventory/offer update events.
    2. Define the schema for storefront cache keys (e.g., `storefront:{tenant_id}:{resource_id}`).
    3. Implement a mock/stub CDN invalidation service (or integrate with an existing caching layer if present) that successfully processes the invalidation event.
    4. Provide 100% unit test coverage for the cache invalidation logic.
    5. Update relevant API endpoints to set proper HTTP cache-control headers for public storefront routes.

  ## Top 5 Things That Do Not Make Sense (For Future Optimization)
  1. No native mobile app yet despite mobile-first constraints.
  2. Disconnect between the internal gRPC APIs and how the Flutter web clients handle latency in low-bandwidth scenarios.
  3. The `ohc_job_queue` implementation mixes scheduling and execution retry logic in the database, risking high contention at scale.
  4. The current caching layer relies heavily on Redis but doesn't have a clear fallback/degraded operation mode when Redis connection is flaky.
  5. The AI agent memory relies solely on DB tables without a dedicated semantic vector search implementation for faster semantic retrieval.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
