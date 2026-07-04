issue_title: "Implement Edge-Cached Dynamic Storefront for High-Performance Delivery"
issue_description: |
  # Architecture Discovery: Edge-Cached Dynamic Storefront

  ## Problem Statement
  Small business owners like Maya (the baker) or Carlos (the handyman) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) for SEO.

  ## Research Report
  Based on the competitive landscape (Shopify, Wix, Vercel), the gold standard for high-performance e-commerce is edge caching. OHC's current architecture (Go API + PostgreSQL) handles dynamic rendering but lacks an integrated, invisible caching layer for public storefronts. Every storefront read currently hits the database.

  We need a "Universal Edge-Cached Dynamic Storefront" combined with "Agentic Cache Invalidation." When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific cache key globally.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant CustomerBrowser
      participant OHC_API (Go)
      participant CacheService (Redis/Valkey)
      participant PostgresDB
      participant AgentOps

      CustomerBrowser->>OHC_API: GET /api/v1/storefront/{tenant_id}/catalog
      OHC_API->>CacheService: GET store_cache:{tenant_id}
      alt Cache Hit
          CacheService-->>OHC_API: Return JSON
      else Cache Miss
          OHC_API->>PostgresDB: SELECT catalog details
          PostgresDB-->>OHC_API: Return data
          OHC_API->>CacheService: SET store_cache:{tenant_id} (TTL: 1 hour)
      end
      OHC_API-->>CustomerBrowser: Fast Response

      AgentOps->>PostgresDB: UPDATE inventory (Item Sold Out)
      AgentOps->>CacheService: DEL store_cache:{tenant_id} (Agentic Invalidation)
  ```

  ### UI/UX Flow
  - **Owner (Maya):** Sees no new technical settings. Her experience is completely invisible.
  - **Customer:** Experiences sub-100ms load times for the storefront catalog, even during heavy traffic.
  - **Mobile:** Mobile payloads are served instantly from memory, respecting low-data constraints.

  ### AI Agent Integration
  - **Operations Agent:** Intercepts inventory decrement events and actively invalidates the `store_cache:{tenant_id}` key in Redis.
  - **Marketing Agent:** Can trigger a pre-warm of the cache after creating a new promotion or updating catalog descriptions.

  ## Implementation Prompt
  Implement a read-through caching layer for the public storefront endpoints (e.g., `/api/v1/storefront/...`).
  - Introduce a `CacheService` module in Go that interfaces with Redis.
  - Modify the storefront catalog retrieval to check Redis first.
  - Upon a cache miss, fetch from PostgreSQL, serialize to JSON, and store in Redis with a TTL.
  - Implement cache invalidation hooks triggered by state-mutating actions (like inventory updates or catalog edits).
  - Ensure the implementation is fully covered by unit tests and does not break existing Playwright E2E flows.
  - The cache key MUST incorporate the `tenant_id` to maintain strict multi-tenant isolation.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
