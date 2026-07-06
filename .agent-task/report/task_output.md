issue_title: "[Architecture] Edge-Caching Dynamic Storefronts Implementation"
issue_description: |
  # Research Report: Edge-Caching Dynamic Storefronts Implementation

  ## 1. Problem Statement
  Small business owners like Maya (Baker) and Leo (Musician) can experience massive spikes in traffic when a TikTok or Instagram reel goes viral. Currently, every page load hits the centralized multi-tenant database (PostgreSQL), risking latency degradation, dropped requests, and potential outages for their storefronts. This creates friction and a poor customer experience during critical revenue-generating moments. Small business owners cannot afford to manage their own caching infrastructure.

  ## 2. Research Report
  - **Competitor Analysis:**
    - **Shopify:** Utilizes a globally distributed edge network (Cloudflare) to cache storefront assets and read-only API requests, serving dynamic content close to the buyer.
    - **Vercel / Next.js:** Employs ISR (Incremental Static Regeneration) and Edge caching for instant load times without sacrificing dynamic product availability.
  - **OHC Requirement:** The caching must be completely invisible to the user. A sold-out item must instantly invalidate the cache across the edge network so that Fatima (Food Cart) doesn't over-sell her pre-orders.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Buyer Mobile App / Web] -->|Requests Storefront| B(Cloudflare/Fastly Edge CDN);
      B -- Cache Hit --> A;
      B -- Cache Miss --> C(OHC API Gateway);
      C --> D[Storefront Service];
      D --> E[(PostgreSQL Read Replica)];

      F[Operations Agent] -->|Updates Inventory| G[Inventory Service];
      G -->|Sold Out Event| H[Cache Invalidation Bus];
      H -->|Purge Key| B;
  ```

  ### Mobile UX Flow (375px First)
  - The storefront loads instantly (<100ms) on a 375px viewport even on slow 3G connections (critical for Fatima).
  - High-quality WebP product images are served directly from the Edge.

  ### AI Agent Integration Points
  - **Operations Agent:** Any inventory mutation or website redesign from the Operations/Marketing Agents triggers an async event to purge the corresponding surrogate keys globally.

  ### Key Design Decisions & Security
  - **Edge Cache Invalidation:** Storefront queries will use surrogate keys (e.g., `storefront:{tenant_id}`, `product:{product_id}`). Any inventory mutation or website redesign from the Operations/Marketing Agents triggers an async event to purge the corresponding surrogate keys globally.

  ## 4. Implementation Prompt
  Implement the Edge-Caching Dynamic Storefronts system.
  - **User-Facing Outcome:** The storefront loads instantly (<100ms) on a 375px viewport even on slow 3G connections. A sold-out item must instantly invalidate the cache across the edge network so that the business owner doesn't over-sell her pre-orders.
  - **CUJ:** When Maya's customer opens her storefront link, the product list should be served from the cache. When Maya updates a cake price, the system must publish a cache invalidation event, ensuring the next customer sees the updated price within 1 second.
  - **Acceptance Criteria:** E2E test verifying cache hits for repeated reads and cache misses immediately following an inventory update. Strict tenant isolation must be maintained in the cache layer. Integrate a reverse-proxy CDN layer (e.g., configuring Cloudflare or a Redis-based API cache simulator for local development) that caches GET requests for public storefront APIs.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
