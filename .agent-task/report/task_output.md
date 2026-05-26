issue_title: "[Architecture] Edge-Cached Dynamic Storefront Engine"
issue_description: |
  # [Architecture] Edge-Cached Dynamic Storefront Engine

  ## Problem Statement
  When Maya (the baker) goes viral on Instagram or TikTok, her custom cake storefront can receive a sudden, massive spike in traffic. If her OHC storefront is slow to load or crashes under the load, she loses critical sales and momentum. Global customers expect sub-100ms load times, but small business owners like Maya have zero knowledge of CDNs, edge caching, or server provisioning. They need a storefront that is instantly fast globally and updates inventory dynamically without any manual configuration.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify:** Utilizes a globally distributed edge network (Fastly/Cloudflare) to cache storefront pages, achieving high availability. However, caching dynamic inventory (e.g., checking if a limited-edition cake is sold out) can sometimes lead to race conditions or delayed updates.
  - **Vercel / Next.js:** Employs Edge Functions to serve dynamic, personalized content while maintaining static-like load times. This is the gold standard for developer-built e-commerce.
  - **Wix:** Has improved its infrastructure significantly but often suffers from bloated client-side JavaScript, slowing down mobile initial load times.

  **Gaps Identified:**
  OHC lacks a unified Edge-Cached Storefront architecture. Currently, requests might hit a central API gateway, which is a bottleneck during viral traffic spikes. We need an edge-native delivery model that pushes storefront static assets and lightweight dynamic checks (inventory, pricing) to the CDN edge, completely shielding the core OHC PostgreSQL cluster from traffic spikes.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Global Edge Network
          CDN[Edge CDN Cache] --> EdgeWorker[Edge Functions / Workers];
      end

      subgraph Mobile Shopper Device
          Browser[375px Mobile Viewport] --> CDN;
      end

      subgraph OHC Core Cloud
          Gateway[API Gateway] --> MainDB[(Cloud Postgres)];
          Gateway --> Agents[AI Agent Swarm];
      end

      EdgeWorker -- "Cache Miss / Checkout" --> Gateway;
      EdgeWorker -. "Async Inventory Check" .-> Gateway;
      Agents -- "Invalidate Cache" --> CDN;

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Monitor Inventory];
          Agents --> MarketingAgent[Marketing: Viral Traffic Alert];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Instant Page Load:** A shopper clicks Maya's Instagram link. The 375px mobile storefront loads in < 100ms from the nearest edge node. The UI features OHC's signature macOS-style Translucent Glass materials.
  2. **Dynamic Inventory Validation:** As the shopper views a cake, an edge function verifies stock availability in the background without blocking the UI rendering. If sold out, the "Add to Cart" button seamlessly transitions to a "Join Waitlist" state.
  3. **Viral Alert:** Maya receives a rich push notification: "Traffic Spike Detected! 500 visitors currently viewing Vegan Cakes."

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors global inventory. When an item is purchased, it updates the central database and instantly issues a targeted cache invalidation command to the Edge CDN to prevent overselling.
  - **Marketing Agent:** Detects traffic spikes and automatically scales edge worker quotas. It also drafts an engagement email to capture new visitors ("Welcome to Maya's Bakery!").

  ### Key Design Decisions
  - **Edge-First Delivery:** All read-heavy storefront traffic is served exclusively from the edge. The core OHC infrastructure is only hit for state-mutating actions (checkout, inventory updates).
  - **Stale-While-Revalidate:** We use a stale-while-revalidate caching strategy to ensure the user never sees a loading spinner for catalog browsing, prioritizing perceived performance.
  - **Zero Trust Multi-Tenancy:** Edge functions use SPIFFE/SPIRE SVIDs to securely authenticate back to the OHC Core when making dynamic inventory queries, ensuring Maya's data is isolated from Priya's data.

  ## Implementation Prompt
  Implement the Edge-Cached Dynamic Storefront Engine for OHC.
  - **User-Facing Outcome:** Storefronts load globally in under 100ms on mobile devices, even under viral traffic spikes, without the business owner configuring anything.
  - **CUJ (Critical User Journey):**
    1. Shopper visits a storefront via social media link.
    2. Page loads instantly from the Edge CDN.
    3. Edge function silently verifies inventory status.
    4. Shopper adds an item to cart and proceeds to secure checkout.
  - **Acceptance Criteria:**
    - Storefronts must achieve a >90 mobile Lighthouse performance score.
    - Read requests for product catalogs must be served from the edge with a cache hit ratio > 95%.
    - Cache invalidation upon inventory change must propagate globally within 2 seconds.
    - Must pass mobile-first UI checks on a 375px viewport with premium glassmorphism styling.
    - Zero configuration required by the OHC user.

  ## Estimated Scope
  Large

issue_priority: P0
issue_scope: Large
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
