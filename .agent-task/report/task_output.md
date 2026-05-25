issue_title: "[Architecture] Edge-Caching Dynamic Storefronts for High-Traffic Drops"
issue_description: |
  # [Architecture] Edge-Caching Dynamic Storefronts for High-Traffic Drops

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Maya (custom cakes) often launch new collections, limited drops, or run seasonal sales. During these high-traffic events, a slow or crashing storefront means lost revenue and damaged reputation. Currently, if Maya's vegan cake goes viral on Instagram, or Priya announces a flash sale, the sudden spike in visitors pulling product catalogs, checking variants, and viewing images can overwhelm the backend database. They need their storefronts to remain instantly responsive (sub-50ms) even under massive concurrent load, without having to configure complex "CDN" or "caching" settings themselves.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify:** Utilizes a globally distributed CDN (Fastly/Cloudflare) and edge caching to serve static assets and cached HTML pages. Highly resilient to traffic spikes.
  - **Wix/Squarespace:** Also rely heavily on edge caching for public storefronts, automatically invalidating caches when products are updated.
  - **Vercel/Next.js:** Next.js uses Incremental Static Regeneration (ISR) and Edge Network caching to serve dynamic content at static speeds.

  **Gaps Identified:**
  OHC's current dynamic storefront delivery relies heavily on direct backend database queries for every page load. While this ensures data is always fresh, it creates a severe bottleneck during high-traffic events. We lack a robust, automated edge-caching layer that seamlessly serves storefronts globally, combined with intelligent, instant cache invalidation when inventory or prices change.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Edge Network (CDN)
          EdgeCache[Edge Cache Node];
          EdgeCache --> UserPhone[User's Mobile Browser 375px];
      end

      subgraph OHC Cloud Services
          Gateway[OHC API Gateway] --> EdgeCache;
          StorefrontEngine[Dynamic Storefront Engine];
          StorefrontEngine --> Gateway;
          StorefrontEngine --> CacheInvalidator[Cache Invalidation Queue];
          CacheInvalidator --> EdgeCache;
      end

      subgraph Data & Agents
          MainDB[(Cloud Postgres Ledger)];
          MainDB --> StorefrontEngine;
          Agents[AI Agent Swarm] --> MainDB;
          OpsAgent[Operations Agent: Inventory Sync] --> CacheInvalidator;
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Flash Sale Launch:** Priya posts a link to her new clothing line on TikTok.
  2. **Instant Load:** Hundreds of users click the link simultaneously on their phones. The OHC storefront loads instantly (sub-50ms) because it's served from the nearest Edge Cache node, utilizing the clean, glassmorphism design system.
  3. **Inventory Update:** A customer buys the last "Blue Medium" shirt.
  4. **Agent Action:** The Operations AI Agent records the sale in the unified ledger and immediately triggers a targeted cache invalidation for that specific product variant's data.
  5. **Seamless Refresh:** The next user to click the link sees "Out of Stock" for that variant, with no lag or backend strain.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory levels. When an item is sold out or restocked, or when Priya updates a price via the OHC app, the agent automatically dispatches a message to the Cache Invalidation Queue to purge the stale edge cache.
  - **Marketing Agent:** Can preemptively pre-warm the cache for anticipated high-traffic pages before a scheduled marketing email or social media post goes live.

  ### Key Design Decisions
  - **Stale-While-Revalidate (SWR):** Implement an SWR strategy at the edge. The cache serves the slightly stale page immediately while fetching the fresh page in the background, ensuring the storefront never goes down or stalls.
  - **Tag-Based Invalidation:** Use surrogate keys/tags (e.g., `store-123`, `product-456`) to group cached content. The Operations Agent invalidates by tag, ensuring only the necessary pages are refreshed without flushing the entire store.
  - **Invisible Complexity:** The user (Priya or Maya) never sees "Clear Cache" buttons or CDN settings. The Operations Agent handles all cache lifecycle management autonomously.
  - **Multi-Tenant Isolation:** Edge cache keys must be strictly partitioned by `tenant_id` to prevent cross-tenant data leakage or accidental invalidation.

  ## Implementation Prompt
  Implement an Edge-Caching layer for the Dynamic Storefront Engine.
  - **User-Facing Outcome:** Storefronts must load instantly (sub-50ms) and remain stable under massive concurrent traffic spikes (e.g., flash sales), without requiring any manual configuration from the business owner.
  - **CUJ:** A user launches a high-traffic flash sale. Hundreds of concurrent mobile visitors access the storefront without degrading backend performance. When an item sells out, the storefront updates instantly for subsequent visitors.
  - **Acceptance Criteria:**
    - Storefront product pages and catalogs are cached at the edge.
    - SWR (Stale-While-Revalidate) is used to ensure high availability.
    - Tag-based cache invalidation is implemented and triggered automatically by the Operations AI Agent when inventory or pricing changes.
    - Strict tenant isolation is maintained in the cache layer.
    - Zero developer jargon or configuration options are exposed to the user.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
