issue_title: "[Platform] Implement Global Edge-Cached Dynamic Storefront Architecture"
issue_description: |
  ## Problem Statement
  Small business owners relying on OHC (like Maya the baker who goes viral on Instagram) face significant business risk from traffic spikes. Unoptimized, centralized database queries during sudden traffic surges lead to slow load times, timeouts, and lost revenue. Furthermore, client-side rendered dynamic storefronts suffer from poor search engine indexing, reducing organic discoverability. SMB owners lack the technical expertise to configure CDNs, Server-Side Rendering (SSR), or complex caching layers.

  ## Research Report
  Our competitive analysis (see `[research]_universal_edge_cached_dynamic_storefront_seo.md`) indicates that while competitors like Shopify offer edge caching via Cloudflare, and developer ecosystems (Vercel/Next.js) offer ISR/Edge compute, these are either not fully automated or inaccessible to non-technical users. OHC must provide enterprise-grade performance and discoverability invisibly.

  The solution requires an architecture that automatically caches storefront reads at a global edge layer and utilizes AI agents to autonomously invalidate cache upon inventory changes and pre-render SEO-optimized static HTML upon content updates.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ STOREFRONT : configures
      STOREFRONT ||--o{ PRODUCT : contains
      PRODUCT ||--o{ INVENTORY : has

      CACHE_LAYER {
          string tenant_id
          string resource_type
          string resource_id
          string pre_rendered_html
      }

      AGENT_ENGINE {
          string type
          string status
      }

      TENANT ||--o{ CACHE_LAYER : isolates
      AGENT_ENGINE }o--|| CACHE_LAYER : invalidates
  ```

  ```mermaid
  sequenceDiagram
      participant Owner as Maya (Owner)
      participant App as OHC App (375px)
      participant API as API Server
      participant OpsAgent as Operations Agent
      participant Cache as Edge Cache (Redis/CDN)
      participant Customer as Customer (Browser)

      Owner->>App: Updates Cake Price
      App->>API: PATCH /api/products/123 (tenant_id)
      API->>OpsAgent: Trigger Inventory/Product Update Event
      OpsAgent->>Cache: PURGE ohc:cache:storefront:tenant_id:product:123
      API-->>App: Success

      Customer->>Cache: GET /storefront/maya/cakes
      Cache-->>Customer: Returns new price immediately (Cache Miss/Re-fetch or Pre-rendered)
  ```

  ### Components
  1.  **Edge Cache Layer:** A distributed cache (e.g., Redis or a dedicated caching proxy like Varnish/Cloudflare interface) that sits in front of the main API. It intercepts storefront read requests (`GET /api/storefront/:tenant_id/*`).
  2.  **Agentic Cache Invalidation Engine:** A module within the OHC backend that listens to internal state change events (e.g., inventory decrease, price change, new product). When an event occurs, it automatically issues granular cache invalidation commands to the Edge Cache Layer using specific cache keys (e.g., `tenant_id:product_list`, `tenant_id:product_id`).
  3.  **Agentic SEO Pre-renderer:** When the Marketing Agent or user updates storefront content (e.g., adds a product, changes theme), an asynchronous job is triggered. This job pre-renders the dynamic React/Flutter components into static HTML, injects relevant meta tags and structured data (JSON-LD), and pushes this pre-rendered payload to the Edge Cache Layer, ready to be served to search engine crawlers and users.

  **Multi-Tenant Isolation:**
  Cache keys MUST strictly include the `tenant_id` to prevent cross-tenant data leakage (e.g., `ohc:cache:storefront:{tenant_id}:{resource_type}:{id}`).

  ### Mobile UX Flow & Wireframes (375px Viewport)
  For the owner (e.g., Maya), this feature is entirely invisible.
  1.  **Screen 1 (Product Edit):** Maya updates a cake price in the OHC app (375px viewport). The screen features a clean, translucent glass card for the price input.
  2.  **Interaction:** She taps "Save" (a prominent 44x44px target).
  3.  **Screen 2 (Loading State):** She sees a brief, non-blocking toast notification "Saving changes..." while the change saves.
  4.  **Background Process:** In the background, the Agentic Cache Invalidation Engine purges the relevant cache keys, and the SEO Pre-renderer updates the static HTML.
  5.  **Screen 3 (Customer View):** When Maya's customers view the storefront on their phones, they instantly see the updated price, served blazingly fast from the edge cache, regardless of traffic volume.

  ### AI Agent Integration
  -   **Operations Agent:** Monitors inventory and triggers cache invalidation when stock levels change.
  -   **Marketing Agent:** Triggers SEO pre-rendering when storefront content is modified, ensuring crawlers always see the latest optimized content.

  ## Implementation Prompt
  Implement the Universal Edge-Cached Dynamic Storefront infrastructure.

  **Requirements:**
  1.  Implement a robust caching middleware/service for storefront API read requests (`GET`), utilizing Redis (or similar) with strict `tenant_id` based keying.
  2.  Implement the Cache Invalidation mechanism that listens to inventory/product change events and precisely purges the affected cache keys.
  3.  Implement a background job for SEO pre-rendering that generates static HTML with injected meta tags upon content updates and stores it in the edge cache.
  4.  Ensure all cache interactions handle failures gracefully (fallback to database read) and maintain strict multi-tenant isolation.
  5.  Write comprehensive unit tests ensuring cache hit/miss logic and invalidation accuracy.
  6.  Write a Playwright E2E test verifying that a product update instantly reflects on the storefront API (via cache invalidation) and that pre-rendered HTML is served.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
