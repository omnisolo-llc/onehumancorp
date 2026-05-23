issue_title: "Architecture: Edge-Caching Dynamic Storefronts for OHC"
issue_description: |
  # Issue Brief: Edge-Caching Dynamic Storefronts

  ## Problem Statement
  Maya (our 28-year-old baker) creates custom cakes and receives bursts of traffic whenever she posts a new reel on Instagram. When 500 people tap the link in her bio at the same time, her storefront takes 5 seconds to load, causing 40% of her potential customers to abandon before seeing her catalog. Fatima (food cart, 50) faces a similar issue: her customers pull up her menu on low-end Androids during the lunch rush in bad cellular areas, and the slow loading makes it hard to quickly order. Currently, OHC storefronts fetch dynamic catalog data from central servers for every visit, creating unacceptable latency and a poor experience that costs our users sales.

  ## Research Report
  ### Competitive Analysis
  - **Shopify**: Uses heavily edge-cached storefronts via Fastly/Cloudflare Workers, rendering pages instantly worldwide. Catalog updates trigger cache invalidations, providing a near-static feel with dynamic backend capabilities.
  - **Wix**: Initially suffered from client-side rendering bloat but moved to server-side rendering (SSR) on edge networks, drastically improving their Core Web Vitals.
  - **Squarespace**: Relies heavily on static generation, which is fast but struggles with real-time inventory updates (like Fatima's sold-out toggles).

  ### Findings
  Our current architecture cannot support instantaneous sub-second loads for mobile storefronts globally because each page load requires database queries in our central cluster. We need a hybrid edge-caching architecture where storefronts are generated and served from the edge (e.g., CDN edges), but dynamic components (like inventory availability and custom quote deposits) communicate asynchronously.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Mobile[Mobile Browser - 375px] -->|Request Storefront| CDN[Edge CDN / Global Cache]
      CDN -->|Cache Miss / Hydration| API[Edge Gateway API]
      API -->|Reads Store Config| Redis[(Edge Cache / Redis)]
      API -->|Mutation / Checkout| Core[OHC Core Cluster]
      Core --> DB[(Primary Datastore)]
      Core --> Agents[AI Agent Swarm]
      Agents -.->|Push Catalog Updates| CDN
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)
  - **Screen 1: Storefront Home**: Loads instantly (LCP < 800ms). Features a full-width header image. Clean, macOS-style Translucent Glass cards for categories (e.g., "Custom Cakes", "Daily Slices").
  - **Screen 2: Product Detail**: Large swipeable image carousel. Big, sticky "Add to Cart / Request Quote" button locked to the bottom of the screen.
  - **Screen 3: AI Assistant Overlay**: A subtle floating bubble at the bottom right. Tapping expands a half-sheet (bottom sheet) where the AI agent helps answer questions like "Do you do vegan cakes?" in real-time.

  ### Mobile UX Flow
  1. User taps Maya's Instagram bio link.
  2. The catalog instantly renders from the nearest Edge node.
  3. Interactive elements (like cart state and AI chat) hydrate asynchronously in the background.
  4. The experience is seamless, maintaining 60fps scrolling and passing the "grandmother test."

  ### AI Agent Integration Points
  - **Marketing Agent**: Automatically analyzes high-traffic periods and adjusts the edge cache TTL (Time to Live) dynamically.
  - **CS/Sales Agent**: Embeds directly into the edge-delivered static page via a lightweight WebSocket connection to handle customer inquiries without a full page reload.
  - **Operations Agent**: Listens to inventory changes (e.g., Fatima marks "Chicken Over Rice" sold out) and immediately triggers a targeted edge cache invalidation so the next user sees the updated menu.

  ### Key Design Decisions
  - **Edge First**: Serve the storefront purely from edge caches. This ensures maximum performance on low-end devices and poor network conditions.
  - **Progressive Hydration**: Critical dynamic data (like "sold out" banners) hydrate on the client immediately after the static frame loads.
  - **Multi-Tenant Cache Isolation**: Ensure cache keys strictly partition Maya's data from Fatima's to prevent cross-tenant data leaks.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the Edge-Caching Dynamic Storefronts architecture. Ensure that storefront requests are routed to an edge-caching layer that serves the initial payload in under 800ms.

  **Core User Journeys (CUJ):**
  - A buyer clicks a storefront link and instantly sees the product catalog, regardless of geographic location.
  - A business owner updates an item's availability, and the storefront reflects this change globally within 2 seconds.

  **Acceptance Criteria:**
  - Storefronts are served from an edge cache.
  - Dynamic elements hydrate asynchronously without blocking the main render.
  - Inventory or catalog updates trigger immediate, targeted cache invalidations.
  - Multi-tenancy isolation is strictly enforced at the caching layer.
  - The implementation must adhere strictly to mobile-first rendering (375px viewports) and pass our Zero-Trust security guidelines.

  Please proceed with detailing and executing the implementation without hardcoding specific underlying libraries or SQL schemas.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
