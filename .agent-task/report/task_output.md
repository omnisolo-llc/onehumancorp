issue_title: "[architecture] Global Edge-Cached Dynamic Storefronts & Inventory Hydration"
issue_description: |
  # [architecture] Global Edge-Cached Dynamic Storefronts & Inventory Hydration

  ## Problem Statement
  For business owners like Priya (Boutique Owner) and Leo (Music Tutor), getting suddenly popular on social media (e.g., a viral TikTok post) is a dream that often turns into a nightmare when their platform crashes or they accidentally oversell limited inventory. Current monolithic architectures cannot handle sudden 100x traffic spikes effectively. When thousands of users click Priya's "link in bio" simultaneously, the storefront must load instantly (sub-100ms) anywhere in the world to convert that impulse interest into sales, but the inventory checks must remain globally consistent to prevent double-selling. They need an architecture that serves their storefronts instantly from the edge while intelligently hydrating dynamic state (like stock levels and personalized AI recommendations) without bringing down the core tenant databases.

  ## Research Report
  **Codebase & Docs Audit:** Current architectures exist for Tap-to-Pay sync (`[feature]_mobile_tap_to_pay_omnichannel_sync.md`) and instant invoicing (`[architecture]_instant_localized_invoicing.md`), but these rely heavily on the central multi-tenant Rust backend and Postgres. There is no documented strategy for decoupling static storefront rendering from dynamic transaction state for high-scale external customer traffic.

  **Competitor Analysis:**
  - **Shopify:** Utilizes a globally distributed edge network (Cloudflare/Fastly) to cache the Liquid template outputs, dynamically pulling cart and inventory state via lightweight XHR/fetch requests. This allows sub-second load times globally.
  - **Wix & Squarespace:** Heavily cache public pages on CDNs, but often suffer from slow "Time to Interactive" (TTI) when custom dynamic elements or deep inventory checks are involved.
  - **Stripe:** Exceptional at edge-routed checkout sessions, but doesn't handle the storefront rendering itself.

  **Gaps Identified:**
  OHC lacks an edge-caching layer that separates the public "Storefront View" (which should be cached globally) from the "Transactional State" (inventory, cart, checkout) which must be strongly consistent. Without this, every page load hits the core API and multi-tenant DB, risking platform stability during viral events.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Global Edge Network (Cloudflare/Vercel)
          Customer[Customer Mobile Browser] --> CDN[Edge CDN Cache];
          CDN --> EdgeWorker[Edge Worker - Routing & Hydration];
      end

      subgraph OHC Core Multi-Tenant Platform
          API[API Gateway - Rate Limited];
          StorefrontSVC[Storefront Rendering Service];
          Ledger[(Global Inventory & DB)];
      end

      subgraph Agent Departments
          MarketingAgent[Marketing AI - SEO & Cache Invalidation];
          OpsAgent[Ops AI - Anomaly Detection];
      end

      EdgeWorker -- "Cache Miss / Purge" --> StorefrontSVC;
      StorefrontSVC --> Ledger;

      EdgeWorker -- "Async Hydrate (Inventory/Cart)" --> API;
      API --> Ledger;

      MarketingAgent -- "Auto-Purge on update" --> CDN;
      OpsAgent -- "Throttle triggers" --> API;
  ```

  ### Mobile UX Flow (375px First)
  1. **Viral Click:** Customer clicks Leo's TikTok link.
  2. **Instant Paint:** The storefront HTML/CSS/images are served in < 50ms from the nearest edge node. The UI displays skeleton loaders for dynamic elements (like "Seats Remaining").
  3. **Hydration:** In the background, the edge worker fetches the current availability from the core API. Within 200ms, the skeletons are replaced with actual data ("2 Spots Left!").
  4. **Purchase:** When the user taps "Book Now," a direct secure tunnel is opened to the core API (bypassing the static cache) to lock the capacity and process the deposit.

  ### AI Agent Integration Points
  - **Marketing Agent:** Whenever Priya changes a product photo or Leo updates his bio, the Marketing Agent automatically issues a targeted cache invalidation request to the Edge Network to ensure the newest version is served without Priya needing to know what a "cache" is.
  - **Operations Agent:** Monitors edge traffic. If it detects a sudden 1000% spike (e.g., a viral video), it proactively scales the backend hydration endpoints and temporarily disables low-priority analytics tracking to preserve core ledger integrity.

  ### Key Design Decisions & Security
  - **Stale-While-Revalidate (SWR):** The edge network will use SWR caching policies for product pages. It serves the cached version immediately while fetching a fresh copy in the background.
  - **Decoupled Hydration:** Storefronts must be built to render fully (visually) without synchronous database calls. Pricing and inventory are "hydrated" client-side or at the edge.
  - **Zero-Trust for Hydration:** All edge requests hydrating data must carry an anonymous session token tied to the specific tenant, preventing bad actors from scraping global inventory levels.

  ## Implementation Prompt
  Implement the "Global Edge-Cached Dynamic Storefronts" architecture.
  - **User-Facing Outcome:** Business storefronts load instantly (sub-100ms) globally, even during massive traffic spikes. Inventory levels accurately reflect the core database without slowing down the initial page render.
  - **CUJ:** Customer opens a viral product link. The page paints instantly. Inventory data loads seamlessly a fraction of a second later. The user adds to cart and checks out successfully.
  - **Acceptance Criteria:**
    - Separate the storefront delivery path into a statically cacheable layer and a dynamic hydration API.
    - Implement Stale-While-Revalidate caching headers for all public storefront routes.
    - Integrate the Marketing AI Agent to automatically clear edge cache upon catalog updates.
    - Validate that under high simulated load (e.g., 10k req/sec), the core database is shielded by the edge cache while checkout remains functional.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_scope: Large
