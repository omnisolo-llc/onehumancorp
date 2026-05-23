issue_title: "Zero-Latency Global Edge Storefront Architecture"
issue_description: |
  # Zero-Latency Global Edge Storefront Architecture

  ## Problem Statement
  When Maya the baker shares a link to her cake catalog on Instagram, or when Leo the tutor puts his booking link in his TikTok bio, their customers expect the page to load instantly. If Fatima's food cart menu takes more than 3 seconds to load on a low-end Android device connected to a spotty 3G network, hungry customers will abandon the page. Small business owners lose revenue every time a page stutters or fails to load. They need storefronts that appear instantly across the globe, without ever having to configure servers, CDNs, or caching rules themselves.

  ## Research Report
  Current platforms like Shopify and Wix utilize global content delivery networks (CDNs) and edge computing to serve storefronts close to the user's location.
  - **Shopify**: Uses an edge-caching architecture called Oxygen and heavily leverages Cloudflare to cache HTML, images, and static assets globally. It invalidates cache dynamically when inventory drops.
  - **Wix**: Delivers sites via a global network of PoPs (Points of Presence) and auto-optimizes images and fonts for mobile devices dynamically.
  - **Squarespace**: Utilizes global CDNs but can occasionally struggle with dynamic content caching compared to headless edge setups.
  - **The OHC Opportunity**: OneHumanCorp must leapfrog these legacy systems by utilizing a completely "invisible" edge-caching architecture. Our storefronts should be pre-rendered dynamically at the edge. The system must seamlessly handle inventory changes (e.g., a cake selling out) by triggering localized cache invalidations instantly via background AI agents, ensuring the customer always sees up-to-date data with sub-second latency.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Customer[Customer on Mobile 375px] -->|HTTPS Request| Edge[Global Edge CDN / PoP];
      Edge -->|Cache Hit| Customer;
      Edge -->|Cache Miss| API[OHC Gateway API];
      API --> Storefront[Storefront Renderer Service];
      Storefront --> DB[(Multi-Tenant DB)];
      DB --> Storefront;
      Storefront -->|HTML/JSON| Edge;

      InventoryAgent[AI Inventory Agent] -->|Spots Sold Out Item| MsgBus[Message Bus];
      MsgBus --> Invalidator[Cache Invalidator Service];
      Invalidator -->|Purge Tags| Edge;
  ```

  ### UI Wireframes & Screen Flow Description (375px First)
  1. **Link Click**: Customer taps TikTok link-in-bio (Leo's portfolio).
  2. **Instant Paint (0.2s)**: 375px viewport instantly displays the skeleton layout (Glassmorphism top nav, bottom CTA). The core content (lessons available) renders immediately from edge cache.
  3. **Interactive Readiness (0.5s)**: Customer can scroll the smooth list of available times. Beautiful, optimized images of Leo's past sessions pop in via lazy-loading.
  4. **Dynamic Update (Background)**: A subtle "Live" indicator updates if a slot is booked by someone else, pushed seamlessly over WebSockets.

  ### Mobile UX Flow
  - **Fast First Input Delay (FID)**: Buttons and navigation must feel instantly responsive, using macOS-style Translucent Glass materials and clean UniFi modular cards.
  - **Offline Mode Graceful Degradation**: If the user drops connection during a train ride, the cached product list remains accessible. When attempting to book, a gentle offline toast appears: "Waiting for connection to complete booking..."

  ### AI Agent Integration Points
  - **Operations AI Department**: Monitors inventory levels continuously. If Fatima sells the last Halal Platter, the AI detects the state change and instantly triggers an edge cache purge for her storefront's specific `tenant-id` and `product-id`.
  - **Marketing AI Department**: Adjusts layout elements based on global traffic patterns. If an influx of traffic arrives from an Instagram story, the AI proactively pre-warms the cache for the linked products.

  ### Key Design Decisions and Why
  - **Tag-Based Cache Invalidation**: Instead of purging the entire site, we use surrogate keys (tags) linking products to specific cache nodes. This ensures that only modified items are regenerated, maximizing cache hits.
  - **Edge SSR (Server-Side Rendering)**: Ensures SEO is perfect for products while maintaining instant interactivity.
  - **Invisible Complexity**: Business owners (the Maya, Carlos, Priya, Leo, Fatima personas) have ZERO configuration options for this. No "clear cache" buttons. The AI Departments handle all state synchronization and cache invalidation invisibly.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Build the Zero-Latency Global Edge Storefront Architecture. The primary CUJ (Critical User Journey) is a customer tapping a storefront link and seeing fully rendered, interactive product catalogs within 500ms, regardless of geographic location.

  **Acceptance Criteria:**
  - Storefront requests are routed through a global edge layer.
  - Static assets (images, fonts, CSS) and dynamic HTML are cached at the edge.
  - When a product's inventory reaches zero, the catalog page cache must be automatically invalidated within 2 seconds, displaying "Sold Out".
  - The system must adhere strictly to multi-tenant isolation; a cache key must uniquely incorporate the `tenant_id`.
  - Business owners must not see any cache management settings in their UI. It must be entirely autonomous.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
