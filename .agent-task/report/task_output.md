issue_title: "[SEO] Edge-Cached Dynamic Storefronts"
issue_description: |
  # Edge-Cached Dynamic Storefronts for OneHumanCorp

  ## Problem Statement
  Small business owners on OneHumanCorp need incredibly fast, globally available storefronts. However, the current standard architecture serves requests dynamically from the central backend. This leads to high latency for geographically distant users, slow First Contentful Paint (FCP) and poor Time to Interactive (TTI) metrics, severely penalizing the business's SEO performance on Google, and directly harming conversion rates. Real users like "Maya the baker" or "Fatima the food cart operator" cannot afford to lose customers because their storefront takes 4 seconds to load on a 3G mobile connection. We need an architecture that edge-caches dynamic tenant storefronts while still permitting instant invalidation when stock runs out or prices change.

  ## Research Report
  - **Competitor Analysis:**
    - **Shopify:** Utilizes a globally distributed CDN with edge-caching for storefront pages, combined with edge workers (Oxygen) to inject dynamic data (cart, personalization) without hitting origin.
    - **Vercel / Next.js:** Employs Incremental Static Regeneration (ISR) and Edge Middleware to serve cached pages globally while revalidating them in the background.
    - **Wix/Squarespace:** Heavily cache public-facing pages, utilizing edge nodes to serve optimized images and HTML.

  - **OHC Current State:** Our multi-tenant architecture (`OHC_MULTITENANT=true`) currently routes traffic to our central Rust/Go API backend. We lack an explicit mechanism to cache tenant-specific read-only views (product catalogs, landing pages) at the CDN/Edge level.

  - **Proposed Solution:** Implement an Edge-Cached Dynamic Storefront architecture. This involves:
    1.  **Cache-Control Directives:** The backend must emit precise `Cache-Control` headers (e.g., `s-maxage=3600, stale-while-revalidate=86400`) for all public, unauthenticated storefront read operations (e.g., GET `/api/v1/storefront/{tenant_id}/catalog`).
    2.  **Surrogate Keys (Cache Tags):** Responses must include a `Surrogate-Key` header tagging the content with the tenant ID and entity type (e.g., `Surrogate-Key: tenant-123-catalog`).
    3.  **Proactive Invalidation:** When the `Operations` or `Finance` AI agent modifies inventory, pricing, or the site design, it triggers an event. An Invalidation Worker listens for these events and issues targeted `PURGE` requests to the CDN using the relevant Surrogate-Key.
    4.  **Edge Injection (Future Phase):** Dynamic, user-specific data (like the shopping cart) will be loaded asynchronously via client-side Javascript, keeping the base HTML cacheable.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant C as Customer (Mobile/Web)
      participant Edge as Edge CDN (Cloudflare/Fastly)
      participant API as OHC API Backend
      participant DB as Postgres (Tenant DB)
      participant W as Invalidation Worker
      participant A as AI Agent (Operations)

      %% Cache Miss Scenario
      C->>Edge: GET /api/storefront/tenant_A/catalog
      Edge->>API: (Miss) Request Catalog
      API->>DB: Fetch Data
      API-->>Edge: Returns JSON (Headers: s-maxage=3600, Surrogate-Key: t_A_cat)
      Edge-->>C: Returns Catalog (Caches at Edge)

      %% Cache Hit Scenario
      C->>Edge: GET /api/storefront/tenant_A/catalog
      Edge-->>C: (Hit) Returns Cached Catalog instantly

      %% Update & Invalidation Scenario
      A->>API: Update Inventory (Item Sold Out)
      API->>DB: Persist Change
      API->>W: Enqueue Event (InventoryUpdated, t_A)
      W->>Edge: PURGE /api/storefront/tenant_A/* (via API/Surrogate-Key)
      Edge-->>W: Acknowledge Purge
  ```

  ### UI Wireframes & Mobile Flow (375px)
  -   **User View:** The storefront loads instantaneously. The layout uses the glassmorphic design system (`backdrop-filter: blur(20px)`). The product list is rendered from the edge-cached JSON payload.
  -   **Loading States:** Minimal to non-existent due to edge caching. If client-side fetching is needed (e.g., for personalized cart state), a skeleton loader using the UniFi card layout is displayed briefly.
  -   **Owner View:** When Maya updates a cake price in the OHC app, she sees a "Storefront updating..." toast. Within milliseconds, the invalidation worker purges the cache, and the toast turns green: "Storefront updated globally."

  ### AI Agent Integration
  -   **Operations Agent ("The Manager"):** Emits inventory level changes that trigger cache invalidation.
  -   **Marketing Agent ("The Promoter"):** When generating a new site design or promotional banner, emits a design update event, triggering cache invalidation for the entire tenant's public endpoints.

  ## Implementation Prompt
  **Task:** Implement the backend Cache-Control and Surrogate-Key tagging for public storefront endpoints, and build the Invalidation Worker to process updates.

  **Acceptance Criteria:**
  1.  **Public Endpoints:** Identify at least one public, read-only endpoint (e.g., a mock `GET /api/public/tenant/{id}/products`).
  2.  **Headers:** Ensure this endpoint returns a `Cache-Control` header suitable for edge caching (e.g., `s-maxage=...`) and a `Surrogate-Key` header identifying the tenant.
  3.  **Invalidation Logic:** Implement a background worker or an event hook that listens for updates to a tenant's data (e.g., `PUT /api/tenant/{id}/products/{pid}`) and logs/triggers a purge request for that tenant's surrogate key.
  4.  **Tests:** Write unit tests verifying that the headers are correctly applied to the public endpoints and that the invalidation hook is called upon updates.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
