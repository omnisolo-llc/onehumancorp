issue_title: "[Architecture] Edge-Caching Dynamic Storefronts"
issue_description: |
  # [Architecture] Globally Distributed Edge-Caching for Dynamic Storefronts

  ## Problem Statement
  When Maya (our baker) posts a TikTok that goes viral, she might suddenly get 10,000 visitors to her online storefront in an hour. Right now, every time a visitor views her cake catalog, the request might travel halfway around the world to a central server, causing slow load times. If the site takes more than 3 seconds to load on a weak cellular connection, the customer will leave, and Maya loses the sale. We need Maya's storefront to load instantly for anyone, anywhere, regardless of how much traffic she suddenly gets, without her needing to configure "servers" or "CDNs."

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Utilizes a globally distributed edge network (Cloudflare-backed) to cache storefront assets and dynamic HTML. They boast millisecond load times globally, which is a massive selling point.
  - **Wix/Squarespace:** Both use aggressive edge caching for static assets, but dynamic content (like real-time inventory for ticket sales or flash sales) can still experience latency under heavy load.
  - **Vercel/Next.js (Industry Standard):** Utilizes Edge Functions and Incremental Static Regeneration (ISR) to serve cached pages at the edge while revalidating data in the background.

  **Market Needs:**
  Small business owners do not understand "cache invalidation" or "edge routing." They just want their site to stay up and be fast when they go viral. OHC needs an architecture that pushes the entire storefront (UI and public data) to edge nodes globally, while seamlessly handling real-time states like "Sold Out" when Fatima's food cart runs out of falafel.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Edge Network [Global Edge Points of Presence]
          CDN[CDN / Edge Proxy]
          EdgeCache[(Edge Cache / Redis Edge)]
          EdgeWorker[Edge Compute Function]
      end

      subgraph OHC Core Cloud [Multi-Tenant Cloud]
          API[Rust API Server]
          Postgres[(Postgres SIPDB)]
          Redis[(Central Redis)]
          Agents[AI Agent Swarm]
      end

      Customer[Customer Mobile Browser] --> CDN
      CDN --> EdgeCache
      CDN -. Miss/Dynamic .-> EdgeWorker
      EdgeWorker --> API
      API --> Postgres
      API --> Redis

      Agents -->|Invalidate/Update On Inventory Change| API
      API -->|Cache Purge Event| EdgeCache

      classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
      class EdgeNetwork,OHCCoreCloud,CDN,EdgeCache,EdgeWorker,API,Postgres,Redis,Agents premium;
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Customer View:** The customer taps Maya's link in bio. The page (375px viewport) loads in under 100ms. The UI is a pristine glassmorphic card layout showing the cake catalog.
  2. **Dynamic Update:** When Maya updates her inventory from her phone (e.g., marks "Vegan Chocolate Cake" as sold out), the change is saved instantly.
  3. **Seamless Revalidation:** The customer refreshing the page 2 seconds later sees a beautiful, subtle transition changing the cake's status to "Sold Out" (with a 200ms ease-out animation).
  4. **Merchant View:** Maya does not see any "Cache Settings" in her app. Her advanced settings are hidden. She only sees "Your store is live and lightning fast."

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory changes (e.g., when an order is placed or a merchant manually changes stock). Automatically fires cache invalidation events to the Edge Network to ensure no overselling occurs.
  - **Marketing Agent:** If a sudden traffic spike is detected at the Edge (viral event), the agent notifies the merchant: "Your store is getting a lot of traffic! Make sure you have enough stock."

  ### Key Design Decisions
  - **Stale-While-Revalidate (SWR):** Use an SWR strategy at the edge to always serve a fast page, fetching real-time inventory in the background to update the UI on the client side without blocking the initial render.
  - **Zero Configuration for Merchants:** Caching must be completely invisible. There are no toggles for "Enable CDN."
  - **Event-Driven Invalidation:** The core API must emit events to the edge when critical data (inventory, prices) changes, rather than relying purely on time-to-live (TTL).

  ## Implementation Prompt
  Implement the Edge-Caching Dynamic Storefront architecture.
  - **User-Facing Outcome:** Customer storefronts load instantly globally. High traffic spikes do not bring down the store. Inventory updates reflect globally within seconds without manual cache purging by the user.
  - **CUJ (Critical User Journey):**
    1. Customer accesses storefront URL.
    2. Edge network serves cached HTML/JSON immediately.
    3. Client-side asynchronously fetches real-time inventory status.
    4. Merchant updates stock -> System invalidates relevant edge cache tags.
  - **Acceptance Criteria:**
    - Storefront requests are served from the edge cache with low latency (target <100ms TTFB).
    - Inventory changes automatically trigger cache invalidation.
    - The solution handles high concurrency gracefully.
    - Zero caching configuration is exposed to the merchant UI (hidden behind advanced settings if absolutely necessary).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
