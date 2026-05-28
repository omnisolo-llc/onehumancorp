issue_title: "[infrastructure] Architect Edge-Caching Dynamic Storefronts for Zero-Latency Mobile Experiences"
issue_description: |
  ## Title
  Architect Edge-Caching Dynamic Storefronts for Zero-Latency Mobile Experiences

  ## Problem Statement
  For small business owners like Maya (the baker selling on Instagram) and Priya (the boutique owner), their storefronts must load instantly on their customers' phones, no matter where they are. If a customer clicks an Instagram link and the page takes 3 seconds to load, they bounce, and Maya loses a sale. Currently, rendering dynamic content (like live inventory, custom pricing, or personalized catalog items) requires full round trips to the central database. This creates unacceptable latency on mobile networks. We need a way to serve fully dynamic storefronts directly from edge locations globally, ensuring the shop feels instantly responsive, just like a native app, without forcing Maya to manually click "rebuild" or "clear cache."

  ## Research Report
  - **Shopify:** Utilizes Shopify Oxygen (based on Cloudflare Workers) to render Hydrogen storefronts at the edge. They cache personalized data via Subrequest Caching, allowing the main shell to load instantly while fetching dynamic fragments.
  - **Wix:** Employs a massive global CDN and edge computing to prerender sites and serve them closest to the user. Their rendering engine isolates user-specific data from generic page data.
  - **Vercel / Next.js:** Pioneers in ISR (Incremental Static Regeneration) and edge middleware. They allow dynamic routing and API calls at the edge, mitigating central DB latency.
  - **OHC Gap Analysis:** Our current cloud-native shared service (Rust API Server + Postgres) requires a central hit for dynamic queries. We lack a robust edge-caching layer that can intelligently serve cached catalogs while injecting real-time state (e.g., "Sold Out", live cart balances) using edge compute. This holds back our promise of "maximum performance" for our real user personas.

  ## Design Doc
  ### Key Design Decisions
  - **Edge Rendering & Cache Invalidation Strategy:** Utilize a globally distributed Edge Compute network to serve storefront requests. The edge will hold the compiled UI shell and static catalog data.
  - **Micro-Fragments for Dynamic State:** Inventory availability, personalized pricing, and cart states will be fetched as micro-fragments via lightweight edge subrequests or WebSocket streams, preventing full page re-renders.
  - **AI Cache Intelligence:** The Operations AI Agent will monitor inventory velocity and proactively push invalidation events to the edge when stock levels change, ensuring Maya never over-sells while keeping the cache hit rate extremely high.
  - **Zero-Trust Multi-Tenancy:** Each tenant's cached data is strictly isolated using cryptographic tenant signatures at the edge layer, verified by SPIFFE/SPIRE context.

  ### Architecture Diagram
  ```mermaid
  graph TD;
      MobileBuyer[Customer Mobile Browser] -->|Edge Request| EdgeCDN[Edge Compute Node];
      EdgeCDN -->|Cache Hit| StorefrontShell[Cached Storefront Shell];
      EdgeCDN -->|Dynamic Fragment Request| API[OHC Rust Core API];
      API -->|Read/Write| PrimaryDB[(Postgres Main DB)];
      AI_Ops[Operations AI Agent] -->|Proactive Invalidation| EdgeCDN;
      AI_Ops -->|Monitor Stock Velocity| API;
      StorefrontShell --> MobileBuyer;
  ```

  ### Mobile UX Flow (375px first)
  1. Customer taps link in bio.
  2. The generic storefront shell loads in <100ms from the edge (Header, Catalog Grid, Hero Image).
  3. Skeleton loaders briefly pulse over dynamic areas (Add to Cart buttons, Stock remaining).
  4. Dynamic fragments resolve via edge computing within <300ms, replacing skeletons with actionable buttons or "Sold Out" badges.
  5. The transition is imperceptible; the site feels completely instantaneous.

  ### AI Agent Integration Points
  - **Operations Agent:** Proactively identifies rapid-selling items (e.g., Fatima's food cart lunch rush) and adjusts cache TTLs on the fly, switching to real-time WebSockets if an item is about to sell out to prevent double-booking.

  ## Implementation Prompt
  Implement the edge-caching capability for dynamic storefronts.
  **User-Facing Outcome:** Customers visiting any OHC storefront on a mobile device should see the page load instantly (<100ms TTFB), with live dynamic data (inventory, cart) popping in gracefully without blocking the initial render.
  **CUJ:** Customer clicks an OHC link on Instagram -> Storefront loads instantly -> Customer sees accurate inventory and adds item to cart -> Smooth checkout flow.
  **Acceptance Criteria:**
  - Introduce an edge-routing capability that intercepts storefront requests and serves cached shells.
  - Implement a mechanism to fetch dynamic micro-fragments (e.g., inventory counts) asynchronously or via edge middleware.
  - Create a proactive cache invalidation event trigger that the AI agents can invoke.
  - Must meet performance target: 99th percentile TTFB under 150ms globally.
  - Do not alter the core tenant isolation paradigms; ensure all edge cached data remains strictly isolated.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []