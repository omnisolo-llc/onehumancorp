issue_title: "[Architecture] Edge-Cached Dynamic Storefronts for Instant Discovery"
issue_description: |
  # [Architecture] Edge-Cached Dynamic Storefronts for Instant Discovery

  ## Problem Statement
  For online-first personas like **Leo (The Music Tutor)** and **Maya (The Baker)**, loading speed and SEO are critical. Currently, OHC storefront pages render dynamically on every request. This causes high latency, especially on slow mobile networks (like Fatima's 3G connection), and hurts SEO rankings. If a customer clicks a link in Leo's TikTok bio and the page takes 4 seconds to load, they will bounce. OHC needs a storefront architecture that is instantly fast globally, without the complexity of managing a static site generator for the non-technical owner.

  ## Research Report
  ### Competitor Analysis
  - **Shopify:** Utilizes a globally distributed edge cache for storefronts. Pages are rendered server-side but aggressively cached at the edge, invalidating instantly on inventory or price changes.
  - **Vercel/Webflow:** Heavy reliance on SSR (Server-Side Rendering) with CDN caching.
  - **Wix:** Employs a complex hydration model that can be slow on low-end devices.

  ### Opportunity for OHC
  By implementing an edge-caching layer (using CDN/Cloudflare/Fastly) combined with intelligent cache invalidation tags, OHC can serve pre-rendered storefronts from the edge in under 50ms globally. The AI Operations Agent can manage cache invalidation seamlessly whenever Maya updates a cake price, keeping the system "zero-config" for the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      CustomerBrowser[Customer Browser / Phone] -->|Requests Storefront| EdgeCDN[Edge CDN Cache];
      EdgeCDN -->|Cache Hit| CustomerBrowser;
      EdgeCDN -->|Cache Miss| LoadBalancer[OHC Load Balancer];
      LoadBalancer --> StorefrontService[Storefront Render Service];
      StorefrontService --> DB[(PostgreSQL Replica)];
      StorefrontService -->|Returns HTML + Cache Headers| EdgeCDN;

      Owner[Maya (Owner)] -->|Updates Price| CoreAPI[OHC Core API];
      CoreAPI -->|Writes to Primary DB| PrimaryDB[(PostgreSQL Primary)];
      CoreAPI --> OperationsAgent[AI Operations Agent];
      OperationsAgent -->|Issues Cache Invalidation| EdgeCDN;
  ```

  ### Mobile UX Flow
  This is an infrastructure capability, but it drastically impacts UX:
  1. Customer taps Maya's link on Instagram.
  2. The page begins rendering content in under 100ms (TTFB).
  3. The catalog images are lazy-loaded via an edge image optimizer.
  4. The "Add to Cart" interaction relies on a localized cart state (localStorage), synchronizing with the backend only on checkout, avoiding network round trips during browsing.

  ### AI Agent Integration Points
  - **AI Operations Agent:** When the owner updates catalog items, policies, or layout, the Operations Agent automatically calculates the necessary cache-tags and issues targeted invalidation requests to the CDN.
  - **AI Marketing Agent:** Analyzes edge CDN analytics (bounce rates, popular pages) to suggest layout optimizations to the owner.

  ### Key Design Decisions
  - **Cache-Control Headers:** Implement `Surrogate-Key` or `Cache-Tag` headers per tenant and per product category for precise invalidation.
  - **Stale-While-Revalidate:** Use SWR caching strategies so customers always see a fast page, even during background catalog updates.
  - **Decoupled Cart:** The shopping cart and user session must be strictly decoupled from the cached HTML (using client-side hydration or edge workers for personalization) to ensure high cache hit rates for the main content.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Design the Edge-Cached Storefront architecture.
  1. Implement a caching middleware in the Rust backend that injects `Cache-Control`, `ETag`, and `Surrogate-Key` headers for all storefront routes (`/site/:tenant_id/*`).
  2. Create a `CacheInvalidationService` that the Operations Agent can call to purge specific tenant or product keys when data changes.
  3. Update the Storefront rendering logic to ensure no user-specific data (like active cart items) is baked into the HTML, relying on client-side fetching for dynamic state.
  4. Add integration tests verifying correct headers are generated and invalidation signals are properly formatted.

  **Priority:** P2
  **Estimated Scope:** Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
