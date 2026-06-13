issue_title: "Implement Edge Execution & Caching Strategy for Mobile Storefronts"
issue_description: |
  ## Issue Title
  Implement Edge Execution & Caching Strategy for Mobile Storefronts

  ## Problem Statement
  Currently, mobile users visiting an owner's storefront experience suboptimal First Contentful Paint (FCP) and Time to Interactive (TTI) due to storefront data being fetched dynamically from the centralized core server. For users like Fatima (food cart, operating on a low-end Android device with poor mobile data) or Maya (bakery owner, routing Instagram clicks to her storefront), the delay causes high bounce rates. To ensure a premium, Apple/Ubiquiti-style experience, we must implement an edge-cached architecture. The architecture needs to pre-render dynamic storefronts at the CDN edge while keeping inventory checks, variants, and pricing dynamically verified at checkout.

  ## Research Report
  - **Market Context:** Competitors like Shopify and Wix achieve high performance by heavily caching storefronts at edge nodes. Shopify's Hydrogen platform leverages edge computing for dynamic rendering.
  - **OHC Gap:** OHC currently processes most storefront rendering and catalog fetch requests on the main `ohc-core` container in cloud environments, increasing TTFB and lowering SEO scores.
  - **Findings:** A hybrid approach using Edge Caching for catalog/storefront layout combined with a dynamic sub-request layer for inventory variants ensures 99% cache hits for initial loads while guaranteeing transactional integrity during the checkout process.

  ## Design Doc
  ### Mobile UX Flow (375px view)
  1. The user taps an Instagram link to Maya's bakery.
  2. The initial HTML and catalog items load instantaneously from the edge cache, showing a vibrant, translucent glass UI.
  3. Prices and inventory ("sold out" status) are asynchronously validated via lightweight JSON requests to the core server in the background.
  4. Interactions (e.g. adding to cart) are handled gracefully with instant local state updates and background syncing.

  ### Architecture
  ```mermaid
  sequenceDiagram
      participant User (Mobile)
      participant Edge (CDN / Next.js Edge)
      participant Core API (Rust / Postgres)

      User->>Edge: GET /storefront/{tenant_id}
      alt Cache Hit
          Edge-->>User: Cached HTML (instant)
      else Cache Miss
          Edge->>Core API: Fetch Storefront Data
          Core API-->>Edge: JSON Data
          Edge-->>Edge: Render HTML & Store in Cache
          Edge-->>User: Rendered HTML
      end
      User->>Edge: JS fetches dynamic pricing/inventory
      Edge->>Core API: GET /api/v1/catalog/{item_id}/inventory
      Core API-->>User: Real-time status ("In Stock")
  ```

  ### Core Decisions
  1. Storefront read models will be materialized and serialized, then cached via Redis/Valkey on the core with an appropriate `Cache-Control` header to allow upstream CDN caching.
  2. A Cache Invalidation mechanism MUST be triggered via PostgreSQL triggers or application logic whenever Maya or Fatima updates their inventory, prices, or layout.
  3. Multi-tenancy must be strictly enforced in the cache keys to prevent data leakage (`ohc:cache:{tenant_id}:storefront`).
  4. The implementation must not prescribe a specific external CDN; it should use HTTP Cache-Control headers and local memory/Valkey caching correctly.

  ### AI Agent Integration Points
  - **Operations Agent:** When Fatima toggles a menu item to "Sold Out", the Operations Agent modifies the DB and subsequently invalidates the relevant `ohc:cache:{tenant_id}` entry.
  - **Sales Agent:** When analyzing storefront analytics, it understands that fast loads lead to conversions and can inform the owner about traffic spikes.

  ## Implementation Prompt
  Implement the Edge Caching Strategy for the OHC Storefront API.
  Your task is to:
  1. Introduce a caching layer utilizing Valkey/Redis for the main storefront and catalog read endpoints (`/api/v1/storefront/{tenant_id}` or equivalent).
  2. Define the cache keys explicitly with strict `tenant_id` boundaries.
  3. Implement the cache invalidation logic hooked into the catalog/storefront mutation pathways (e.g. when an item is created, updated, or deleted, or inventory changes).
  4. Set correct HTTP `Cache-Control` headers for edge caching compatibility (e.g. `s-maxage`).
  5. Add E2E Playwright tests that verify the storefront loads correctly and that updates to inventory successfully invalidate the cache and reflect on the frontend within the established threshold.
  6. The visual layout must adhere to the macOS translucent glass and Ubiquiti modular card standards.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
