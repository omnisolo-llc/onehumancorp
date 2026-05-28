issue_title: "[architecture] Edge-Caching Dynamic Storefronts for Instant Link-in-Bio Conversions"
issue_description: |
  # [Edge-Caching Dynamic Storefronts] Instant Link-in-Bio Conversions for Solopreneurs

  ## Problem Statement
  For modern solopreneurs like Leo (music tutor with a TikTok following) or Maya (custom baker operating through Instagram DMs), the "link in bio" is their primary acquisition funnel. When followers click this link from a social media app, every millisecond of latency translates to lost revenue. If Leo posts a viral video, the resulting traffic spike can overwhelm standard dynamic rendering. These users need an invisible, zero-config infrastructure that delivers their storefronts instantly (sub-100ms LCP) anywhere in the world, while gracefully handling dynamic state like real-time inventory counts, sold-out toggles, and booked calendar slots.

  ## Research Report
  **Competitive Analysis:**
  - **Linktree / Beacons:** Offer near-instant load times but severely lack deep e-commerce, custom cart, and calendar booking integration.
  - **Shopify:** Provides excellent edge delivery, but building highly customized, extremely fast link-in-bio specific pages often requires advanced headless development (Hydrogen/Oxygen).
  - **Wix / Squarespace:** Traditional rendering can be sluggish on mobile networks within social media in-app browsers.

  **Market Needs:**
  Social commerce demands zero friction. A 3-second load time in an Instagram webview means a 50% bounce rate. OHC must orchestrate globally distributed edge caching (e.g., Cloudflare Workers / Vercel Edge) to serve static-like HTML globally, while hydrating dynamic capabilities (e.g., Cart, Checkout, Calendar availability) via edge-adjacent APIs, abstracting the complexity entirely away from the merchant.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Global Edge Network
          CDN[Edge CDN / Cache] --> EdgeWorker[Edge Worker function];
          EdgeWorker --> CachedStorefront[Cached HTML/CSS/JS];
      end

      subgraph Core Platform
          Gateway[OHC API Gateway];
          MainDB[(Cloud Postgres)];
          MainDB --> Gateway;
          Gateway -- Cache Invalidation --> CDN;
      end

      subgraph Mobile Client (375px)
          SocialWebView[TikTok / IG In-App Browser] --> EdgeWorker;
          SocialWebView -- Dynamic Hydration --> Gateway;
      end

      subgraph Agent Departments
          Agents[AI Agent Swarm];
          Agents --> OpsAgent[Ops: Monitor Inventory & Invalidate Cache];
          Agents --> MarketingAgent[Marketing: Analytics Aggregation];
          OpsAgent --> Gateway;
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Acquisition:** A user clicks Leo's link-in-bio on TikTok.
  2. **Instant Render:** The Edge CDN instantly serves the cached HTML shell, rendering a beautiful glassmorphic profile and service list within 50ms.
  3. **Hydration:** A lightweight client script fetches the latest calendar availability directly from the OHC API Gateway.
  4. **Conversion:** The user taps a 30-minute lesson slot and pays via Apple/Google Pay. The transaction succeeds in under 3 seconds.
  5. **Auto-Invalidation:** The successful booking triggers the Ops Agent, which immediately invalidates the edge cache for Leo's specific service page so the slot disappears for the next visitor.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory/calendar changes. When Maya's vegan cake goes out of stock or Leo's Tuesday slot is booked, the agent triggers a highly targeted cache invalidation payload to the Edge CDN.
  - **Marketing Agent:** Processes edge analytics asynchronously to inform the merchant which social media platform is driving the highest conversions and optimal posting times.

  ### Key Design Decisions
  - **Stale-While-Revalidate (SWR):** Pages use aggressive SWR caching to ensure a cache hit on the edge, while fetching fresh dynamic state in the background.
  - **Edge-First Abstraction:** All OHC storefronts default to this architecture. There is no "enable edge caching" toggle for the user.
  - **Granular Invalidation:** Cache invalidation must be tied to specific tenant IDs and entity tags (ETags) to avoid full platform cache purges.

  ## Implementation Prompt
  Implement the Edge-Caching Dynamic Storefront delivery system and corresponding cache invalidation pipeline.
  - **User-Facing Outcome:** Visitors clicking a merchant's OHC link load the storefront almost instantaneously, regardless of geographic location or traffic spikes. Merchants see real-time updates (like sold-out items) reflected on their live site within seconds, without configuring any CDN settings.
  - **CUJ (Critical User Journey):**
    1. Merchant updates an item's price or stock in the OHC mobile app.
    2. OHC backend processes the change and instructs the Ops Agent to invalidate the relevant cache keys.
    3. The next visitor to the storefront receives the fresh, newly cached page from the nearest edge node.
  - **Acceptance Criteria:**
    - Storefronts must achieve sub-100ms TTFB (Time to First Byte) on global edge nodes.
    - Updates to inventory/pricing must be reflected globally within 5 seconds.
    - The implementation must support high concurrency during traffic spikes without degrading primary database performance.
    - No CDN/edge terminology is exposed to the merchant.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
