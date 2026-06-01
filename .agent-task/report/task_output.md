issue_title: "[Architecture] Multi-Tenant Edge Caching for Instant Storefronts"
issue_description: |
  ## Problem Statement
  When a non-technical business owner like Maya (the home baker) launches her OneHumanCorp storefront, she expects it to be instantly available and lightning fast for her customers globally. Currently, every time a customer visits her storefront, the request might travel all the way back to the core database to fetch product catalog, pricing, and variant details. This introduces latency, degrades the customer experience, and unnecessarily loads the core infrastructure. OHC needs a robust, multi-tenant edge caching architecture that serves storefront pages, compressed images, and static assets from geographical edge nodes globally in milliseconds, ensuring high availability even under traffic spikes, while strictly maintaining `tenant_id` isolation to prevent data leaks between businesses.

  ## Research Report
  - **Competitive Landscape**:
    - **Shopify**: Utilizes a highly optimized global CDN and caching tier to serve static assets and catalog data rapidly.
    - **Wix/Squarespace**: Also leverage heavy caching, but dynamic content (like real-time inventory for variants) can sometimes bypass caches leading to slower load times.
  - **OHC Core Advantage**: By implementing a unified caching strategy at the edge, coupled with our WebP auto-compression and GraphQL/REST API designs, we can guarantee sub-100ms load times for the critical path of the storefront UI. The challenge is ensuring that when inventory changes (e.g., a cake size sells out), the edge cache is selectively invalidated without complex configuration from the business owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Client(Customer Browser/Phone) -->|Request Storefront| EdgeNode[Edge Cache Node / CDN];

      EdgeNode -->|Cache Hit| Client;
      EdgeNode -->|Cache Miss| API[OHC API Gateway];

      API --> StorefrontService[Storefront Service];
      StorefrontService --> DB[(Tenant Isolated DB)];

      API -->|Write Response| EdgeNode;

      InventoryUpdate(Inventory Sync) -->|Sold Out| API;
      API -->|Cache Invalidation Event| EdgeNode;
  ```

  ### Key Architectural Invariants
  1. **Cache Partitioning**: All cached data MUST be strictly partitioned by `tenant_id` (e.g., cache keys must be prefixed with `ohc:cache:{tenant_id}:...`) to ensure zero cross-tenant data leakage.
  2. **Selective Invalidation**: When core data changes (e.g., price update, inventory depletion), the system must emit targeted invalidation events rather than clearing the entire tenant cache.
  3. **Stale-While-Revalidate**: The edge cache should serve stale content (if within acceptable limits) while asynchronously fetching fresh data to ensure the UI never blocks.

  ### Mobile-First UX Impact
  - **Performance**: Storefronts will load instantly on 375px mobile screens, even on 3G networks.
  - **Visual Stability**: Images (WebP) and CSS (Glassmorphism tokens) will render without layout shift.
  - **No Owner Configuration**: Maya does not need to know what a "CDN" or "Edge Node" is. It works invisibly.

  ## Implementation Prompt
  **Goal**: Design and implement the Multi-Tenant Edge Caching strategy for OHC storefronts.

  **Core User Journey (CUJ)**:
  1. Maya uploads a new custom cake product on her phone.
  2. The Operations agent saves the product and triggers an async invalidation event for her specific storefront cache.
  3. A customer in another city clicks Maya's Instagram link.
  4. The edge node detects the cache is stale (or missing) for that specific product, fetches the new data from the core API, caches it, and serves the page to the customer in <200ms. Subsequent visits by other customers hit the edge cache directly in <50ms.

  **Acceptance Criteria**:
  - Implement the cache key generation strategy ensuring strict `tenant_id` prefixing.
  - Integrate the caching middleware in the API Gateway layer (or configure the CDN provider logic).
  - Implement the cache invalidation event publisher in the core services (e.g., when a Product or Inventory record is updated).
  - Write comprehensive unit tests verifying that tenant A cannot access cached data of tenant B, and that invalidation events correctly purge specific keys.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
