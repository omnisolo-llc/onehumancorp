issue_title: "[Architecture] Edge-Caching Dynamic Storefronts"
issue_description: |
  # Research Report & Design Doc: Edge-Caching Dynamic Storefronts

  ## Problem Statement
  Currently, OneHumanCorp (OHC) dynamically serves storefronts from the central Go API server. For small-business personas like Maya (home baker) or Priya (boutique operator), their online storefronts must load instantly, even on slow mobile networks, and must be optimized for search engine indexing (SEO). Waiting for database queries and central server rendering on every request degrades the user experience and impacts SEO ranking.

  ## Research Report
  Industry leaders (Shopify, Wix, Next.js Commerce) heavily utilize edge computing and Content Delivery Networks (CDNs) to serve storefront pages.
  - **Shopify**: Uses an extensive edge network (Shopify Oxygen / CDN) to cache dynamic Liquid pages, only falling back to the core database for personalized data (cart, checkout).
  - **Wix**: Aggressively caches rendered HTML at the edge.
  - **Next.js**: Employs Incremental Static Regeneration (ISR) to cache pages at the edge and rebuild them in the background when data changes.

  **OHC's Approach**: OHC should implement a decentralized edge-caching architecture. Storefront HTML and catalog data should be cached at the edge (CDN layer). When a product price changes, an inventory item sells out, or an offer is updated, the OHC system must intelligently invalidate the edge cache.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      BuyerMobile[Buyer Mobile Browser] --> CDN[Edge CDN / Cache Layer];
      CDN -- Cache Miss / Dynamic --> API[Go API Server];
      API --> DB[(PostgreSQL)];
      API --> EdgeCacheManager[Edge Cache Invalidator];
      EdgeCacheManager -- Webhooks / API --> CDN;
      Agent[Operations Agent] --> API[Update Catalog/Inventory];
  ```

  ### Core Components
  1.  **Edge Cache Layer**: An external layer (e.g., Cloudflare, Fastly, or a custom Varnish/Redis edge) that caches HTTP `GET` responses for public storefront routes (`/store/*`).
  2.  **Cache Control Headers**: The Go API must emit precise `Cache-Control`, `ETag`, and `Surrogate-Key` (or `Cache-Tag`) headers for storefront content.
  3.  **Invalidation Engine**: A robust mechanism within the OHC server. When a mutation occurs (e.g., inventory decreases, product updated), the engine must trigger an invalidation request to the Edge Cache using the specific `Cache-Tag` associated with the tenant or product.

  ### Mobile UX Flow (375px) - Non-Technical Owner
  - **Owner View**: The owner (Maya) updates a cake's price in her OHC app. She taps "Save".
  - **System Action**: The app updates the DB. The Invalidation Engine immediately pings the CDN.
  - **Result**: Maya's customer instantly sees the new price on their phone. The owner is unaware of "edge caching"—it just feels "fast and correct."

  ### AI Agent Integration
  - **Operations Assistant**: When the agent automatically updates a menu item because ingredients are out of stock, it utilizes the same standard mutation paths, inherently triggering cache invalidations without requiring special "agent-to-CDN" logic.

  ## Implementation Prompt
  Implement the foundational `Cache Invalidator` service and middleware in the Go backend.
  1.  Define an interface for `EdgeCacheProvider` with a method to `InvalidateTags(ctx context.Context, tags []string) error`.
  2.  Provide a default implementation (e.g., a no-op or Redis-backed local cache invalidation for testing).
  3.  Implement Go HTTP middleware that automatically attaches `Cache-Tag` headers to public storefront routes (e.g., tagging by `tenant_id` and `product_id`).
  4.  Hook the `InvalidateTags` call into the core product/inventory mutation endpoints (ensure this is non-blocking or managed via the background job queue using the `SKIP LOCKED` pattern).
  5.  Write unit tests and E2E tests verifying that after a product update, subsequent fetches reflect the new data (simulating edge invalidation). Do not prescribe a specific external CDN vendor (e.g., Cloudflare) in the core logic.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
