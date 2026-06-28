issue_title: "[Architecture] Distributed Edge Caching and Dynamic Storefront SEO Engine"
issue_description: |
  # Distributed Edge Caching and Dynamic Storefront SEO Engine

  ## Problem Statement
  SMB owners (like Maya the baker and Priya the boutique operator) need their online storefronts to be blazing fast to capture sales and rank high on search engines. However, dynamic capabilities (like personalized offers, real-time inventory checks, and complex product variants) often slow down page loads. A traditional monolithic database query for every page view is not scalable or performant enough for modern SEO requirements or high-traffic events (e.g., a viral TikTok post). Owners don't have the technical expertise to set up CDNs, edge functions, or complex cache invalidation rules. We need a zero-configuration, autonomous edge caching system that perfectly balances dynamic storefront needs with sub-100ms response times globally.

  ## Research Report
  - **Competitor Analysis:** Shopify uses a complex Edge caching layer (Oxygen/Hydrogen) combined with stale-while-revalidate patterns. Wix uses geographically distributed CDNs but often struggles with complex dynamic data rendering.
  - **Market Gap:** Current solutions require owners to understand "cache clearing" or deal with out-of-sync inventory on storefronts. OHC needs an invisible, automatic cache invalidation mechanism tied directly to the `Ledger` and `Inventory` mutations, ensuring that a product is never shown as "available" when it's sold out, while keeping 99% of page loads served from edge cache.
  - **Technical Requirement:** Implementation of a stale-while-revalidate (SWR) cache layer, likely leveraging Cloudflare Workers or similar edge compute, coordinated by our AI Operations department for intelligent pre-fetching based on predictive traffic models.

  ## Design Doc
  ### Mobile UX Flow
  1. Customer taps a link from Instagram (Maya's cake shop).
  2. The storefront loads instantly (<100ms) from the edge cache.
  3. In the background, an async request checks for real-time inventory or personalized offers.
  4. If the item sold out moments ago, the UI updates gracefully, offering an alternative or a waitlist signup (managed by the Sales/Operations agent).
  5. The owner (Maya) sees real-time traffic spikes in her OHC app without needing to "scale up servers".

  ### Architecture
  - **Edge Cache Layer:** Storefront HTML, CSS, and static assets are cached at the edge.
  - **Dynamic Invalidation Engine:** When an inventory mutation occurs (e.g., via the POS or an online sale), a lightweight webhook or Kafka event triggers targeted cache invalidation for the affected product/storefront routes.
  - **AI Coordination:** The Operations Agent monitors traffic patterns. If a spike is detected, it pre-warms the cache for related products.
  - **Zero-Trust & Tenancy:** Cache keys MUST incorporate `tenant_id` to prevent data leakage between stores.

  ### AI Agent Integration
  - **Operations Agent:** Monitors cache hit rates and traffic spikes, adjusting edge caching rules dynamically.
  - **Marketing Agent:** Optimizes SEO metadata during background processing and pushes updates to the edge cache.

  ## Implementation Prompt
  Implement an Edge Caching Coordinator service. This service should listen to inventory and product mutation events and issue targeted invalidation requests to our CDN/Edge layer. It must be completely transparent to the user. Ensure strict multi-tenant isolation by namespacing all cache keys with the `tenant_id`. Include E2E tests simulating a high-traffic event and verifying that inventory changes invalidate the cache correctly without exposing stale data during checkout.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
