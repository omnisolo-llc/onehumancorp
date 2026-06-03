issue_title: "[Architecture] Decentralized Edge Multi-Region Dynamic Storefront Caching"
issue_description: |
  # [Architecture] Decentralized Edge Multi-Region Dynamic Storefront Caching

  ## Problem Statement

  Maya (The Home Baker, 28) and Priya (The Boutique Owner, 35) rely on their digital storefronts to be fast and responsive, regardless of where their customers are located. Currently, all storefront requests route back to the central OHC core platform (PostgreSQL/Redis) in a single region. This means customers halfway across the world experience high latency (300ms+), reducing conversion rates. Furthermore, if the core database experiences a spike (e.g., a viral TikTok post for Priya's new dress), the entire platform's database is strained, potentially degrading the experience for all other tenants.

  The core issue is that our dynamic storefronts are not cached at the edge. We need a decentralized, multi-region edge caching layer that can serve dynamic tenant storefronts globally with sub-50ms latency, while still supporting real-time invalidation (e.g., when a product sells out).

  ## Research Report

  - **The Status Quo:** Shopify heavily utilizes Cloudflare Workers and a distributed KV store to serve their storefronts from the edge. Wix and Squarespace also use aggressive edge caching. Our current single-region approach is an architectural bottleneck.
  - **The OHC Differentiator:** Our users expect "invisible magic." They don't configure CDNs or cache headers. The platform must automatically push their catalog, pricing, and availability to the edge globally, and invalidate it instantly when inventory changes or they update their site design.
  - **Architectural Gap Discovered:** Lack of a global Edge KV/Cache layer integrated with our multi-tenant inventory and storefront rendering engine.
  - **Goal Targets:**
    - Edge latency for storefront rendering < 50ms globally.
    - Cache invalidation latency < 2 seconds globally.
    - Zero configuration for the business owner.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      subgraph "Customer (Global)"
          Browser[Mobile Browser]
      end

      subgraph "Edge Network (Multi-Region)"
          EdgeNode[Edge Worker / CDN]
          EdgeCache[(Distributed Edge KV Store)]
          Browser --> EdgeNode
          EdgeNode --> EdgeCache
      end

      subgraph "Core OHC Multi-Tenant Platform"
          StorefrontService[Storefront Rendering Service]
          InventoryService[Inventory Ledger Service]
          CoreDB[(Global PostgreSQL DB)]
          CacheInvalidator[Edge Cache Invalidator]
      end

      EdgeNode -->|Cache Miss / Dynamic Cart| StorefrontService
      StorefrontService --> CoreDB
      InventoryService --> CoreDB
      InventoryService -->|Item Sold / Updated| CacheInvalidator
      CacheInvalidator -->|Purge Key / Sync| EdgeCache
  ```

  ### Mobile UX Flow (375px baseline)
  - The business owner (Maya) sees no changes in her UI. She updates a product description or price in the OHC App.
  - The customer navigating Maya's storefront on their phone experiences instant page loads, smooth glassmorphic transitions, and no waiting for catalogs to populate.
  - Cart and checkout operations (which require strong consistency) bypass the edge cache and route to the core platform seamlessly.

  ### AI Agent Integration Points
  - **Marketing Dept:** If a product goes viral and cache hit rates spike, the Marketing Agent alerts the owner and suggests running a promotional campaign.
  - **Operations Dept:** Monitors global cache invalidation delays and can temporarily enable "optimistic inventory" if edge sync falls behind.

  ### Key Design Decisions
  - **Cache Key Strategy:** Keys must be scoped by `tenant_id` and resource (e.g., `storefront:{tenant_id}:catalog`).
  - **Stale-While-Revalidate:** Edge nodes should serve stale content while asynchronously fetching fresh data on cache expiry, masking latency from the user.
  - **Cart/Checkout Exclusion:** Paths requiring strict transactional consistency (e.g., `/checkout`, `/cart/add`) must always bypass the cache and hit the core `StorefrontService`.

  ## Implementation Prompt

  **To the Implementer Swarm:**
  Implement the Edge Caching synchronization layer for the dynamic storefronts.

  **Acceptance Criteria:**
  1. Define a robust cache key schema supporting multi-tenancy.
  2. Implement the `CacheInvalidator` service that listens for inventory/catalog updates and purges the relevant edge cache keys.
  3. Ensure that critical transactional paths (checkout) are explicitly excluded from the edge cache.
  4. Write comprehensive unit and E2E tests simulating a cache miss, cache hit, and cache invalidation cycle.
  5. Latency metrics for cache hits must be instrumented via OpenTelemetry.

  ## Priority
  `P1`

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
