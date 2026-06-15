issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Inventory Sync"
issue_description: |
  # OHC Architecture Brief: Edge-Cached Dynamic Storefront & Real-Time Inventory Sync

  ## Target Persona: Priya (Boutique Owner)
  Priya needs her online store to be instantly responsive (to capture impulsive buyers) while staying perfectly in sync with her physical store's inventory. If she sells her last item in-store, it needs to instantly show as "Out of Stock" online to prevent double-selling.

  ## Problem Statement
  Current e-commerce platforms like Shopify and Wix often struggle to provide instant edge-cached page loads while maintaining real-time inventory synchronization for hybrid merchants. While platforms like Shopify offer high scalability, micro-SMEs face disjointed operations where online catalogs fall out of sync with in-person sales (tap-to-pay), leading to inventory conflicts and poor customer experiences. We need to introduce an edge-cached dynamic storefront architecture that combines static-like load speeds for SEO/UX with real-time distributed inventory locking.

  ## Research Report & Competitor Analysis
  - **Shopify**: Excellent edge-caching for storefronts, but requires expensive third-party tools or high-tier plans to guarantee sub-second real-time multi-channel inventory locks across POS and web.
  - **Wix & Squarespace**: Drag-and-drop builders generate heavy DOMs. They provide adequate static caching but lack robust, real-time distributed POS sync.
  - **GoDaddy**: Simple but lacks multi-tenant distributed transaction locks, leading to frequent double-bookings for fast-moving inventory.
  - **OHC Opportunity**: Provide an OHC-native architecture using Next.js/Flutter PWA style edge-caching combined with Redis Redlock for distributed inventory reservations. This ensures ultra-fast storefronts with zero risk of double-selling.

  ## Design Doc & Architecture

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant CDN (Edge Cache)
      participant OHC API (Backend)
      participant Redis (Redlock)
      participant DB (Central Ledger)

      Customer->>CDN: Request Storefront Page
      CDN-->>Customer: Serve Cached Page (Fast)
      Customer->>OHC API: Add to Cart (Initiate Checkout)
      OHC API->>Redis: Acquire Lock `ohc:lock:{tenant_id}:inventory:{product_id}`
      Redis-->>OHC API: Lock Acquired
      OHC API->>DB: Reserve Inventory Temporarily
      OHC API-->>Customer: Confirm Reservation
      Customer->>OHC API: Finalize Payment
      OHC API->>DB: Commit Sale & Update Stock
      OHC API->>Redis: Release Lock
      OHC API->>CDN: Invalidate/Update Cache for Product
  ```

  ### UI/UX Flow & Mobile First (375px)
  - **Storefront View**: Clean, translucent glass UI components. Product grids use lazy-loaded WebP images. Optimized for 375px viewports.
  - **Tap-to-Pay / POS View**: Minimalist interface. ≥44x44px touch targets for rapid checkout.
  - **Optimistic UI Updates**: Inventory changes reflect instantly on the UI while network requests resolve in the background.

  ### AI Agent Integration
  - **Operations Agent**: Monitors inventory drops and coordinates restock notifications.
  - **Customer Success Agent**: Automatically triggers if a cart item becomes unavailable, offering alternatives or backorder options.

  ### Key Design Decisions
  - **Redis Redlock**: Chosen for distributed, temporary inventory reservations across multi-channel environments, eliminating double-sells.
  - **Edge Caching**: Implementing aggressive CDN caching for catalog views, with targeted invalidation hooks on inventory changes.

  ## Implementation Prompt
  - Design and build the distributed inventory locking mechanism using Redis Redlock for the `checkout` and `add_to_cart` flows. Ensure strict tenant isolation.
  - Create the API endpoints for real-time POS sync to the central PostgreSQL ledger.
  - Implement the cache invalidation triggers on inventory reduction.
  - Develop the mobile-first (375px) storefront components ensuring responsive layouts and premium translucent design tokens.
  - Implement full Playwright E2E coverage for the scenario of simultaneous online and offline purchase attempts on the same inventory item.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
