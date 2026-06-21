issue_title: "Implement Edge-Cached Dynamic Storefront pre-rendering and Agentic Cache Invalidation"
issue_description: |
  # Mission Queue Protocol Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Search engines also struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## Research Report
  - **Market Context**: Platforms like Shopify provide strong edge network capabilities (via Cloudflare). Vercel/Next.js ecosystem provides the gold standard for developers (ISR, Edge computing) but is inaccessible to non-technical users.
  - **OHC Opportunity**: Implement a "Universal Edge Caching" layer that seamlessly caches all storefront reads globally. This will be paired with an "Agentic Cache Invalidation" capability where the Operations Agent instantly purges edge cache keys when inventory changes.
  - **Agentic SEO Pre-rendering**: When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process, generating highly optimized, static HTML injected with meta tags and pushing it to the edge cache. This boosts organic ranking without the user lifting a finger.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] -->|Hits Edge| B{Edge Cache (e.g., Cloudflare)}
      B -->|Cache Hit| C[Static Pre-rendered HTML]
      B -->|Cache Miss| D[OHC API Gateway]
      D --> E[PostgreSQL DB]
      F[Operations Agent] -->|Inventory Update| G[Cache Invalidator Service]
      G -->|Purge Key| B
      H[Marketing Agent] -->|Storefront Edit| I[SEO Pre-render Service]
      I -->|Generate & Push Static HTML| B
  ```

  ### Mobile UX Flow
  - This feature is entirely invisible to the non-technical owner. There is no new UI configuration required in the 375px mobile app. The "Agentic Cache Invalidation" and "SEO Pre-rendering" operate purely in the background.

  ### AI Agent Integration Points
  - **Operations Agent**: Needs to emit a `CACHE_INVALIDATE` event containing the relevant resource keys (e.g., `product:123`) whenever an inventory count changes or a product is updated.
  - **Marketing Agent**: Needs to emit a `PRE_RENDER_REQUEST` event when storefront layouts or catalog metadata changes.

  ### Key Design Decisions
  - **Zero Configuration**: The user must not need to know what "CDN" or "Edge Cache" means.
  - **Event-Driven Invalidation**: Rely on the existing OHC Event Mesh/Queue to distribute cache invalidation events.

  ## Implementation Prompt
  **User-Facing Outcome**: Maya updates the price of her custom cake on her phone. Within milliseconds, the cache is invalidated globally. A customer visiting from the other side of the country instantly sees the new price from a fast local edge node, and Google's crawler indexes the fully pre-rendered static HTML version.

  **CUJ & Acceptance Criteria**:
  1. Set up a rudimentary Edge Cache mechanism (this can be mocked or a local Redis cache mimicking an edge layer for testing purposes).
  2. Implement an internal `CacheInvalidatorService` that listens to `PRODUCT_UPDATED` or `INVENTORY_CHANGED` events.
  3. When the Operations Agent (or standard backend logic) updates a product, the `CacheInvalidatorService` must successfully purge the corresponding cache key.
  4. Build an `SeoPreRenderService` that listens to storefront changes, generates static HTML with SEO tags for a product/storefront, and populates the cache.
  5. **Testing**: Write backend unit tests verifying the invalidation logic and Playwright E2E tests simulating a customer request hitting the cache, the owner updating the product, and the next customer request hitting the newly pre-rendered cache.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
